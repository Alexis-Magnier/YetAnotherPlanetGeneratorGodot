@tool
extends EditorPlugin

func _enter_tree():
	var addon_path = get_script().resource_path.get_base_dir()
	print("path : %s", addon_path)
