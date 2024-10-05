#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableRow(usize);

impl TableRow {
    pub(crate) const INVALID: TableRow = TableRow::new(usize::MAX);

    pub(crate) const fn new(value: usize) -> Self {
        Self(value)
    }

    #[inline]
    pub(crate) const fn as_usize(self) -> usize {
        self.0
    }
}
