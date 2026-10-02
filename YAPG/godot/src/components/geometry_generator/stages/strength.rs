use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Strength_Impl)]
struct Strength{
    pub inner: yapg_stages::geometry_generator::stages::strength::Strength,
    base: Base<RefCounted>
}

delegate_inner! {
    Strength {
        noise_seed: u32,
        noise_octaves: u32,
        noise_frequency: f32,
        noise_lacunarity: f32,
        noise_persistence: f32,
        noise_amplitude: f32,
        noise_mean: f32,
        noise_weight: f32,
        thermal_weight: f32,
    }
}