class_name Building extends Node3D

var map: Map
var id: int
var floors: Dictionary[int, BuildingFloor]

static func mk(m: Map, i: int) -> Building:
	var b := preload("res://scenes/building.tscn").instantiate()
	b.map = m
	b.id = i
	b.position = m.building_pos(i)
	return b

func new_floor(f: BuildingFloor) -> void:
	floors[f.id] = f
	add_child(f)
