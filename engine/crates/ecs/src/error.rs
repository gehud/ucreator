use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    EntityAlreadyPresented,
    EntityNotPresented,
    WorldOutOfBounds,
    TypeNotPresented,
    TypeAlreadyPresented,
    ComponentAlreadyPresented,
    SystemAlreadyRegistered(&'static str),
    SystemNotRegistered(&'static str),
    SystemAlreadyPresented(&'static str),
    GroupNotRegistered,
    ArchetypeAllreadyPresented,
    ArchetypeNotPresented,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EntityAlreadyPresented =>
                write!(f, "Entity is already presented"),
            Error::EntityNotPresented =>
                write!(f, "Entity is not presented"),
            Error::WorldOutOfBounds =>
                write!(f, "World is out of bounds"),
            Error::TypeNotPresented =>
                write!(f, "Type is not presented"),
            Error::TypeAlreadyPresented =>
                write!(f, "Type is already presented"),
            Error::ComponentAlreadyPresented =>
                write!(f, "Component is already presented"),
            Error::SystemAlreadyRegistered(name) =>
                write!(f, "System '{}' is already registered", name),
            Error::SystemNotRegistered(name) =>
                write!(f, "System '{}' is not registered", name),
            Error::SystemAlreadyPresented(name) =>
                write!(f, "System '{}' is already presented", name),
            Error::GroupNotRegistered =>
                write!(f, "Group is not registered"),
            Error::ArchetypeAllreadyPresented =>
                write!(f, "Archetype allready presented"),
            Error::ArchetypeNotPresented =>
                write!(f, "Archetype not presented"),
        }
    }
}
