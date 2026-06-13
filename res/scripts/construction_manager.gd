extends Node3D

var map: Map

func setup(_map: Map) -> void:
	# setup the map and its signals
	map = _map
	map.on_new_construction.connect(_on_new_construction)

func _on_new_construction(id: int) -> void:
	var c := map.load_construction(id)
	print("new construction: ", c)
	add_child(c)
