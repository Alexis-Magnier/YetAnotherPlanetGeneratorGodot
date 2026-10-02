use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Detect_Collisions_Impl)]
struct DetectCollisions{
    pub inner: yapg_stages::geometry_generator::stages::detect_collisions::CollisionDetection,

    base: Base<RefCounted>
}

delegate_inner! {
    DetectCollisions {
        dt: f32,
        boundary_threshold: f32,
        pinning_weight_stress_factor: f32,
        pinning_self_subduction_height: f32,
        pinning_other_subduction_height: f32,
    }
}