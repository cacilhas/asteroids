extends Parallax2D

# TODO: Rust doesn’t support Parallax2D yet


func _on_stage_moved(movement: Vector2) -> void:
	self.scroll_offset += movement * self.scroll_scale

