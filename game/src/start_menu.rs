use std::f64::consts::PI;

use chrono::Utc;
use godot::engine::{Control, Engine, IControl, InputEvent, Label, Sprite2D};
use godot::prelude::*;
use crate::since_string::SinceString;

use crate::globals::Globals;

#[derive(Debug, GodotClass)]
#[class(init, base=Control)]
struct StartMenu {
    base: Base<Control>,
    #[export]
    stage_scene: Gd<PackedScene>,
    dart: Option<Gd<Sprite2D>>,
    highscore: Option<Gd<Label>>,
    when: Option<Gd<Label>>,
}

impl StartMenu {
    unwrap![dart: Sprite2D];
    unwrap![highscore: Label];
    unwrap![when: Label];
}

#[godot_api]
impl IControl for StartMenu {

    fn ready(&mut self) {
        self.dart = Some(self.base().get_node_as("Dart"));
        self.highscore = Some(self.base().get_node_as("HighScore"));
        self.when = Some(self.base().get_node_as("When"));
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as real;
        self.dart().rotate(-PI as real * delta);
        let globals: Gd<Globals> = Engine::singleton()
            .get_singleton("Globals".into())
            .unwrap()
            .cast();
        let globals = globals.bind();
        self.highscore().set_text(globals.highscore.to_string().into());
        if let Some(when) = globals.when {
            let since: SinceString = Utc::now().signed_duration_since(when).into();
            self.when().set_text(since.to_string().into());
            self.when().set_visible(true);
        } else {
            self.when().set_visible(false);
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("start".into()) {
            self.on_start_button_pressed();
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("exit".into()) {
            self.base().get_tree().unwrap().quit();
        }
    }
}

#[godot_api]
impl StartMenu {

    #[func]
    fn on_start_button_pressed(&mut self) {
        self.base()
            .get_tree()
            .unwrap()
            .change_scene_to_packed(self.get_stage_scene());
    }
}
