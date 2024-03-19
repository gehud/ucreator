use std::fmt;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    index: usize
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity: {{ {} }}", self.index)
    }
}

impl Entity {
    pub fn new(index: usize) -> Self {
        Self {
            index
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
