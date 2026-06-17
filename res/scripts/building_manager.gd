extends Node3D

var map: Map
var buildings: Dictionary[int, Building]

func setup(_map: Map) -> void:
	map = _map

func update_all() -> void:
	var bids := map.building_ids()
	
	for bid in bids:
		var b := Building.mk(map, bid)
		
		for fid in range(0, map.floors(bid)):
			var f := BuildingFloor.mk(map, bid, fid)
			var cs = map.load_floor_constructions(bid, fid)
			for c in cs: f.new_construction(c)
			b.new_floor(f)
		
		add_child(b)
