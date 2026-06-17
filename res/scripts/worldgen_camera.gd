extends Camera3D

func _process(delta: float):
	var up := Input.get_axis("up", "down")
	var dir := Input.get_vector("left", "right", "forward", "backward")
	var vec := (basis * Vector3(dir.x, -up, dir.y)).normalized()
	
	position += vec
