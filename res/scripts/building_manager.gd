extends Node3D

var map: Map
var buildings: Dictionary[int, Building]

func setup(_map: Map) -> void:
	map = _map
	map.on_new_building.connect(_on_new_building)
	map.on_new_floor.connect(_on_new_floor)
	map.on_new_floor_construction.connect(_on_new_floor_construction)

func _on_new_building(building: int) -> void:
	# make a building
	var b := Building.mk(building)
	b.position = map.building_pos(building)
	
	# add it
	buildings[building] = b
	add_child(b)

func _on_new_floor(b: int, f: int) -> void:
	var y := map.floor_y(b, f)
	var flr := BuildingFloor.mk(f)
	flr.position.y = y
	buildings[b].new_floor(flr)

func _on_new_floor_construction(building: int, flr: int, construction: int) -> void:
	var cons := map.load_floor_construction(building, flr, construction)
	buildings[building].floors[flr].new_construction(cons)
