use std::{mem::MaybeUninit, panic::{catch_unwind, resume_unwind, AssertUnwindSafe}, ptr::NonNull};

use crate::{Command, World};

struct CommandLauncher {
    apply_command_and_get_size:
        unsafe fn(bytes: *mut u8, world: Option<*mut World>, cursor: &mut usize),
}

pub struct Commands {
    bytes: NonNull<Vec<MaybeUninit<u8>>>,
    cursor: NonNull<usize>,
    panic_recovery: NonNull<Vec<MaybeUninit<u8>>>
}

impl Commands {
    pub fn new() -> Self {
        unsafe {
            Self {
                bytes: NonNull::new_unchecked(Box::into_raw(Box::default())),
                cursor: NonNull::new_unchecked(Box::into_raw(Box::new(0usize))),
                panic_recovery: NonNull::new_unchecked(Box::into_raw(Box::default()))
            }
        }
    }

    pub fn push<C: Command>(&mut self, command: C) {
        // `repr(C)` prevents the compiler from reordering the fields,
        // `repr(packed)` prevents the compiler from inserting padding bytes.
        #[repr(C, packed)]
        struct Packed<T: Command> {
            launcher: CommandLauncher,
            command: T,
        }

        let launcher = CommandLauncher {
            apply_command_and_get_size: |bytes, world, cursor| {
                *cursor += std::mem::size_of::<C>();

                let command: C = unsafe { bytes.cast::<C>().read_unaligned() };
                match world {
                    Some(world) => {
                        let world = unsafe { world.as_mut().unwrap() };
                        command.apply(world);
                    },
                    None => drop(command)
                }
            },
        };

        let bytes = unsafe { self.bytes.as_mut() };

        let old_len = bytes.len();

        bytes.reserve(std::mem::size_of::<Packed<C>>());

        let ptr = unsafe { bytes.as_mut_ptr().add(old_len) };

        unsafe {
            ptr.cast::<Packed<C>>()
                .write_unaligned(Packed { launcher, command });
        }

        unsafe {
            bytes.set_len(old_len + std::mem::size_of::<Packed<C>>());
        }
    }

    unsafe fn apply_or_drop(&mut self, world: Option<*mut World>) {
        let start = *self.cursor.as_ref();
        let stop = self.bytes.as_ref().len();
        let mut local_cursor = start;

        *self.cursor.as_mut() = stop;

        while local_cursor < stop {
            let meta = self.bytes
                .as_mut()
                .as_mut_ptr()
                .add(local_cursor)
                .cast::<CommandLauncher>()
                .read_unaligned();

            local_cursor += std::mem::size_of::<CommandLauncher>();

            let cmd = self.bytes.as_mut().as_mut_ptr().add(local_cursor).cast();

            let result = catch_unwind(AssertUnwindSafe(|| {
                (meta.apply_command_and_get_size)(cmd, world, &mut local_cursor);
            }));

            if let Err(payload) = result {
                let panic_recovery = self.panic_recovery.as_mut();
                let bytes = self.bytes.as_mut();
                let current_stop = bytes.len();
                panic_recovery.extend_from_slice(&bytes[local_cursor..current_stop]);
                bytes.set_len(start);
                *self.cursor.as_mut() = start;

                if start == 0 {
                    bytes.append(panic_recovery);
                }

                resume_unwind(payload);
            }
        }

        self.bytes.as_mut().set_len(start);
        *self.cursor.as_mut() = start;
    }

    pub fn apply(&mut self, world: &mut World) {
        unsafe {
            self.apply_or_drop(Some(world));
        }
    }
}
