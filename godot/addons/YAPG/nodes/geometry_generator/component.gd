@tool
class_name YAPG_GeometryGeneratorComponent extends YAPG_PlanetComponent

@onready var _impl: __YAPG_Geometry_Generator_Component_Impl = __YAPG_Geometry_Generator_Component_Impl.new()
@export var pipeline: YAPG_GeometryGeneratorPipeline

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
