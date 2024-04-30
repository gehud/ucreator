use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    path: PathBuf
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}
