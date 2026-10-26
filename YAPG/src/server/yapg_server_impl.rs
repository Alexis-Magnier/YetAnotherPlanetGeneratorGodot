use crate::components::{Component, ComponentEnum};
use crate::types::*;
use crate::server::server_error::PlanetServerError;
use crate::storage::component_storage::ComponentStorage;
use crate::storage::planet_storage::PlanetStorage;
use godot::prelude::*;

pub struct YAPGServerImpl{
    pub planets: PlanetStorage,
    pub components: ComponentStorage,
}

impl YAPGServerImpl{
    pub fn default() -> Self{
        YAPGServerImpl {
            planets: PlanetStorage::default(),
            components: ComponentStorage::default(),
        }
    }

    pub fn create_planet(&mut self) -> Result<PlanetID, PlanetServerError>{
        Ok(self.planets.create())
    }

    pub fn free_planet(&mut self, id: PlanetID) -> Result<(), PlanetServerError>{

        let mut planet = self.planets
            .free(id)
            .ok_or(PlanetServerError::PlanetNotFound(id))?;

        for (_, maybe_id) in &planet.components {
            if let Some(id) = maybe_id {
                if let Some(component) = self.components.get_mut(*id) {
                    let _ = component.implementation.cleanup(&mut planet.storage);
                }
            }
        }

    Ok(())
    }


    pub fn planet_exists(&self, id: PlanetID) -> bool{
        self.planets.exists(id)
    }


    pub fn generate_planet(&mut self, planet_id: PlanetID, node: Gd<Node>) -> Result<(), PlanetServerError>{
        let planet = self.planets.get_mut(planet_id)
            .ok_or_else(|| PlanetServerError::PlanetNotFound(planet_id))?;

        planet.generate(&self.components, node)
            .map_err(|err| PlanetServerError::PlanetGenerationError(err))
    }


    pub fn tick_planet(&mut self, planet_id: PlanetID, dt: f32) -> Result<(), PlanetServerError>{
        let planet = self.planets.get_mut(planet_id)
            .ok_or_else(|| PlanetServerError::PlanetNotFound(planet_id))?;
        
        planet.update(&self.components, dt)
            .map_err(|err| PlanetServerError::PlanetUpdateError(err))
    }


    pub fn add_planet_component(&mut self, planet_id: PlanetID, component_type: ComponentEnum) -> Result<ComponentID, PlanetServerError>{
        let planet = self.planets.get_mut(planet_id)
            .ok_or_else(|| PlanetServerError::PlanetNotFound(planet_id))?;

        if let Some(comp_id) = planet.get_component(component_type) {
            return Ok(comp_id);
        }

        let implementation = component_type.build()
            .ok_or_else(|| PlanetServerError::InvalidComponentType(component_type))?;

        let new_component = Component{
            planet: planet_id,
            enum_type: component_type,
            implementation
        };

        let id = self.components.insert(new_component);
        planet.set_component(component_type, id);

        Ok(id)
    }


    pub fn remove_component(&mut self, id: ComponentID) -> Result<(), PlanetServerError>{

        let component = self.components.free(id)
            .ok_or_else(|| PlanetServerError::ComponentNotFound(id))?;

        let planet = self.planets.get_mut(component.planet)
            .ok_or_else(|| PlanetServerError::PlanetNotFound(component.planet))?;

        let _ = component.implementation.cleanup(&mut planet.storage);
        planet.remove_component(component.enum_type);

        Ok(())
    }


    pub fn component_exists(&self, id: ComponentID) -> bool{
        self.components.exists(id)
    }
}