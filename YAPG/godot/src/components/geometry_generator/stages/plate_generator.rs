use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=Resource, init, rename=YAPG_PlateGeneratorMaterial)]
struct PlateMaterial{

    #[export] probabilistic_weight: f32,
    #[export] density_mean: f32,
    #[export] density_dev: f32,
    #[export] height_mean: f32,
    #[export] height_dev: f32,
    #[export] thermal_cooldown: f32,
    #[export] thermal_diffusion_rate: f32,
    #[export] strength_mean: f32,
    #[export] strength_dev: f32,
    
    base: Base<Resource>
}

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Plate_Generator_Impl)]
struct PlateGenerator{
    pub inner: yapg_stages::geometry_generator::stages::plate_generator::PlateGenerator,

    base: Base<RefCounted>
}

delegate_inner! {
    PlateGenerator {
        plate_count: u32,

        priority_mean: f32,
        priority_dev: f32,

        directionnal_preference_mean: f32,
        directionnal_preference_dev: f32,

        velocity_mean: f32,
        velocity_dev: f32,

        seed: i64 => {
            get => |this| this.inner.seed as i64,
            set => |this, val| this.inner.seed = val.max(0) as u64
        },

        plate_materials: Array<Gd<PlateMaterial>> => {
            get => |this| Array::new(),
            set => |this, val| {}
        },
    }
}