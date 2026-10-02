class_name YAPG_GeometryCellGeneratorStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Cell_Generator_Impl.new()

@export var seed: int:
	set(val): impl.set_seed(val)
	get(): return impl.get_seed()

## The number of cell the planet will have
@export var cell_count: int = 10000:
	set(val): impl.set_cell_count(val)
	get(): return impl.get_cell_count()

@export var jitter: float = 0.5:
	set(val): impl.set_jitter(val)
	get(): return impl.get_jitter()
