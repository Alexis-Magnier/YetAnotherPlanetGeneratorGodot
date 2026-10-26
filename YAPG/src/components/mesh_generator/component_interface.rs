use std::sync::Arc;

use godot::classes::Material;
use godot::prelude::*;
use crate::components::mesh_generator::mesh_component::MeshGeneratorComponent;
use crate::server::yapg_server::YAPGServer;
use crate::types::*;

#[derive(GodotClass)]
#[class(base=Object, no_init)]
struct MeshGeneratorComponentInterface{
    server: Gd<YAPGServer>,
    component: ComponentID,
    base: Base<Object>
}

#[godot_api]
impl MeshGeneratorComponentInterface {
    #[func]
    fn create_from_component_id(component: ComponentID, server: Gd<YAPGServer>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { server, base, component })
    }

    fn get_component(&self) -> Option<Arc<MeshGeneratorComponent>> {
        let server_bind = self.server.bind();
        let server_impl = &server_bind.implementation;

        let component_trait = server_impl.components.get(self.component)?;

        component_trait
            .implementation
            .clone()
            .downcast_arc::<MeshGeneratorComponent>()
            .ok()
    }

    #[func]
    fn set_material(&mut self, material: Option<Gd<Material>>){
        let Some(component) = self.get_component() else { return; };
        component.update_material(material);
    }

    #[func]
    fn get_material(&self) -> Option<Gd<Material>>{
        let Some(component) = self.get_component() else { return Default::default(); };
        return component.parameters.read().unwrap().material.clone();
    }
}