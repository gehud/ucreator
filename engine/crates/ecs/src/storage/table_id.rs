#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableId(usize);

impl TableId {
    pub(crate) const INVALID: TableId = TableId(usize::MAX);

    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) const fn as_usize(self) -> usize {
        self.0
    }
}
