class_name YAPG_GeometryDetectCollisionsStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Detect_Collisions_Impl.new()

@export var dt: int:
	set(val): impl.set_dt(val)
	get(): return impl.get_dt()

@export var boundary_threshold: float:
	set(val): impl.set_boundary_threshold(val)
	get: return impl.get_boundary_threshold()
	
@export var pinning_weight_stress_factor: float:
	set(val): impl.set_pinning_weight_stress_factor(val)
	get: return impl.get_pinning_weight_stress_factor()
	
@export var pinning_self_subduction_height: float:
	set(val): impl.set_pinning_self_subduction_height(val)
	get: return impl.get_pinning_self_subduction_height()
	
@export var pinning_other_subduction_height: float:
	set(val): impl.set_pinning_other_subduction_height(val)
	get: return impl.get_pinning_other_subduction_height()
	
