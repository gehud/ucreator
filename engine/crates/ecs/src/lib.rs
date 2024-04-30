#![allow(incomplete_features)]
#![feature(specialization)]

mod error;
pub use error::Error;
pub use error::Result;

mod entity;
pub use entity::Entity;

mod storage;
pub use storage::Storage;
pub use storage::Table;

mod data;
pub use data::Data;

mod filter;
pub use filter::With;
pub use filter::Without;
pub use filter::Filter;

mod query;
pub use query::Query;

mod group;
pub use group::Group;
pub use group::SystemGroup;
pub use group::Simulation;
pub use group::Physics;

mod system;
pub use system::System;
pub use system::SystemCreation;
pub use system::SystemDestruction;
pub use system::Context;

mod world;
pub use world::World;
