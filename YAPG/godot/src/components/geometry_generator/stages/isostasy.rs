use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Isostasy_Impl)]
struct Isostasy{
    pub inner: yapg_stages::geometry_generator::stages::isostasy::Isostasy,
    base: Base<RefCounted>
}

delegate_inner! {
    Isostasy {
        mantle_density: f32,
        phase_iterations: u32,
        fold_wavelength_km: f32,
        fold_gain: f32,
        fault_spacing_km: f32,
        fault_gain: f32,
        yield_stress: f32,
        reference_thickness_km: f32,
        smoothing_iterations: u32,
        seed: u32,
    }
}