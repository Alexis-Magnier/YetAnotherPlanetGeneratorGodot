class_name YAPG_GeometryTemperatureStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Temperature_Impl.new()

@export var divergent_temp: float:
	set(val): impl.set_divergent_temp(val)
	get(): return impl.get_divergent_temp()

@export var convergent_temp: float:
	set(val): impl.set_convergent_temp(val)
	get(): return impl.get_convergent_temp()

@export var transform_temp: float:
	set(val): impl.set_transform_temp(val)
	get(): return impl.get_transform_temp()

@export var cooldown_factor: float:
	set(val): impl.set_cooldown_factor(val)
	get(): return impl.get_cooldown_factor()

@export var diffuse_step: int:
	set(val): impl.set_diffuse_step(val)
	get(): return impl.get_diffuse_step()

@export var diffuse_weight: float:
	set(val): impl.set_diffuse_weight(val)
	get(): return impl.get_diffuse_weight()

@export var diffusion_coef: float:
	set(val): impl.set_diffusion_coef(val)
	get(): return impl.get_diffusion_coef()

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
