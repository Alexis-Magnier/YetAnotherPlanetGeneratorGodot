use std::sync::Arc;

use godot::prelude::*;

use crate::{components::geometry_generator::{geometry_component::GeometryGeneratorComponent, stages::plate_generator::PlateBaseMaterial}, server::yapg_server::YAPGServer, types::ComponentID};

#[macro_export]
macro_rules! impl_component_parameters {
    ($class:ident, $(($struct_field:ident, $category:ident, $field:ident, $ty:ty, $def:literal)),* $(,)?) => {
        paste::paste! {
            #[::godot::register::godot_api(secondary)]
            impl $class {
                $(
                    #[func]
                    fn [<set_ $category _ $field>](&mut self, val: [<$ty>]) {
                        let Some(component) = self.get_component() else { return; };
                        if let Ok(mut parameters) = component.parameters.write() {
                            parameters.$struct_field.$field = val;
                        };
                    }

                    #[func]
                    fn [<get_ $category _ $field>](&self) -> $ty {
                        let Some(component) = self.get_component() else { return Default::default(); };
                        return component.parameters.read().unwrap().$struct_field.$field;
                    }

                    #[func]
                    fn [<get_ $category _ $field _default>]() -> $ty {
                        $def
                    }
                )*
            }
        }
    };
}

#[derive(GodotClass)]
#[class(base=Object, no_init)]
struct GeometryGeneratorComponentInterface{
    server: Gd<YAPGServer>,
    component: ComponentID,
    base: Base<Object>
}

#[godot_api]
impl GeometryGeneratorComponentInterface {
    #[func]
    fn create_from_component_id(component: ComponentID, server: Gd<YAPGServer>) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { server, base, component })
    }

    pub fn get_component(&self) -> Option<Arc<GeometryGeneratorComponent>> {
        let server_bind = self.server.bind();
        let server_impl = &server_bind.implementation;

        let component_trait = server_impl.components.get(self.component)?;

        component_trait
            .implementation
            .clone()
            .downcast_arc::<GeometryGeneratorComponent>()
            .ok()
    }

    #[func]
    fn set_plates_materials(&mut self, plates: Array<Option<Gd<PlateMaterial>>>){

        let Some(component) = self.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials = 
            plates
                .iter_shared()
                .map(|maybe_material| {
                    let Some(material) = maybe_material else { return PlateBaseMaterial::default() };
                    let bind = material.bind();

                    PlateBaseMaterial {
                        probabilistic_weight: bind.probabilistic_weight,
                        density_mean: bind.density_mean,
                        density_dev: bind.density_dev,
                        height_mean: bind.height_mean,
                        height_dev: bind.height_dev,
                        thermal_cooldown: bind.thermal_cooldown,
                        thermal_diffusion_rate: bind.thermal_diffusion_rate,
                        strength_mean: bind.strength_mean,
                        strength_dev: bind.strength_dev,
                    }
                })
                .collect()
        };

        plates
            .iter_shared()
            .enumerate()
            .for_each(|(idx, maybe_material)| {
                
                let Some(mut gd_material) = maybe_material else { return; };
                let mut material = gd_material.bind_mut();

                material.index = idx;
                material.interface.replace(self.to_gd());
            });
    }
}

#[derive(GodotClass)]
#[class(base=Resource, init, rename=YAPG_PlateMaterial, tool)]
struct PlateMaterial{
    #[export]
    #[var(set=set_probabilistic_weight)]
    pub probabilistic_weight: f32,
    #[export] #[var(set=set_density_mean)] pub density_mean: f32,
    #[export] #[var(set=set_density_dev)] pub density_dev: f32,
    #[export] #[var(set=set_height_mean)] pub height_mean: f32,
    #[export] #[var(set=set_height_dev)] pub height_dev: f32,
    #[export] #[var(set=set_thermal_cooldown)] pub thermal_cooldown: f32,
    #[export] #[var(set=set_thermal_diffusion_rate)] pub thermal_diffusion_rate: f32,
    #[export] #[var(set=set_strength_mean)] pub strength_mean: f32,
    #[export] #[var(set=set_strength_dev)] pub strength_dev: f32,

    pub index: usize,
    pub interface: Option<Gd<GeometryGeneratorComponentInterface>>
}

#[godot_api]
impl PlateMaterial {
    #[func]
    fn set_probabilistic_weight(&mut self, val: f32) {
        self.probabilistic_weight = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].probabilistic_weight = val;
            godot_print!("val : {val}. internal : {}", self.probabilistic_weight);
        };
    }

    #[func]
    fn set_density_mean(&mut self, val: f32) {
        self.density_mean = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].density_mean = val;
        };
    }

    #[func]
    fn set_density_dev(&mut self, val: f32) {
        self.density_dev = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].density_dev = val;
        };
    }

    #[func]
    fn set_height_mean(&mut self, val: f32) {
        self.height_mean = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].height_mean = val;
        };
    }

    #[func]
    fn set_height_dev(&mut self, val: f32) {
        self.height_dev = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].height_dev = val;
        };
    }

    #[func]
    fn set_thermal_cooldown(&mut self, val: f32) {
        self.thermal_cooldown = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].thermal_cooldown = val;
        };
    }

    #[func]
    fn set_thermal_diffusion_rate(&mut self, val: f32) {
        self.thermal_diffusion_rate = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].thermal_diffusion_rate = val;
        };
    }

    #[func]
    fn set_strength_mean(&mut self, val: f32) {
        self.strength_mean = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].strength_mean = val;
            godot_print!("{val}");
        };
    }

    #[func]
    fn set_strength_dev(&mut self, val: f32) {
        self.strength_dev = val;

        let Some(interface) = &self.interface else { return; };
        let interface_bind = interface.bind();
        let Some(component) = interface_bind.get_component() else { return; };

        if let Ok(mut parameters) = component.parameters.write() {
            parameters.plate_generator.materials[self.index].strength_dev = val;
            godot_print!("{}", parameters.plate_generator.materials[self.index].probabilistic_weight);
        };
    }
}

impl_component_parameters!(
    GeometryGeneratorComponentInterface,
    (cell_generator, cells, cell_count, u32, 10000),
    (cell_generator, cells, jitter, f32, 0.5),
    (cell_generator, cells, seed, u32, 0),

    (plate_generator, plates, seed, u32, 0),
    (plate_generator, plates, plate_count, u32, 150),
    (plate_generator, plates, priority_mean, f32, 0.5),
    (plate_generator, plates, priority_dev, f32, 0.5),
    (plate_generator, plates, directionnal_preference_dev, f32, 0.5),
    (plate_generator, plates, directionnal_preference_mean, f32, 0.5),
    (plate_generator, plates, velocity_mean, f32, 15.0),
    (plate_generator, plates, velocity_dev, f32, 1.5),

    (collision_detection, collision, dt, f32, 0.1),
    (collision_detection, collision, boundary_threshold, f32, 5.0),
    (collision_detection, collision, weight_stress_factor, f32, 1.0),
    (collision_detection, collision, self_subduction_height, f32, -0.1),
    (collision_detection, collision, other_subduction_height, f32, 0.5),

    (base_properties, properties, density_noise_seed, u32, 0),
    (base_properties, properties, height_noise_seed, u32, 0),
    (base_properties, properties, octaves, u32, 3),
    (base_properties, properties, frequency, f32, 8.0),
    (base_properties, properties, lacunarity, f32, 2.0),
    (base_properties, properties, persistence, f32, 0.5),

    (temperatures, temperature, divergent_temp, f32, 1.0),
    (temperatures, temperature, convergent_temp, f32, 0.4),
    (temperatures, temperature, transform_temp, f32, 0.1),
    (temperatures, temperature, cooldown_factor, f32, 0.1),
    (temperatures, temperature, diffuse_steps, u32, 3),
    (temperatures, temperature, diffuse_weight, f32, 0.1),
    (temperatures, temperature, diffuse_coef, f32, 0.25),
    (temperatures, temperature, noise_seed, u32, 6222),
    (temperatures, temperature, noise_frequency, f32, 2.0),
    (temperatures, temperature, noise_amplitude, f32, 0.1),
    (temperatures, temperature, noise_lacunarity, f32, 2.5),
    (temperatures, temperature, noise_persistence, f32, 0.5),

    (strength, strength, noise_seed, u32, 0),
    (strength, strength, noise_octaves, u32, 6),
    (strength, strength, noise_frequency, f32, 1.0),
    (strength, strength, noise_lacunarity, f32, 5.0),
    (strength, strength, noise_persistence, f32, 0.5),
    (strength, strength, noise_amplitude, f32, 1.0),
    (strength, strength, noise_mean, f32, 0.0),
    (strength, strength, noise_weight, f32, 1.0),
    (strength, strength, thermal_weight, f32, 0.1),

    (stress, stress, transmission_threshold, f32, 0.1),
    (stress, stress, base_strength, f32, 0.1),
    (stress, stress, strength_mult, f32, 1.0),
    (stress, stress, base_stress_mult, f32, 1.0),
    (stress, stress, reference_distance, f32, 1.0),
    (stress, stress, diffuse_weight, f32, 0.2),
    (stress, stress, diffuse_steps, u32, 5),
    (stress, stress, noise_seed, u32, 0),
    (stress, stress, noise_octaves, u32, 6),
    (stress, stress, noise_frequency, f32, 1.0),
    (stress, stress, noise_amplitude, f32, 0.5),
    (stress, stress, noise_lacunarity, f32, 1.0),
    (stress, stress, noise_persistence, f32, 5.0),
    (stress, stress, noise_strength_weight, f32, 2.0),
    (stress, stress, noise_max, f32, 0.1),

    (isostasy, isostasy, mantle_density, f32, 3.3),
    (isostasy, isostasy, phase_iterations, u32, 48),
    (isostasy, isostasy, fold_wavelength, f32, 1.0),
    (isostasy, isostasy, fold_gain, f32, 1.2),
    (isostasy, isostasy, fault_spacing, f32, 45.0),
    (isostasy, isostasy, fault_gain, f32, 1.0),
    (isostasy, isostasy, yeild_stress, f32, 0.25),
    (isostasy, isostasy, reference_thickness, f32, 0.35),
    (isostasy, isostasy, smoothing_iterations, u32, 2),
    (isostasy, isostasy, seed, u32, 0x9E37_79B9),
);