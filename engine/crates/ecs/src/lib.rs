#![feature(concat_idents)]

mod error;
pub use error::Error;

mod entity;
pub use entity::Entity;

mod component;
pub use component::Component;
pub use component::StorageType;

mod archetype;
pub use archetype::Archetype;
pub use crate::__ecs_archetype__ as archetype;

mod storage;
pub use storage::Storage;
pub use storage::Table;

mod query;
pub use query::Query;
pub use query::QueryData;
pub use query::With;
pub use query::Without;
pub use query::QueryFilter;

mod group;
pub use group::Group;
pub use group::SystemGroup;
pub use group::Simulation;
pub use group::Physics;

mod system;
pub use system::System;
pub use system::In;

mod world;
pub use world::World;

mod command;
pub use command::Command;

mod commands;
pub use commands::Commands;

mod schedule;
pub use schedule::Schedule;

pub use uengine_ecs_macros::*;
