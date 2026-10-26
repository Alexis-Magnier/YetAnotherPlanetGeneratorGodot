@tool
class_name YAPG_Planet extends Node3D

@export_tool_button("Generate") var _generate_button = generate

var _id: int = YAPGServer.INVALID_PLANET_ID

func _enter_tree() -> void:
	if _id == YAPGServer.INVALID_PLANET_ID:
		_id = YAPGServer.create_planet()

func _exit_tree() -> void:
	if _id != YAPGServer.INVALID_PLANET_ID:
		YAPGServer.free_planet(_id)
		_id = YAPGServer.INVALID_PLANET_ID

func get_planet_id() -> int:
	return _id

func generate():
	assert(_id != YAPGServer.INVALID_PLANET_ID)
	YAPGServer.generate_planet(_id, self)

func tick(dt:float):
	assert(_id != YAPGServer.INVALID_PLANET_ID)
	YAPGServer.tick_planet(_id, dt)
