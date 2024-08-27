use std::fmt::{Debug, Display, Formatter, Result};

use super::SystemId;

pub enum SystemError<I = (), O = ()> {
    SystemNotRegistered(SystemId<I, O>)
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SystemError::SystemNotRegistered(id) =>
                write!(f, "System {0:?} was not registered", id),
        }
    }
}

impl<I, O> Debug for SystemError<I, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::SystemNotRegistered(id) => {
                f.debug_tuple("SystemNotRegistered").field(id).finish()
            }
        }
    }
}
