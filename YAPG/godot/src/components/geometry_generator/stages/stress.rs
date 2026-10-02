use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Stress_Impl)]
struct Stress{
    pub inner: yapg_stages::geometry_generator::stages::stress::Stress,
    base: Base<RefCounted>
}

delegate_inner! {
    Stress {
        transmission_threshold: f32,
        base_strength: f32,
        strength_mult: f32,
        base_stress_mult: f32,
        reference_distance: f32,
        diffuse_weight: f32,
        diffuse_steps: u32,
        noise_seed: u32,
        noise_octaves: u32,
        noise_frequency: f32,
        noise_amplitude: f32,
        noise_lacunarity: f32,
        noise_persistence: f32,
        noise_strength_weight: f32,
        noise_max: f32,
    }
}