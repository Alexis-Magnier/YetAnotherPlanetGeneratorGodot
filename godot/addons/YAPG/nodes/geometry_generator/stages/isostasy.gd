class_name YAPG_GeometryIsostasyStage
extends __YAPG_Geometry_Generator_Component_Stage_Base

var impl := __YAPG_Geometry_Generator_Isostasy_Impl.new()

@export var seed: int:
	set(val): impl.set_seed(val)
	get(): return impl.get_seed()

@export var mantle_density: float:
	set(val): impl.set_mantle_density(val)
	get(): return impl.get_mantle_density()

@export var phase_iterations: int:
	set(val): impl.set_phase_iterations(val)
	get(): return impl.get_phase_iterations()

@export var fold_wavelength_km: float:
	set(val): impl.set_fold_wavelength_km(val)
	get(): return impl.get_fold_wavelength_km()

@export var fold_gain: float:
	set(val): impl.set_fold_gain(val)
	get(): return impl.get_fold_gain()

@export var fault_spacing_km: float:
	set(val): impl.set_fault_spacing_km(val)
	get(): return impl.get_fault_spacing_km()

@export var fault_gain: float:
	set(val): impl.set_fault_gain(val)
	get(): return impl.get_fault_gain()

@export var yield_stress: float:
	set(val): impl.set_yield_stress(val)
	get(): return impl.get_yield_stress()

@export var reference_thickness_km: float:
	set(val): impl.set_reference_thickness_km(val)
	get(): return impl.get_reference_thickness_km()

@export var smoothing_iterations: int:
	set(val): impl.set_smoothing_iterations(val)
	get(): return impl.get_smoothing_iterations()
