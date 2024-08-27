use crate::World;

pub trait Command: 'static {
    fn apply(self, world: &mut World);
}

impl<T: Fn(&mut World) -> () + 'static> Command for T {
    fn apply(self, world: &mut World) {
        self(world);
    }
}
