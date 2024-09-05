use crate::compass::Compass;
use crate::globals::Globals;
use crate::meteor::Meteor;
use godot::engine::{
    Area2D,
    AudioStreamPlayer2D,
    CollisionObject2D,
    CollisionPolygon2D,
    CpuParticles2D,
    Engine,
    InputEvent,
    InputEventMouseMotion,
    Label,
    Node2D,
    Timer,
};
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=Node2D)]
struct Stage {
    base: Base<Node2D>,
    #[export]
    start_menu: GString, /* Godot 4.* is crashing on cycled reference */
    #[export]
    bullet_scene: Gd<PackedScene>,
    #[export]
    meteor_scene: Gd<PackedScene>,
    #[export]
    amount_of_meteors: i32,
    #[export]
    accel: real,
    #[export]
    rotation_speed: real,
    player: Option<Gd<Area2D>>,
    compass: Option<Gd<Compass>>,
    engine: Option<Gd<AudioStreamPlayer2D>>,
    shot: Option<Gd<AudioStreamPlayer2D>>,
    fire: Option<Gd<CpuParticles2D>>,
    theme: Option<Gd<AudioStreamPlayer>>,
    shoot_timer: Option<Gd<Timer>>,
    start_menu_scene: Option<Gd<PackedScene>>,
    speed: Vector2,
    can_shoot: bool,
    score: u32,
    score_label: Option<Gd<Label>>,
    highscore_label: Option<Gd<Label>>,
    highscore: u32,
    alive: bool,
}

impl Stage {
    unwrap![player: Area2D];
    unwrap![compass: Compass];
    unwrap![engine: AudioStreamPlayer2D];
    unwrap![shot: AudioStreamPlayer2D];
    unwrap![fire: CpuParticles2D];
    unwrap![theme: AudioStreamPlayer];
    unwrap![start_menu_scene: PackedScene];
    unwrap![shoot_timer: Timer];
    unwrap![score_label: Label];
    unwrap![highscore_label: Label];
}

impl Stage {

    fn start_meteors(&mut self) {
        let amount = self.amount_of_meteors;
        for _ in 0..amount {
            let mut meteor: Gd<Meteor> = self.get_meteor_scene()
                .instantiate()
                .unwrap()
                .cast();
            {
                let mut meteor = meteor.bind_mut();
                meteor.go_to_random_position();
            }
            meteor.call_deferred("reload".into(), &[]);
            self.base_mut().connect("moved".into(), meteor.callable("on_stage_moved"));
            meteor.connect("throw_debris".into(), self.base_mut().callable("on_meteor_throw_debris"));
            meteor.connect("score".into(), self.base_mut().callable("on_score"));
            self.base_mut().add_child(meteor.upcast());
        }
    }

    fn accel_action(&mut self, delta: real) {
        let speed = Vector2::LEFT.rotated(self.player().get_rotation()) * self.accel;
        self.speed -= speed * delta;
    }

    fn fire_action(&mut self) {
        self.can_shoot = false;
        self.shoot_timer().start();
        let mut bullet: Gd<Area2D> = self.get_bullet_scene()
            .instantiate()
            .unwrap()
            .cast();
        let speed_factor: real = bullet.get("speed_factor".into()).to();
        let speed = self.speed + Vector2::RIGHT.rotated(self.player().get_rotation()) * speed_factor;
        bullet.set("speed".into(), Variant::from(speed));
        bullet.set_position(self.player().get_position());
        self.base_mut().connect("moved".into(), bullet.callable("on_stage_moved"));
        self.base_mut().add_child(bullet.upcast());
    }

    fn left_stick_action(&mut self, delta: real, input: &Gd<Input>) {
        let rotation = Vector2::new(
            input.get_axis("go left".into(), "go right".into()),
            input.get_axis("go up".into(), "go down".into()),
        ).normalized();
        let rotation = (rotation.angle() - self.player().get_rotation()) * delta;
        let rotation = self.player().get_rotation() + rotation;
        self.player().set_rotation(rotation);
    }

    fn arrows_action(&mut self, delta: real, input: &Gd<Input>) {
        let rotation_speed = self.rotation_speed;
        self.player().rotate(
            input.get_axis("turn left".into(), "turn right".into())
            * rotation_speed * delta
        );
    }

    fn turn_compass(&mut self) {
        let meteors: Vec<Gd<Meteor>> = self.base()
            .get_children()
            .iter_shared()
            .filter(|e| e.is_in_group("meteor".into()))
            .map(|e| e.cast::<Meteor>())
            .collect();
        let player_position = self.player().get_position();
        let mut closer: Option<Vector2> = None;
        for meteor in meteors {
            let meteor: Gd<Meteor> = meteor.cast();
            let position = meteor.get_position();
            if closer.is_none()
                || position.distance_squared_to(player_position)
                    < closer.unwrap().distance_squared_to(player_position) {
                        let _ = closer.insert(position);
            }
        }
        if let Some(closer) = closer {
            self.compass().call("set_closer".into(), &[Variant::from(closer)]);
        } else {
            self.compass().call("no_closer".into(), &[]);
        }
    }
}

#[godot_api]
impl INode2D for Stage {

    fn ready(&mut self) {
        self.player = Some(self.base().get_node_as("Player"));
        self.compass = Some(self.base().get_node_as("Compass"));
        self.engine = Some(self.base().get_node_as("Player/Engine"));
        self.shot = Some(self.base().get_node_as("Player/Shot"));
        self.fire = Some(self.base().get_node_as("Player/Fire"));
        self.theme = Some(self.base().get_node_as("Theme"));
        self.start_menu_scene = Some(load(self.start_menu.clone()));
        self.shoot_timer = Some(self.base().get_node_as("ShootTimer"));
        self.speed = Vector2::ZERO;
        self.score = 0;
        self.score_label = Some(self.base().get_node_as("HUD/Score"));
        self.highscore_label = Some(self.base().get_node_as("HUD/HighScore"));
        self.alive = true;
        self.start_meteors();

        let globals: Gd<Globals> = Engine::singleton()
            .get_singleton("Globals".into())
            .unwrap()
            .cast();
        let globals = globals.bind();
        self.highscore = globals.highscore;
        self.highscore_label().set_text(globals.highscore.to_string().into());
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as real;
        let input = Input::singleton();

        if self.alive {
            if input.is_action_pressed("accel".into()) {
                self.accel_action(delta);
                self.fire().set_emitting(true);
                if !self.engine().is_playing() {
                    self.engine().play();
                }
            } else {
                self.fire().set_emitting(false);
                self.engine().stop();
            }

            if self.can_shoot && input.is_action_pressed("fire".into()) {
                self.shot().play();
                self.fire_action();
            }

            if input.is_action_pressed("go left".into()) || input.is_action_pressed("go right".into())
                || input.is_action_pressed("go up".into()) || input.is_action_pressed("go down".into()) {
                    self.left_stick_action(delta, &input);
            }

            self.arrows_action(delta, &input);
            self.turn_compass();

        } else {
            self.fire().set_emitting(false);
            self.engine().stop();
            self.compass().set_visible(false);
        }

        let speed = self.speed * -1.0;
        self.base_mut().emit_signal(
            "moved".into(),
            &[Variant::from(speed * delta)],
        );
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("exit".into()) {
            self.base()
                .get_tree()
                .unwrap()
                .change_scene_to_packed(self.start_menu_scene().clone());
            return;
        }
        if event.is_class("InputEventMouseMotion".into()) {
            let mouse: Gd<InputEventMouseMotion> = event.clone().cast();
            self.player().look_at(mouse.get_position());
        }
    }
}

#[godot_api]
impl Stage {
    #[signal]
    fn moved(movement: Vector2);

    #[func]
    fn on_shoot_timer_timeout(&mut self) {
        self.can_shoot = true;
    }

    #[func]
    fn on_meteor_throw_debris(&mut self, mut debris: Gd<Node>) {
        self.base_mut().connect("moved".into(), debris.callable("on_stage_moved"));
        debris.connect("throw_debris".into(), self.base_mut().callable("on_meteor_throw_debris"));
        debris.connect("score".into(), self.base_mut().callable("on_score"));
        self.base_mut().add_child(debris);
    }

    #[func]
    fn on_player_body_entered(&mut self, body: Gd<CollisionObject2D>) {
        if body.is_in_group("meteor".into()) {
            self.player().set_deferred("visible".into(), Variant::from(false));
            let mut collision: Gd<CollisionPolygon2D> = self.base().get_node_as("Player/Collision");
            collision.set_deferred("disabled".into(), Variant::from(true));
            self.alive = false;

            let mut explosion_sound: Gd<AudioStreamPlayer> = self.base().get_node_as("PlayerExplosionSound");
            explosion_sound.play();
            let mut explosion: Gd<CpuParticles2D> = self.base().get_node_as("PlayerExplosion");
            explosion.set_emitting(true);
            let mut label: Gd<Label> = self.base().get_node_as("HUD/GameOver");
            label.set_visible(true);
            self.theme().stop();

            let mut globals: Gd<Globals> = Engine::singleton()
                .get_singleton("Globals".into())
                .unwrap()
                .cast();
            let mut globals = globals.bind_mut();
            globals.save();
        }
    }

    #[func]
    fn on_score(&mut self, bonus: i32) {
        if self.alive {
            let score = self.score as i32 + bonus;
            self.score = score.max(0) as u32;
            let score = self.score;
            let highscore = self.highscore.max(score);
            self.highscore = highscore;
            self.score_label().set_text(format!("{}", score).into());
            self.highscore_label().set_text(format!("{}", highscore).into());

            let mut globals: Gd<Globals> = Engine::singleton()
                .get_singleton("Globals".into())
                .unwrap()
                .cast();
            let mut globals = globals.bind_mut();
            globals.highscore = highscore;
        }
    }
}
