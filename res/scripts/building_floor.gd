class_name BuildingFloor extends Node3D

var id: int

static func mk(i: int) -> BuildingFloor:
	var r := preload("res://scenes/building_floor.tscn").instantiate()
	r.id = i
	return r

func new_construction(c) -> void:
	add_child(c)
