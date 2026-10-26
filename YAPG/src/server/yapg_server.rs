use godot::prelude::*;
use super::yapg_server_impl::YAPGServerImpl;
use crate::{components::ComponentEnum, types::*};


#[derive(GodotClass)]
#[class(base=Object, tool, rename=_YAPGServer)]
pub struct YAPGServer {
    pub implementation: YAPGServerImpl,
    base: Base<Object>,
}

#[godot_api]
impl IObject for YAPGServer {
    fn init(base: Base<Object>) -> Self {
        Self {
            implementation: YAPGServerImpl::default(),
            base,
        }
    }
}

#[godot_api]
impl YAPGServer{

    #[constant]
    const INVALID_PLANET_ID: PlanetID = INVALID_PLANET_ID;
    
    #[constant]
    const INVALID_COMPONENT_ID: ComponentID = INVALID_COMPONENT_ID;

    #[func]
    fn create_planet(&mut self) -> PlanetID{
        match self.implementation.create_planet(){
            Ok(id) => id,
            Err(err) => {
                godot_error!("[YAPGServer] {err}");
                INVALID_PLANET_ID
            }
        }
    }

    #[func]
    fn free_planet(&mut self, planet: PlanetID){
        if let Err(err) = self.implementation.free_planet(planet){
            godot_warn!("[YAPGServer] {err}");
        }
    }

    #[func]
    fn planet_exists(&self, planet: PlanetID) -> bool{
        self.implementation.planet_exists(planet)
    }

    #[func]
    fn generate_planet(&mut self, planet: PlanetID, node: Gd<Node>){
        if let Err(err) = self.implementation.generate_planet(planet, node){
            godot_error!("[YAPGServer] {err}");
        }
    }

    #[func]
    fn tick_planet(&mut self, planet: PlanetID, dt: f32){
        if let Err(err) = self.implementation.tick_planet(planet, dt){
            godot_error!("[YAPGServer] {err}");
        }
    }

    #[func]
    fn add_planet_component(&mut self, planet: PlanetID, component: ComponentEnum) -> ComponentID{
        godot_print!("add component");
        match self.implementation.add_planet_component(planet, component){
            Ok(id) => id,
            Err(err) => {
                godot_error!("[YAPGServer] {err}");
                INVALID_COMPONENT_ID
            }
        }
    }

    #[func]
    fn remove_component(&mut self, component: ComponentID){
        godot_print!("remove component");
        if let Err(err) = self.implementation.remove_component(component){
            godot_warn!("[YAPGServer] {err}")
        }
    }

    #[func]
    fn component_exists(&self, component: ComponentID) -> bool{
        self.implementation.component_exists(component)
    }
}