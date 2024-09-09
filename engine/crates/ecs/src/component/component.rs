use super::StoragePolicy;

pub trait Component: 'static {
    const STORAGE_POLICY: StoragePolicy;
}
