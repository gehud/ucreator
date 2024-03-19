use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    EntityAlreadyPresented,
    EntityNotPresented,
    WorldOutOfBounds,
    TypeNotPresented,
    TypeAlreadyPresented,
    ComponentAlreadyPresented
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EntityAlreadyPresented =>
                write!(f, "Entity already presented"),
            Error::EntityNotPresented =>
                write!(f, "Entity not presented"),
            Error::WorldOutOfBounds =>
                write!(f, "World out of bounds"),
            Error::TypeNotPresented =>
                write!(f, "Type not presented"),
            Error::TypeAlreadyPresented =>
                write!(f, "Type already presented"),
            Error::ComponentAlreadyPresented =>
                write!(f, "Component already presented")
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
