class_name YAPG_GeometryStressStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Stress_Impl.new()

@export var transmission_threshold: float:
	set(val): impl.set_transmission_threshold(val)
	get(): return impl.get_transmission_threshold()

@export var base_strength: float:
	set(val): impl.set_base_strength(val)
	get(): return impl.get_base_strength()

@export var strength_mult: float:
	set(val): impl.set_strength_mult(val)
	get(): return impl.get_strength_mult()

@export var base_stress_mult: float:
	set(val): impl.set_base_stress_mult(val)
	get(): return impl.get_base_stress_mult()

@export var reference_distance: float:
	set(val): impl.set_reference_distance(val)
	get(): return impl.get_reference_distance()

@export var diffuse_weight: float:
	set(val): impl.set_diffuse_weight(val)
	get(): return impl.get_diffuse_weight()

@export var diffuse_steps: int:
	set(val): impl.set_diffuse_steps(val)
	get(): return impl.get_diffuse_steps()

@export var noise_seed: int:
	set(val): impl.set_noise_seed(val)
	get(): return impl.get_noise_seed()

@export var noise_octaves: int:
	set(val): impl.set_noise_octaves(val)
	get(): return impl.get_noise_octaves()

@export var noise_frequency: float:
	set(val): impl.set_noise_frequency(val)
	get(): return impl.get_noise_frequency()

@export var noise_amplitude: float:
	set(val): impl.set_noise_amplitude(val)
	get(): return impl.get_noise_amplitude()

@export var noise_lacunarity: float:
	set(val): impl.set_noise_lacunarity(val)
	get(): return impl.get_noise_lacunarity()

@export var noise_persistence: float:
	set(val): impl.set_noise_persistence(val)
	get(): return impl.get_noise_persistence()

@export var noise_strength_weight: float:
	set(val): impl.set_noise_strength_weight(val)
	get(): return impl.get_noise_strength_weight()

@export var noise_max: float:
	set(val): impl.set_noise_max(val)
	get(): return impl.get_noise_max()
