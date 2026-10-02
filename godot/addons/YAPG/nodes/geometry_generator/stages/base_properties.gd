class_name YAPG_GeometryBasePropertiesStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Base_Properties_Impl.new()

@export var density_noise_seed: int:
	set(val): impl.set_density_noise_seed(val)
	get(): return impl.get_density_noise_seed()

@export var height_noise_seed: int:
	set(val): impl.set_height_noise_seed(val)
	get(): return impl.get_height_noise_seed()

@export var octaves: int:
	set(val): impl.set_octaves(val)
	get(): return impl.get_octaves()

@export var frequency: float:
	set(val): impl.set_frequency(val)
	get(): return impl.get_frequency()

@export var lacunarity: float:
	set(val): impl.set_lacunarity(val)
	get(): return impl.get_lacunarity()

@export var persistence: float:
	set(val): impl.set_persistence(val)
	get(): return impl.get_persistence()
