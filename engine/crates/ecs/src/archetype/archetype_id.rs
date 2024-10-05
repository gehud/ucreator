#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchetypeId(usize);

impl ArchetypeId {
    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }
}
