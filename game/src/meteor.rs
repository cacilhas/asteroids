use std::f64::consts::{PI, TAU};

use godot::engine::{AudioStreamPlayer2D, RigidBody2D, CollisionShape2D, IRigidBody2D, Sprite2D, Timer};
use godot::global::{pow, randf_range, randi_range};
use godot::obj::NewAlloc;
use godot::prelude::*;

#[derive(Debug, GodotClass)]
#[class(init, base=RigidBody2D)]
pub(crate) struct Meteor {
    base: Base<RigidBody2D>,
    #[export]
    min_force: real,
    #[export]
    max_force: real,
    #[var]
    hits: i32,
    hit: bool,
    sprite: Option<Gd<Sprite2D>>,
    explosion: Option<Gd<AudioStreamPlayer2D>>,
    collision: Option<Gd<CollisionShape2D>>,
    self_scene: Option<Gd<PackedScene>>,
}

impl Meteor {
    unwrap![sprite: Sprite2D];
    unwrap![explosion: AudioStreamPlayer2D];
    unwrap![collision: CollisionShape2D];
    unwrap![self_scene: PackedScene];
}

#[godot_api]
impl IRigidBody2D for Meteor {

    fn ready(&mut self) {
        self.explosion = Some(self.base().get_node_as("Explosion"));
        self.collision = Some(self.base().get_node_as("Collision"));
        self.self_scene = Some(load("res://meteor.tscn"));
    }

    fn process(&mut self, _delta: f64) {
        if self.hit {
            self.after_hit();
        }
        {
            let hits = self.hits as f64;
            self.base_mut().set_scale(Vector2::ONE / pow(2_f64.sqrt(), hits) as real);
            self.base_mut().set_mass(1.0 / pow(2.0, hits) as real);

            let speed = self.base().get_linear_velocity();
            if speed.length_squared() > 225_000_000.0 {
                self.base_mut().set_linear_velocity(speed.normalized() * 25_000.0);
            }
        }

        let mut position = self.base().get_global_position();
        if position.x < -1_280.0 {
            position.x += 3_840.0;
        } else if position.x > 2_560.0 {
            position.x -= 3_840.0;
        }
        if position.y < -720.0 {
            position.y += 2_160.0;
        } else if position.y > 1_440.0 {
            position.y -= 2_160.0;
        }
        self.base_mut().set_global_position(position);
    }

    fn physics_process(&mut self, _delta: f64) {
        // FIXME: WORKAROUND rigid bodies not moving
        let force = self.base().get_linear_velocity();
        let rotation = self.base().get_angular_velocity();
        self.base_mut().move_and_collide(force);
        self.base_mut().rotate(rotation);
    }
}

#[godot_api]
impl Meteor {

    /*
     * Signals
     */

    #[signal]
    fn throw_debris(debris: Gd<Node>);

    #[signal]
    fn score(bonus: i32);

    /*
     * Functions
     */

    #[func]
    pub(crate) fn go_to_random_position(&mut self) {
        let center = Vector2::new(640.0, 360.0);
        let mut position = center.clone();
        while position.distance_squared_to(center) <= 360_000.0 {
                position = Vector2::new(
                    randf_range(-1_280.0, 2_560.0) as real,
                    randf_range(-720.0, 1_440.0) as real,
                );
            }
        self.base_mut().set_position(position);
    }

    #[func]
    fn reload(&mut self) {
        self.select_skin();
        self.apply_random_force();
        self.schedule_to_enable_collision();
    }

    fn select_skin(&mut self) {
        let idx = randi_range(1, 4);
        let name = format!("Meteor{}", idx);
        let mut sprite: Gd<Sprite2D> = self.base().get_node_as(name);
        sprite.set_visible(true);
        self.sprite = Some(sprite);
    }

    fn apply_random_force(&mut self) {
        let mass = self.base().get_mass();
        let min_force = self.min_force as f64;
        let max_force = self.max_force as f64;
        self.base_mut().apply_force(
            Vector2::LEFT.rotated(randf_range(0.0, TAU) as real)
            * randf_range(min_force, max_force) as real
            * mass
        );
        self.base_mut().set_angular_velocity(randf_range(-PI, PI) as real);
    }

    fn schedule_to_enable_collision(&mut self) {
        let mut timer = Timer::new_alloc();
        timer.set_wait_time(0.5);
        timer.set_one_shot(true);
        timer.set_autostart(true);
        timer.connect("timeout".into(), self.base_mut().callable("enable_collision"));
        self.collision().add_child(timer.upcast());
    }

    /*
     * Handlers
     */

    #[func]
    fn on_stage_moved(&mut self, movement: Vector2) {
        let position = self.base().get_global_position() + movement;
        self.base_mut().set_global_position(position);
    }

    #[func]
    fn on_hit(&mut self) {
        self.sprite().set_deferred("visible".into(), Variant::from(false));
        self.collision().set_deferred("disabled".into(), Variant::from(true));
        self.hit = true;
    }

    #[func]
    fn enable_collision(&mut self) {
        self.collision().set_disabled(false);
    }

    #[func]
    fn after_hit(&mut self) {
        self.hit = false;
        if self.hits > 4 && randi_range(0, 5) > 0 {
            self.die();
        } else {
            self.crack();
        }
        self.explosion().play();
    }

    fn die(&mut self) {
        self.base_mut().emit_signal("score".into(), &[Variant::from(10)]);
        let cb = self.base_mut().callable("queue_free");
        self.explosion().connect("finished".into(), cb);
    }

    fn crack(&mut self) {
        self.base_mut().emit_signal("score".into(), &[Variant::from(1)]);
        let hits: i32 = self.hits + 1;

        let position = self.base().get_global_position();
        let displacement = Vector2::LEFT.rotated(randf_range(0.0, TAU) as real);

        self.reload();
        self.base_mut().set_global_position(position - displacement);
        self.hits = hits;

        let mut debris: Gd<Meteor> = self.self_scene().instantiate().unwrap().cast();
        {
            let mut debris = debris.bind_mut();
            debris.hits = hits;
        }
        if randi_range(0, 3) == 0 {
            debris.set("hits".into(), Variant::from(hits+1));
        } else {
            debris.set("hits".into(), Variant::from(hits));
        }

        debris.set_global_position(position + displacement);
        debris.call_deferred("reload".into(), &[]);
        self.base_mut().emit_signal(
            "throw_debris".into(),
            &[Variant::from(debris.upcast::<Node>()),
        ]);
    }
}
