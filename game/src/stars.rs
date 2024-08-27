use godot::engine::Node2D;
use godot::global::{randf_range, randi_range};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
struct Stars {
    base: Base<Node2D>,
    #[export]
    amount: i32,
    #[export]
    star_size: real,
    stars: Vec<Vector2>,
}

#[godot_api]
impl INode2D for Stars {

    fn ready(&mut self) {
        for _ in 0..self.amount {
            self.stars.push(Vector2 {
                x: randf_range(0.0, 1_280.0) as real,
                y: randf_range(0.0, 720.0) as real,
            });
        }
    }

    fn draw(&mut self) {
        let stars = self.stars.clone();
        let size = self.star_size;
        for star in stars {
            let color = COLORS[randi_range(0, 4) as usize];
            self.base_mut().draw_circle(star, size, color);
        }
    }
}

static COLORS: [Color; 5] = [
    Color {r: 1.0, g: 0.75, b: 0.75, a: 1.0},
    Color {r: 1.0, g: 0.875, b: 0.75, a: 1.0},
    Color {r: 1.0, g: 1.0, b: 0.75, a: 1.0},
    Color {r: 0.75, g: 0.75, b: 1.0, a: 1.0},
    Color {r: 0.875, g: 0.875, b: 1.0, a: 1.0},
];
