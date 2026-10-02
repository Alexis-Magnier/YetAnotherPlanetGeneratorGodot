class_name YAPG_GeometryPlateGeneratorStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Plate_Generator_Impl.new()

@export var seed: int:
	set(val): impl.set_seed(val)
	get(): return impl.get_seed()
	
## 
@export var plate_count: int = 80:
	set(val): impl.set_plate_count(val)
	get(): return impl.get_plate_count()


## The number of cell the planet will have
@export var priority_mean: float = 0.5:
	set(val): impl.set_priority_mean(val)
	get(): return impl.get_priority_mean()


@export var priority_dev: float = 0.5:
	set(val): impl.set_priority_dev(val)
	get(): return impl.get_priority_dev()


@export var directional_preference_mean: float = 0.5:
	set(val): impl.set_directional_preference_mean(val)
	get(): return impl.get_directional_preference_mean()


@export var directional_preference_dev: float = 0.5:
	set(val): impl.set_directional_preference_dev(val)
	get(): return impl.get_directional_preference_dev()


@export var velocity_mean: float = 0.5:
	set(val): impl.set_velocity_mean(val)
	get(): return impl.get_velocity_mean()


@export var velocity_dev: float = 0.5:
	set(val): impl.set_velocity_dev(val)
	get(): return impl.get_velocity_dev()


@export var plate_materials: Array[YAPG_PlateGeneratorMaterial] = []:
	set(val): impl.set_plate_materials(val)
	get(): return impl.get_plate_materials()
