use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;


#[derive(Debug, GodotClass)]
#[class(init, base=Sprite2D)]
pub(crate) struct Compass {
    base: Base<Sprite2D>,
    change: f32,
}

#[godot_api]
impl ISprite2D for Compass {

    fn ready(&mut self) {
        self.change = 1.0;
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as real;
        let change = self.change;
        let mut modulate = self.base().get_modulate();
        modulate.a = (modulate.a + change * delta).clamp(0.0, 1.0);
        if modulate.a == 0.0 {
            self.change = change.abs();
        } else if modulate.a == 1.0 {
            self.change = -change.abs();
        }
        self.base_mut().set_modulate(modulate);
    }
}
