class_name BuildingFloor extends Node3D

var map: Map
var id: int
var bid: int

static func mk(m: Map, i: int, b: int) -> BuildingFloor:
	var r := preload("res://scenes/building_floor.tscn").instantiate()
	r.map = m
	r.id = i
	r.bid = b
	r.position.y = m.floor_y(b, i)
	return r

func new_construction(c) -> void:
	add_child(c)
