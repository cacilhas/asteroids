#[macro_use] mod macros;
mod bullet;
mod compass;
mod globals;
mod meteor;
mod since_string;
mod stage;
mod stars;
mod start_menu;

use globals::Globals;
use godot::engine::Engine;
use godot::global::randomize;
use godot::prelude::*;

struct Asteroids;

#[gdextension]
unsafe impl ExtensionLibrary for Asteroids {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            randomize();
            Engine::singleton().register_singleton(
                "Globals".into(),
                Globals::new_alloc().upcast(),
            )
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let globals = engine
                .get_singleton("Globals".into())
                .unwrap();
            engine.unregister_singleton("Globals".into());
            globals.free();
        }
    }
}
