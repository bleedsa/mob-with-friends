class_name Building extends Node3D

var id: int
var floors: Dictionary[int, BuildingFloor]

static func mk(i: int) -> Building:
	var b := preload("res://scenes/building.tscn").instantiate()
	b.id = i
	return b

func new_floor(f: BuildingFloor) -> void:
	floors[f.id] = f
	add_child(f)
