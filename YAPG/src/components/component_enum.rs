use std::sync::Arc;

use godot::prelude::*;
use enum_map::Enum;


use super::ComponentImpl;
use super::mesh_generator::mesh_component::MeshGeneratorComponent;
use super::geometry_generator::geometry_component::GeometryGeneratorComponent;

#[derive(GodotConvert, Var, Export, Default, Debug, Enum, Clone, Copy, PartialEq, PartialOrd)]
#[godot(via = GString)]
pub enum ComponentEnum {
    #[default]
    None,
    Geometry,
    Mesh,
}

impl ComponentEnum{
    pub fn build(&self) -> Option<Arc<dyn ComponentImpl>>{
        match self {
            Self::None => None,
            ComponentEnum::Geometry => Some(Arc::new(GeometryGeneratorComponent::default())),
            ComponentEnum::Mesh => Some(Arc::new(MeshGeneratorComponent::default()))
        }
    }
}

impl std::fmt::Display for ComponentEnum{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_string()))
    }
}