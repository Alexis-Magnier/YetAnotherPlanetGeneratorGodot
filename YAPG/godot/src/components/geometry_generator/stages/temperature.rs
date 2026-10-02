use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Temperature_Impl)]
struct Temperature{
    pub inner: yapg_stages::geometry_generator::stages::temperature::Temperatures,

    base: Base<RefCounted>
}

delegate_inner! {
    Temperature {
        divergent_temp: f32,
        convergent_temp: f32,
        transform_temp: f32,
        cooldown_factor: f32,
        diffuse_step: u32,
        diffuse_weight: f32,
        diffusion_coef: f32,
        noise_seed: u32,
        noise_octaves: u32,
        noise_frequency: f32,
        noise_amplitude: f32,
        noise_lacunarity: f32,
        noise_persistence: f32,
    }
}