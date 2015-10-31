
extern crate cgmath;

use self::cgmath::{ Vector2 };

#[derive(Debug)]
pub struct Actor {
	pub position: Vector2<f32>,
	pub rotation: f32,
	pub hitboxes: Vec<Hitbox>
}

impl Actor {
	pub fn new() -> Actor {
		Actor {
			position: Vector2 { x: 0.0, y: 0.0 },
			rotation: 0.0f32,
			hitboxes: Vec::new()
		}
	}

	pub fn add_hitbox(self: &mut Actor, hitbox: Hitbox) {
		self.hitboxes.push(hitbox);
	}
}

#[derive(Debug)]
pub enum Hitbox {
	Polygon { offset: Vector2<f32>, vertices: Vec<Vector2<f32>> },
	Circle { offset: Vector2<f32>, radius: f32 }
}