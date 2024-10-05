use std::hash::Hash;

use uengine_any::TypeInfo;

use super::{Component, ComponentId, StoragePolicy};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ComponentInfo {
    id: ComponentId,
    type_info: TypeInfo,
    storage_policy: StoragePolicy
}

impl ComponentInfo {
    pub(crate) fn new<C: Component>(id: ComponentId) -> Self {
        Self {
            id,
            type_info: TypeInfo::of::<C>(),
            storage_policy: C::STORAGE_POLICY
        }
    }

    pub fn id(&self) -> ComponentId {
        self.id
    }

    pub fn type_info(&self) -> &TypeInfo {
        &self.type_info
    }

    pub fn storage_policy(&self) -> &StoragePolicy {
        &self.storage_policy
    }
}

impl Hash for ComponentInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_info.hash(state);
    }
}
