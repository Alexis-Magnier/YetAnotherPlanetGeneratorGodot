use slotmap::{DefaultKey, Key, KeyData, SlotMap};

use crate::{
    planet::Planet,
    types::{
        INVALID_PLANET_ID,
        PlanetID
    }
};


pub struct PlanetStorage{
    storage: SlotMap<DefaultKey, Planet>
}

impl PlanetStorage{
    pub fn default() -> Self{
        Self {
            storage: SlotMap::default()
        }
    }

    fn key_to_id(key: DefaultKey) -> PlanetID{
        if key.is_null() {
            INVALID_PLANET_ID
        } else {
            key.data().as_ffi() as PlanetID
        }
    }

    fn id_to_key(id: PlanetID) -> DefaultKey{
        if id == INVALID_PLANET_ID {
            DefaultKey::null()
        } else {
            DefaultKey::from(KeyData::from_ffi(id as u64))
        }
    }

    pub fn create(&mut self) -> PlanetID{
        Self::key_to_id(
            self.storage.insert(Planet::default())
        )
    }

    pub fn free(&mut self, id: PlanetID) -> Option<Planet>{
        self.storage.remove(
            Self::id_to_key(id)
        )
    }

    pub fn get(&self, id: PlanetID) -> Option<&Planet>{
        self.storage.get(
            Self::id_to_key(id)
        )
    }

    pub fn get_mut(&mut self, id: PlanetID) -> Option<&mut Planet>{
        self.storage.get_mut(
            Self::id_to_key(id)
        )
    }

    pub fn exists(&self, id: PlanetID) -> bool{
        self.storage.contains_key(Self::id_to_key(id))
    }
}