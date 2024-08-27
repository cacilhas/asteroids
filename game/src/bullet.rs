use godot::engine::{Area2D, CollisionObject2D, IArea2D};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Area2D)]
struct Bullet {
    base: Base<Area2D>,
    #[export]
    speed_factor: real,
    #[var]
    speed: Vector2,
}

#[godot_api]
impl IArea2D for Bullet {

    fn process(&mut self, delta: f64) {
        let position = self.base().get_global_position();
        if position.x < -640.0 || position.x > 1920.0
            || position.y < -360.0 || position.y > 1080.0 {
                self.base_mut().queue_free();
                return;
        }

        let delta = delta as real;
        let mut position = self.base().get_position();
        position += self.speed * delta;
        self.base_mut().set_position(position);
    }
}

#[godot_api]
impl Bullet {

    #[func]
    fn on_stage_moved(&mut self, movement: Vector2) {
        let position = self.base().get_position() + movement;
        self.base_mut().set_position(position);
    }

    #[func]
    fn on_body_entered(&mut self, mut body: Gd<CollisionObject2D>) {
        if body.is_in_group("meteor".into()) {
            body.call("on_hit".into(), &[]);
            self.base_mut().queue_free();
        }
    }
}
