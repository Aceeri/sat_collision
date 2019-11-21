
extern crate cgmath;

pub mod object;

use entities::object::*;
use cgmath::{ Vector2, InnerSpace };

#[derive(Debug)]
struct NormalDot {
	normal: Vector2<f32>,
	min: f32,
	max: f32
}

pub fn colliding(actor1: &mut Actor, actor2: &mut Actor) -> bool {
	for hitbox1 in &actor1.hitboxes {
		let hitbox1_center: Vector2<f32>;
		let mut dots: Vec<NormalDot> = Vec::new();

		match hitbox1 {
			&Hitbox::Polygon { ref offset, ref vertices } => {
				hitbox1_center = actor1.position + offset;

				for (index_inner, item_inner) in vertices.iter().enumerate() {
					let mut layer: NormalDot = NormalDot {
						normal: Vector2 { x: item_inner.y, y: -item_inner.x },
						min: 0.0,
						max: 0.0
					};

					for (index_outer, item_outer) in vertices.iter().enumerate() {
						let dot = (item_outer + &hitbox1_center).dot(layer.normal);

						if index_outer == 0 {
							layer.max = dot;
							layer.min = dot;
						}

						if dot > layer.max {
							layer.max = dot;
						} else if dot < layer.min {
							layer.min = dot;
						}
					}

					dots.push(layer);
				}
			},
			&Hitbox::Circle { ref offset, ref radius } => {
				hitbox1_center = actor1.position + offset;
			}
		}

		for hitbox2 in &actor2.hitboxes {
			let hitbox2_center: Vector2<f32>;

			match hitbox2 {
				&Hitbox::Polygon { ref offset, ref vertices } => {
					hitbox2_center = actor2.position - offset;
					let mut colliding: bool = true;

					for (index_inner, item_inner) in vertices.iter().enumerate() {
						let layer = &dots[index_inner];
						let mut min: f32 = 0.0;
						let mut max: f32 = 0.0;

						for (index_outer, item_outer) in vertices.iter().enumerate() {
							let dot = (item_outer + &hitbox2_center).dot(layer.normal);

							if index_outer == 0 {
								max = dot;
								min = dot;
							}

							if dot > max {
								max = dot;
							} else if dot < min {
								min = dot;
							}
						}

						println!("Layer: {:?}   min: {:?} max: {:?}", layer, min, max);

						if !(min > layer.max || max > layer.min) {
							colliding = false;
							break;
						}
					}

					println!("Colliding: {:?}", colliding);
				},
				&Hitbox::Circle { ref offset, ref radius } => {
					hitbox2_center = actor2.position - offset;
				}
			}
		}
	}

	false
}
