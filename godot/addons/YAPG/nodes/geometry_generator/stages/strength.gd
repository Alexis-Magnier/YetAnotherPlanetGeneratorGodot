class_name YAPG_GeometryStrengthStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Strength_Impl.new()

@export var noise_seed: int:
	set(val): impl.set_noise_seed(val)
	get(): return impl.get_noise_seed()

@export var noise_octaves: int:
	set(val): impl.set_noise_octaves(val)
	get(): return impl.get_noise_octaves()

@export var noise_frequency: float:
	set(val): impl.set_noise_frequency(val)
	get(): return impl.get_noise_frequency()

@export var noise_lacunarity: float:
	set(val): impl.set_noise_lacunarity(val)
	get(): return impl.get_noise_lacunarity()

@export var noise_persistence: float:
	set(val): impl.set_noise_persistence(val)
	get(): return impl.get_noise_persistence()

@export var noise_amplitude: float:
	set(val): impl.set_noise_amplitude(val)
	get(): return impl.get_noise_amplitude()

@export var noise_mean: float:
	set(val): impl.set_noise_mean(val)
	get(): return impl.get_noise_mean()

@export var noise_weight: float:
	set(val): impl.set_noise_weight(val)
	get(): return impl.get_noise_weight()

@export var thermal_weight: float:
	set(val): impl.set_thermal_weight(val)
	get(): return impl.get_thermal_weight()
