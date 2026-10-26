@tool
class_name YAPG_GeometryGenerator extends YAPG_PlanetComponent

var interface: GeometryGeneratorComponentInterface = null




@export_group("cells", "cells_")
@export var cells_seed: int = 10000:
	set(val):
		cells_seed = val
		if interface != null:
			interface.set_cells_seed(val)
	get():
		return cells_seed
@export var cells_jitter: float = 0.5:
	set(val):
		cells_jitter = val
		if interface != null:
			interface.set_cells_jitter(val)
	get():
		return cells_jitter
@export var cells_cell_count: int = 0:
	set(val):
		cells_cell_count = val
		if interface != null:
			interface.set_cells_cell_count(val)
	get():
		return cells_cell_count

@export_group("plates", "plates_")
@export var plates_seed: int = 0:
	set(val):
		plates_seed = val
		if interface != null:
			interface.set_plates_seed(val)
	get():
		return plates_seed
@export var plates_plate_count: int = 150:
	set(val):
		plates_plate_count = val
		if interface != null:
			interface.set_plates_plate_count(val)
	get():
		return plates_plate_count
@export var plates_priority_dev: float = 0.5:
	set(val):
		plates_priority_dev = val
		if interface != null:
			interface.set_plates_priority_dev(val)
	get():
		return plates_priority_dev
@export var plates_priority_mean: float = 0.5:
	set(val):
		plates_priority_mean = val
		if interface != null:
			interface.set_plates_priority_mean(val)
	get():
		return plates_priority_mean
@export var plates_directionnal_preference_dev: float = 0.5:
	set(val):
		plates_directionnal_preference_dev = val
		if interface != null:
			interface.set_plates_directionnal_preference_dev(val)
	get():
		return plates_directionnal_preference_dev
@export var plates_directionnal_preference_mean: float = 0.5:
	set(val):
		plates_directionnal_preference_mean = val
		if interface != null:
			interface.set_plates_directionnal_preference_mean(val)
	get():
		return plates_directionnal_preference_mean
@export var plates_velocity_mean: float = 15.0:
	set(val):
		plates_velocity_mean = val
		if interface != null:
			interface.set_plates_velocity_mean(val)
	get():
		return plates_velocity_mean
@export var plates_velocity_dev: float = 1.5:
	set(val):
		plates_velocity_dev = val
		if interface != null:
			interface.set_plates_velocity_dev(val)
	get():
		return plates_velocity_dev
@export var plates_materials: Array[YAPG_PlateMaterial] = []:
	set(val):
		plates_materials = val
		if interface != null:
			interface.set_plates_materials(val)
	get():
		return plates_materials

@export_group("collision", "collision_")
@export var collision_dt: float = 0.1:
	set(val):
		collision_dt = val
		if interface != null:
			interface.set_collision_dt(val)
	get():
		return collision_dt
@export var collision_boundary_threshold: float = 5.0:
	set(val):
		collision_boundary_threshold = val
		if interface != null:
			interface.set_collision_boundary_threshold(val)
	get():
		return collision_boundary_threshold
@export var collision_weight_stress_factor: float = 1.0:
	set(val):
		collision_weight_stress_factor = val
		if interface != null:
			interface.set_collision_weight_stress_factor(val)
	get():
		return collision_weight_stress_factor
@export var collision_self_subduction_height: float = -0.1:
	set(val):
		collision_self_subduction_height = val
		if interface != null:
			interface.set_collision_self_subduction_height(val)
	get():
		return collision_self_subduction_height
@export var collision_other_subduction_height: float = 0.5:
	set(val):
		collision_other_subduction_height = val
		if interface != null:
			interface.set_collision_other_subduction_height(val)
	get():
		return collision_other_subduction_height

@export_group("properties", "properties_")
@export var properties_density_noise_seed: int = 0:
	set(val):
		properties_density_noise_seed = val
		if interface != null:
			interface.set_properties_density_noise_seed(val)
	get():
		return properties_density_noise_seed
@export var properties_height_noise_seed: int = 0:
	set(val):
		properties_height_noise_seed = val
		if interface != null:
			interface.set_properties_height_noise_seed(val)
	get():
		return properties_height_noise_seed
@export var properties_octaves: int = 3:
	set(val):
		properties_octaves = val
		if interface != null:
			interface.set_properties_octaves(val)
	get():
		return properties_octaves
@export var properties_frequency: float = 8.0:
	set(val):
		properties_frequency = val
		if interface != null:
			interface.set_properties_frequency(val)
	get():
		return properties_frequency
@export var properties_lacunarity: float = 2.0:
	set(val):
		properties_lacunarity = val
		if interface != null:
			interface.set_properties_lacunarity(val)
	get():
		return properties_lacunarity
@export var properties_persistence: float = 0.5:
	set(val):
		properties_persistence = val
		if interface != null:
			interface.set_properties_persistence(val)
	get():
		return properties_persistence

@export_group("temperature", "temperature_")
@export var temperature_divergent_temp: float = 1.0:
	set(val):
		temperature_divergent_temp = val
		if interface != null:
			interface.set_temperature_divergent_temp(val)
	get():
		return temperature_divergent_temp
@export var temperature_convergent_temp: float = 0.4:
	set(val):
		temperature_convergent_temp = val
		if interface != null:
			interface.set_temperature_convergent_temp(val)
	get():
		return temperature_convergent_temp
@export var temperature_transform_temp: float = 0.1:
	set(val):
		temperature_transform_temp = val
		if interface != null:
			interface.set_temperature_transform_temp(val)
	get():
		return temperature_transform_temp
@export var temperature_cooldown_factor: float = 0.1:
	set(val):
		temperature_cooldown_factor = val
		if interface != null:
			interface.set_temperature_cooldown_factor(val)
	get():
		return temperature_cooldown_factor
@export var temperature_diffuse_steps: int = 3:
	set(val):
		temperature_diffuse_steps = val
		if interface != null:
			interface.set_temperature_diffuse_steps(val)
	get():
		return temperature_diffuse_steps
@export var temperature_diffuse_weight: float = 0.1:
	set(val):
		temperature_diffuse_weight = val
		if interface != null:
			interface.set_temperature_diffuse_weight(val)
	get():
		return temperature_diffuse_weight
@export var temperature_diffuse_coef: float = 0.25:
	set(val):
		temperature_diffuse_coef = val
		if interface != null:
			interface.set_temperature_diffuse_coef(val)
	get():
		return temperature_diffuse_coef
@export var temperature_noise_seed: int = 6222:
	set(val):
		temperature_noise_seed = val
		if interface != null:
			interface.set_temperature_noise_seed(val)
	get():
		return temperature_noise_seed
@export var temperature_noise_frequency: float = 2.0:
	set(val):
		temperature_noise_frequency = val
		if interface != null:
			interface.set_temperature_noise_frequency(val)
	get():
		return temperature_noise_frequency
@export var temperature_noise_amplitude: float = 0.1:
	set(val):
		temperature_noise_amplitude = val
		if interface != null:
			interface.set_temperature_noise_amplitude(val)
	get():
		return temperature_noise_amplitude
@export var temperature_noise_lacunarity: float = 2.5:
	set(val):
		temperature_noise_lacunarity = val
		if interface != null:
			interface.set_temperature_noise_lacunarity(val)
	get():
		return temperature_noise_lacunarity
@export var temperature_noise_persistence: float = 0.5:
	set(val):
		temperature_noise_persistence = val
		if interface != null:
			interface.set_temperature_noise_persistence(val)
	get():
		return temperature_noise_persistence

@export_group("strength", "strength_")
@export var strength_noise_seed: int = 0:
	set(val):
		strength_noise_seed = val
		if interface != null:
			interface.set_strength_noise_seed(val)
	get():
		return strength_noise_seed
@export var strength_noise_octaves: int = 6:
	set(val):
		strength_noise_octaves = val
		if interface != null:
			interface.set_strength_noise_octaves(val)
	get():
		return strength_noise_octaves
@export var strength_noise_frequency: float = 1.0:
	set(val):
		strength_noise_frequency = val
		if interface != null:
			interface.set_strength_noise_frequency(val)
	get():
		return strength_noise_frequency
@export var strength_noise_lacunarity: float = 5.0:
	set(val):
		strength_noise_lacunarity = val
		if interface != null:
			interface.set_strength_noise_lacunarity(val)
	get():
		return strength_noise_lacunarity
@export var strength_noise_persistence: float = 0.5:
	set(val):
		strength_noise_persistence = val
		if interface != null:
			interface.set_strength_noise_persistence(val)
	get():
		return strength_noise_persistence
@export var strength_noise_amplitude: float = 1.0:
	set(val):
		strength_noise_amplitude = val
		if interface != null:
			interface.set_strength_noise_amplitude(val)
	get():
		return strength_noise_amplitude
@export var strength_noise_mean: float = 0.0:
	set(val):
		strength_noise_mean = val
		if interface != null:
			interface.set_strength_noise_mean(val)
	get():
		return strength_noise_mean
@export var strength_noise_weight: float = 1.0:
	set(val):
		strength_noise_weight = val
		if interface != null:
			interface.set_strength_noise_weight(val)
	get():
		return strength_noise_weight
@export var strength_thermal_weight: float = 0.1:
	set(val):
		strength_thermal_weight = val
		if interface != null:
			interface.set_strength_thermal_weight(val)
	get():
		return strength_thermal_weight

@export_group("stress", "stress_")
@export var stress_transmission_threshold: float = 0.1:
	set(val):
		stress_transmission_threshold = val
		if interface != null:
			interface.set_stress_transmission_threshold(val)
	get():
		return stress_transmission_threshold
@export var stress_base_strength: float = 0.1:
	set(val):
		stress_base_strength = val
		if interface != null:
			interface.set_stress_base_strength(val)
	get():
		return stress_base_strength
@export var stress_strength_mult: float = 1.0:
	set(val):
		stress_strength_mult = val
		if interface != null:
			interface.set_stress_strength_mult(val)
	get():
		return stress_strength_mult
@export var stress_base_stress_mult: float = 1.0:
	set(val):
		stress_base_stress_mult = val
		if interface != null:
			interface.set_stress_base_stress_mult(val)
	get():
		return stress_base_stress_mult
@export var stress_reference_distance: float = 1.0:
	set(val):
		stress_reference_distance = val
		if interface != null:
			interface.set_stress_reference_distance(val)
	get():
		return stress_reference_distance
@export var stress_diffuse_weight: float = 0.2:
	set(val):
		stress_diffuse_weight = val
		if interface != null:
			interface.set_stress_diffuse_weight(val)
	get():
		return stress_diffuse_weight
@export var stress_diffuse_steps: int = 5:
	set(val):
		stress_diffuse_steps = val
		if interface != null:
			interface.set_stress_diffuse_steps(val)
	get():
		return stress_diffuse_steps
@export var stress_noise_seed: int = 0:
	set(val):
		stress_noise_seed = val
		if interface != null:
			interface.set_stress_noise_seed(val)
	get():
		return stress_noise_seed
@export var stress_noise_octaves: int = 6:
	set(val):
		stress_noise_octaves = val
		if interface != null:
			interface.set_stress_noise_octaves(val)
	get():
		return stress_noise_octaves
@export var stress_noise_frequency: float = 1.0:
	set(val):
		stress_noise_frequency = val
		if interface != null:
			interface.set_stress_noise_frequency(val)
	get():
		return stress_noise_frequency
@export var stress_noise_amplitude: float = 0.5:
	set(val):
		stress_noise_amplitude = val
		if interface != null:
			interface.set_stress_noise_amplitude(val)
	get():
		return stress_noise_amplitude
@export var stress_noise_lacunarity: float = 1.0:
	set(val):
		stress_noise_lacunarity = val
		if interface != null:
			interface.set_stress_noise_lacunarity(val)
	get():
		return stress_noise_lacunarity
@export var stress_noise_persistence: float = 5.0:
	set(val):
		stress_noise_persistence = val
		if interface != null:
			interface.set_stress_noise_persistence(val)
	get():
		return stress_noise_persistence
@export var stress_noise_strength_weight: float = 2.0:
	set(val):
		stress_noise_strength_weight = val
		if interface != null:
			interface.set_stress_noise_strength_weight(val)
	get():
		return stress_noise_strength_weight
@export var stress_noise_max: float = 0.1:
	set(val):
		stress_noise_max = val
		if interface != null:
			interface.set_stress_noise_max(val)
	get():
		return stress_noise_max

@export_group("isostasy", "isostasy_")
@export var isostasy_phase_iterations: int = 3.3:
	set(val):
		isostasy_phase_iterations = val
		if interface != null:
			interface.set_isostasy_phase_iterations(val)
	get():
		return isostasy_phase_iterations
@export var isostasy_mantle_density: float = 48:
	set(val):
		isostasy_mantle_density = val
		if interface != null:
			interface.set_isostasy_mantle_density(val)
	get():
		return isostasy_mantle_density
@export var isostasy_fold_wavelength: float = 1.0:
	set(val):
		isostasy_fold_wavelength = val
		if interface != null:
			interface.set_isostasy_fold_wavelength(val)
	get():
		return isostasy_fold_wavelength
@export var isostasy_fold_gain: float = 1.2:
	set(val):
		isostasy_fold_gain = val
		if interface != null:
			interface.set_isostasy_fold_gain(val)
	get():
		return isostasy_fold_gain
@export var isostasy_fault_spacing: float = 45.0:
	set(val):
		isostasy_fault_spacing = val
		if interface != null:
			interface.set_isostasy_fault_spacing(val)
	get():
		return isostasy_fault_spacing
@export var isostasy_fault_gain: float = 1.0:
	set(val):
		isostasy_fault_gain = val
		if interface != null:
			interface.set_isostasy_fault_gain(val)
	get():
		return isostasy_fault_gain
@export var isostasy_yeild_stress: float = 0.25:
	set(val):
		isostasy_yeild_stress = val
		if interface != null:
			interface.set_isostasy_yeild_stress(val)
	get():
		return isostasy_yeild_stress
@export var isostasy_reference_thickness: float = 0.35:
	set(val):
		isostasy_reference_thickness = val
		if interface != null:
			interface.set_isostasy_reference_thickness(val)
	get():
		return isostasy_reference_thickness
@export var isostasy_smoothing_iterations: int = 2:
	set(val):
		isostasy_smoothing_iterations = val
		if interface != null:
			interface.set_isostasy_smoothing_iterations(val)
	get():
		return isostasy_smoothing_iterations
@export var isostasy_seed: int = 0x9E37_79B9:
	set(val):
		isostasy_seed = val
		if interface != null:
			interface.set_isostasy_seed(val)
	get():
		return isostasy_seed

func set_defaults():
	interface.set_cells_seed(cells_seed)
	interface.set_cells_jitter(cells_jitter)
	interface.set_cells_cell_count(cells_cell_count)
	interface.set_plates_seed(plates_seed)
	interface.set_plates_plate_count(plates_plate_count)
	interface.set_plates_priority_dev(plates_priority_dev)
	interface.set_plates_priority_mean(plates_priority_mean)
	interface.set_plates_directionnal_preference_dev(plates_directionnal_preference_dev)
	interface.set_plates_directionnal_preference_mean(plates_directionnal_preference_mean)
	interface.set_plates_velocity_mean(plates_velocity_mean)
	interface.set_plates_velocity_dev(plates_velocity_dev)
	interface.set_plates_materials(plates_materials)
	interface.set_collision_dt(collision_dt)
	interface.set_collision_boundary_threshold(collision_boundary_threshold)
	interface.set_collision_weight_stress_factor(collision_weight_stress_factor)
	interface.set_collision_self_subduction_height(collision_self_subduction_height)
	interface.set_collision_other_subduction_height(collision_other_subduction_height)
	interface.set_properties_density_noise_seed(properties_density_noise_seed)
	interface.set_properties_height_noise_seed(properties_height_noise_seed)
	interface.set_properties_octaves(properties_octaves)
	interface.set_properties_frequency(properties_frequency)
	interface.set_properties_lacunarity(properties_lacunarity)
	interface.set_properties_persistence(properties_persistence)
	interface.set_temperature_divergent_temp(temperature_divergent_temp)
	interface.set_temperature_convergent_temp(temperature_convergent_temp)
	interface.set_temperature_transform_temp(temperature_transform_temp)
	interface.set_temperature_cooldown_factor(temperature_cooldown_factor)
	interface.set_temperature_diffuse_steps(temperature_diffuse_steps)
	interface.set_temperature_diffuse_weight(temperature_diffuse_weight)
	interface.set_temperature_diffuse_coef(temperature_diffuse_coef)
	interface.set_temperature_noise_seed(temperature_noise_seed)
	interface.set_temperature_noise_frequency(temperature_noise_frequency)
	interface.set_temperature_noise_amplitude(temperature_noise_amplitude)
	interface.set_temperature_noise_lacunarity(temperature_noise_lacunarity)
	interface.set_temperature_noise_persistence(temperature_noise_persistence)
	interface.set_strength_noise_seed(strength_noise_seed)
	interface.set_strength_noise_octaves(strength_noise_octaves)
	interface.set_strength_noise_frequency(strength_noise_frequency)
	interface.set_strength_noise_lacunarity(strength_noise_lacunarity)
	interface.set_strength_noise_persistence(strength_noise_persistence)
	interface.set_strength_noise_amplitude(strength_noise_amplitude)
	interface.set_strength_noise_mean(strength_noise_mean)
	interface.set_strength_noise_weight(strength_noise_weight)
	interface.set_strength_thermal_weight(strength_thermal_weight)
	interface.set_stress_transmission_threshold(stress_transmission_threshold)
	interface.set_stress_base_strength(stress_base_strength)
	interface.set_stress_strength_mult(stress_strength_mult)
	interface.set_stress_base_stress_mult(stress_base_stress_mult)
	interface.set_stress_reference_distance(stress_reference_distance)
	interface.set_stress_diffuse_weight(stress_diffuse_weight)
	interface.set_stress_diffuse_steps(stress_diffuse_steps)
	interface.set_stress_noise_seed(stress_noise_seed)
	interface.set_stress_noise_octaves(stress_noise_octaves)
	interface.set_stress_noise_frequency(stress_noise_frequency)
	interface.set_stress_noise_amplitude(stress_noise_amplitude)
	interface.set_stress_noise_lacunarity(stress_noise_lacunarity)
	interface.set_stress_noise_persistence(stress_noise_persistence)
	interface.set_stress_noise_strength_weight(stress_noise_strength_weight)
	interface.set_stress_noise_max(stress_noise_max)
	interface.set_isostasy_phase_iterations(isostasy_phase_iterations)
	interface.set_isostasy_mantle_density(isostasy_mantle_density)
	interface.set_isostasy_fold_wavelength(isostasy_fold_wavelength)
	interface.set_isostasy_fold_gain(isostasy_fold_gain)
	interface.set_isostasy_fault_spacing(isostasy_fault_spacing)
	interface.set_isostasy_fault_gain(isostasy_fault_gain)
	interface.set_isostasy_yeild_stress(isostasy_yeild_stress)
	interface.set_isostasy_reference_thickness(isostasy_reference_thickness)
	interface.set_isostasy_smoothing_iterations(isostasy_smoothing_iterations)
	interface.set_isostasy_seed(isostasy_seed)



func _register_component(planet_id) -> int:
	return YAPGServer.add_planet_component(planet_id, "Geometry")

func _setup_interface(_component_id: int, server: _YAPGServer):
	interface = GeometryGeneratorComponentInterface.create_from_component_id(_component_id, server)
	set_defaults()
	
func _cleanup_interface():
	interface = null
