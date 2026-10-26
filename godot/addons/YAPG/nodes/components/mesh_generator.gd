@tool
class_name YAPG_MeshGenerator extends YAPG_PlanetComponent

@export var material: Material:
	set(val):
		material = val
		if interface != null:
			interface.set_material(val)
	get():
		return material

var interface: MeshGeneratorComponentInterface = null

func set_defaults():
	interface.set_material(material)

func _register_component(planet_id) -> int:
	return YAPGServer.add_planet_component(planet_id, "Mesh")

func _setup_interface(_component_id: int, server: _YAPGServer):
	interface = MeshGeneratorComponentInterface.create_from_component_id(_component_id, server)
	set_defaults()
	
func _cleanup_interface():
	interface = null
	pass
