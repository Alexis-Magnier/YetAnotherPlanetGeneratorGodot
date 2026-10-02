@tool
class_name YAPG_Planet extends Node3D

var _planet:__YAPG_Planet_Impl = null

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	_planet = __YAPG_Planet_Impl.new()
	_planet.hello_world()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _notification(what: int) -> void:
	match what:
		NOTIFICATION_CHILD_ORDER_CHANGED: update_configuration_warnings()


## Checks if there are children and if they are all of correct type
func _check_children(warnings: PackedStringArray) -> void:
	var children = get_children()

	if children: 
		for node in children:
			if node is YAPG_PlanetComponent: continue
			warnings.append("Child '%s' is not a PlanetComponent" % node.name)
	else:
		warnings.append("The planet does not have any component")


func _get_configuration_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	
	_check_children(warnings)
	
	return warnings
