use godot::prelude::*;
use enum_map::EnumMap;

use super::components::{ComponentEnum, component_impl::ComponentError};
use super::storage::{
    component_storage::ComponentStorage,
    node_property::NodeProperty,
    planet_properties_registry::PlanetPropertyRegistry
};

use crate::storage::planet_properties_registry::{PropertyGuard, PropertyGuardMut};
use crate::types::ComponentID;


pub struct Planet{
    pub components: EnumMap<ComponentEnum, Option<ComponentID>>,
    pub storage: PlanetPropertyRegistry,
}

impl Planet{
    pub fn default() -> Self{
        Planet {
            components: EnumMap::default(),
            storage: PlanetPropertyRegistry::default(),
        }
    }

    #[inline]
    pub fn get_component(&self, component: ComponentEnum) -> Option<ComponentID>{
        self.components[component]
    }

    #[inline]
    pub fn set_component(&mut self, component: ComponentEnum, id: ComponentID){
        if self.components[component].replace(id) != None {
            godot_warn!("[YAPGServer] Replacing component {}", component.to_string())
        }
    }

    #[inline]
    pub fn has_component(&self, component: ComponentEnum) -> bool{
        self.components[component].is_some()
    }

    #[inline]
    pub fn remove_component(&mut self, component: ComponentEnum){
        self.components[component] = None;
    }

    #[inline]
    pub fn insert_property<T: 'static + Send + Sync>(&mut self, value: T) -> Option<T>{
        return self.storage.insert::<T>(value)
    }

    #[inline]
    pub fn remove_property<T: 'static + Send + Sync>(&mut self) -> Option<T>{
        return self.storage.remove::<T>()
    }

    #[inline]
    pub fn get_property<T: 'static + Send + Sync>(&mut self) -> Option<PropertyGuard<'_, T>>{
        return self.storage.get::<T>()
    }
    
    #[inline]
    pub fn get_property_mut<T: 'static + Send + Sync>(&mut self) -> Option<PropertyGuardMut<'_, T>>{
        return self.storage.get_mut::<T>()
    }

    #[inline]
    pub fn has_property<T: 'static + Send + Sync>(&mut self) -> bool{
        return self.storage.has::<T>()
    }
    
    pub fn generate(&mut self, components: &ComponentStorage, node: Gd<Node>) -> Result<(), ComponentError>{
        let mut storage = PlanetPropertyRegistry::default();

        storage.insert(NodeProperty::new(node));

        for (_, c) in &self.components{
            let Some(component_id) = c else { continue };
            let Some(component) = components.get(*component_id) else { continue };

            component.implementation.generate(&mut storage)?;
        }

        self.storage = storage;

        Ok(())
    }

    pub fn update(&mut self, components: &ComponentStorage, dt: f32) -> Result<(), ComponentError>{
        for (_, c) in &self.components{
            let Some(component_id) = c else { continue };
            let Some(component) = components.get(*component_id) else { continue };

            component.implementation.update(&mut self.storage, dt)?;
        }

        Ok(())
    }

}