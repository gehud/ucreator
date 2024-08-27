pub enum StorageType {
    Archetype,
    SparseSet
}

pub trait Component: 'static {
    const STORAGE_TYPE: StorageType;
}
