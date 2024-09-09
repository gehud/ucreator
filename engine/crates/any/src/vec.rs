use std::{mem::{needs_drop, MaybeUninit}, ptr::copy_nonoverlapping};

use crate::TypeInfo;

pub struct AnyVec {
    info: TypeInfo,
    len: usize,
    data: Vec<MaybeUninit<u8>>,
    _drop: Option<unsafe fn(*mut MaybeUninit<u8>)>
}

impl AnyVec {
    pub fn new<T: 'static>() -> Self {
        Self {
            info: TypeInfo::of::<T>(),
            len: 0usize,
            data: Vec::new(),
            _drop: needs_drop::<T>().then_some(Self::drop_in_place_as::<T>)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_
    /// or if the element type does not match the element type when the vestor was initialized.
    pub fn push<T: 'static>(&mut self, value: T) {
        self.type_mismatch_check::<T>();

        let old_len = self.data.len();
        self.data.resize(old_len + self.info.size(), MaybeUninit::uninit());

        unsafe {
            self.data.as_mut_ptr()
                .add(old_len)
                .cast::<T>()
                .write(value);
        }

        self.len += 1;
    }

    /// Returns a reference to an element or subslice depending on the type of
    /// index.
    ///
    /// - If given a position, returns a reference to the element at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    ///
    /// # Panics
    ///
    /// Panics if the element type does not match the element type when the vestor was initialized.
    pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
        self.type_mismatch_check::<T>();

        if !(0..self.len()).contains(&index) {
            return None
        } else {
            let start = index * self.info.size();

            unsafe {
                self.data
                    .as_ptr()
                    .add(start)
                    .cast::<T>()
                    .as_ref()
            }
        }
    }

    /// Returns a mutable reference to an element or subslice depending on the
    /// type of index (see [`get`]) or `None` if the index is out of bounds.
    ///
    /// # Panics
    ///
    /// Panics if the element type does not match the element type when the vestor was initialized.
    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        self.type_mismatch_check::<T>();

        if !(0..self.len()).contains(&index) {
            return None
        } else {
            let start = index * self.info.size();

            unsafe {
                self.data
                    .as_mut_ptr()
                    .add(start)
                    .cast::<T>()
                    .as_mut()
            }
        }
    }

    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`
    /// or if the element type does not match the element type when the vestor was initialized.
    pub fn insert<T: 'static>(&mut self, index: usize, element: T)  {
        self.type_mismatch_check::<T>();

        assert!(index <= self.len());

        let old_len = self.data.len();

        let shift_start = index * self.info.size();
        let shift_end = old_len;
        let shift_len = shift_end - shift_start;

        self.data.resize(old_len + self.info.size(), MaybeUninit::uninit());

        let insert_start = shift_start;
        let insert_end = insert_start + self.info.size();

        unsafe {
            let ptr = self.data.as_mut_ptr();
            copy_nonoverlapping(ptr.add(shift_start), ptr.add(insert_end), shift_len);

            ptr.add(insert_start).cast::<T>().write(element);
        }

        self.len += 1;
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds
    /// or if the element type does not match the element type when the vestor was initialized.
    pub fn remove<T: 'static>(&mut self, index: usize) -> T {
        self.type_mismatch_check::<T>();

        let start = index * self.info.size();
        let end = start + self.info.size();

        if !(0..self.len()).contains(&index) {
            panic!("'index' is out of bounds");
        }

        // match self.drop {
        //     Some(func) => {
        //         unsafe {
        //             let ptr = self.data.as_mut_ptr().add(start);
        //             func(ptr);
        //         }
        //     },
        //     None => (),
        // }

        self.len -= 1;

        unsafe {
            self.data
                .drain(start..end)
                .as_slice()
                .as_ptr()
                .cast::<T>()
                .read()
        }
    }

    #[inline]
    fn type_mismatch_check<T: 'static>(&self) {
        assert_eq!(TypeInfo::of::<T>().id(), self.info.id(), "time mismatch");
    }

    #[inline]
    unsafe fn drop_in_place_as<T>(ptr: *mut MaybeUninit<u8>) {
        ptr.cast::<T>().drop_in_place();
    }
}

#[cfg(test)]
mod tests {
    use super::AnyVec;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
    struct A {
        a: i32,
        b: u32,
        c: i64,
        d: u64,
        e: Box<u8>
    }

    #[test]
    fn test() {
        let mut vec = AnyVec::new::<A>();

        let a = A {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: Box::new(5u8)
        };

        vec.push(a.clone());

        assert_eq!(&a, vec.get(0usize).unwrap());
        vec.insert(0usize, a.clone());

        assert_eq!(a, vec.remove(0usize));
        assert_eq!(&a, vec.get(0usize).unwrap());
    }
}
