use slotmap::{DefaultKey, Key, KeyData, SlotMap};
use crate::types::{ComponentID, INVALID_COMPONENT_ID};
use crate::components::Component;

pub struct ComponentStorage{
    storage: SlotMap<DefaultKey, Component>
}

impl ComponentStorage{
    pub fn default() -> Self{
        Self {
            storage: SlotMap::default()
        }
    }

    fn key_to_id(key: DefaultKey) -> ComponentID{
        if key.is_null() {
            INVALID_COMPONENT_ID
        } else {
            key.data().as_ffi() as ComponentID
        }
    }

    fn id_to_key(id: ComponentID) -> DefaultKey{
        if id == INVALID_COMPONENT_ID {
            DefaultKey::null()
        } else {
            DefaultKey::from(KeyData::from_ffi(id as u64))
        }
    }

    pub fn insert(&mut self, comp: Component) -> ComponentID{
        Self::key_to_id(
            self.storage.insert(comp)
        )
    }

    pub fn free(&mut self, id: ComponentID) -> Option<Component>{
        self.storage.remove(
            Self::id_to_key(id)
        )
    }

    pub fn get(&self, id: ComponentID) -> Option<&Component>{
        self.storage.get(
            Self::id_to_key(id)
        )
    }

    pub fn get_mut(&mut self, id: ComponentID) -> Option<&mut Component>{
        self.storage.get_mut(
            Self::id_to_key(id)
        )
    }

    pub fn exists(&self, id: ComponentID) -> bool{
        self.storage.contains_key(Self::id_to_key(id))
    }
}