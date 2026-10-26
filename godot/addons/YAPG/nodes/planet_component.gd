@tool
class_name YAPG_PlanetComponent extends Node

var _previous_parent: Node = null
var parent_planet := YAPGServer.INVALID_PLANET_ID
var component_id := YAPGServer.INVALID_COMPONENT_ID

func _notification(what: int) -> void:
	match what:
		NOTIFICATION_PARENTED, NOTIFICATION_ENTER_TREE: _check()
		NOTIFICATION_UNPARENTED, NOTIFICATION_EXIT_TREE: _cleanup()

func _check() -> void:
	update_configuration_warnings()
	
	var new_parent := get_parent()
	
	if new_parent != _previous_parent:
		_cleanup()
		_previous_parent = new_parent
	
	if component_id != YAPGServer.INVALID_COMPONENT_ID:
		return
	
	if is_inside_tree() and new_parent is YAPG_Planet:
		var planet_node := new_parent as YAPG_Planet
		parent_planet = planet_node.get_planet_id()
		
		if parent_planet != YAPGServer.INVALID_PLANET_ID:
			component_id = _register_component(parent_planet)
			if component_id != YAPGServer.INVALID_COMPONENT_ID:
				_setup_interface(component_id, YAPGServer)

func _cleanup() -> void:
	if component_id != YAPGServer.INVALID_COMPONENT_ID:
		_cleanup_interface()
		YAPGServer.remove_component(component_id)
		component_id = YAPGServer.INVALID_COMPONENT_ID
		parent_planet = YAPGServer.INVALID_PLANET_ID
	
	if get_parent() == null:
		_previous_parent = null

func _get_configuration_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	if not get_parent() is YAPG_Planet:
		warnings.append("The component MUST be a child of a YAPG_Planet node.")
	return warnings

func _register_component(_planet_id: int) -> int:
	push_error("THIS FUNCTION MUST BE IMPLEMENTED BY CHILDREN")
	return YAPGServer.INVALID_COMPONENT_ID

func _setup_interface(_component_id: int, server: _YAPGServer):
	push_warning("THIS FUNCTION MUST BE IMPLEMENTED BY CHILDREN")

func _cleanup_interface():
	push_warning("THIS FUNCTION MUST BE IMPLEMENTED BY CHILDREN")
