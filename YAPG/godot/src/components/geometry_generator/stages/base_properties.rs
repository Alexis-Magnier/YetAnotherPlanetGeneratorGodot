use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Base_Properties_Impl)]
struct BaseProperties{
    pub inner: yapg_stages::geometry_generator::stages::base_properties::AssignBaseProperties,
    base: Base<RefCounted>
}

delegate_inner! {
    BaseProperties {
        density_noise_seed: u32,
        height_noise_seed: u32,
        octaves: u32,
        frequency: f32,
        lacunarity: f32,
        persistence: f32,
    }
}