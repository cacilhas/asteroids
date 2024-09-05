use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;


const MIN_ALPHA: real = 0.0;
const MAX_ALPHA: real = 0.6;
const MIN_DIST: real = 240_000.0;

#[derive(Debug, GodotClass)]
#[class(init, base=Sprite2D)]
pub(crate) struct Compass {
    base: Base<Sprite2D>,
    closer: Option<Vector2>,
    change: f32,
}

#[godot_api]
impl ISprite2D for Compass {

    fn ready(&mut self) {
        self.change = 0.8;
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as real;
        let change = self.change;
        let mut sqr_dist = 0.0;

        if let Some(closer) = self.closer {
            sqr_dist = self.base().get_position().distance_squared_to(closer);
            let mut rotation = self.base().get_rotation();
            self.base_mut().look_at(closer);
            let angle = self.base().get_rotation();
            rotation += (angle - rotation) * 2.0 * delta;
            self.base_mut().set_rotation(rotation);
        }

        let mut modulate = self.base().get_modulate();
        if sqr_dist > MIN_DIST {
            modulate.a = (modulate.a + change * delta).clamp(MIN_ALPHA, MAX_ALPHA);
        } else {
            modulate.a = (modulate.a - change.abs() * delta).clamp(MIN_ALPHA, MAX_ALPHA);
        }
        if modulate.a == MIN_ALPHA {
            self.change = change.abs();
        } else if modulate.a == MAX_ALPHA {
            self.change = -change.abs();
        }
        self.base_mut().set_modulate(modulate);
    }
}

#[godot_api]
impl Compass {

    #[func]
    fn set_closer(&mut self, closer: Vector2) {
        self.closer = Some(closer);
    }

    #[func]
    fn no_closer(&mut self) {
        self.closer = None;
    }
}
