@tool
class_name YAPG_PlanetComponent extends __YAPG_Planet_Component

func _notification(what: int) -> void:
	if what == NOTIFICATION_PARENTED:
		update_configuration_warnings()

func _get_configuration_warnings() -> PackedStringArray:
	var warnings := PackedStringArray()
	
	if not get_parent() is YAPG_Planet:
		warnings.append("The component MUST be a child of a planet")
		
	return warnings
