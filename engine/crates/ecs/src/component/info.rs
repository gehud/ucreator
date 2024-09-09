use uengine_any::TypeInfo;

use super::{Component, StoragePolicy};

pub struct ComponentInfo {
    type_info: TypeInfo,
    storage_policy: StoragePolicy
}

impl ComponentInfo {
    pub fn of<C: Component>() -> Self {
        Self {
            type_info: TypeInfo::of::<C>(),
            storage_policy: C::STORAGE_POLICY
        }
    }

    pub fn type_info(&self) -> &TypeInfo {
        &self.type_info
    }

    pub fn storage_policy(&self) -> &StoragePolicy {
        &self.storage_policy
    }
}
