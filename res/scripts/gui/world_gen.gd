extends Node3D

@onready var map: Map = $Map
@onready var construction_manager := $ConstructionManager
@onready var building_manager := $BuildingManager

@onready var materials_rejected = $Control/MaterialsRejected

func _ready() -> void:
	construction_manager.setup(map)
	building_manager.setup(map)

func _process(_delta: float) -> void:
	materials_rejected.text = "materials rejected: " + str(Stats.materials_rejected())

func _on_generate_button_pressed() -> void:
	map.generate()
