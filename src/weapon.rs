
use rand::{thread_rng, Rng};

use crate::{
	sprite::Sprite,
	Pos,
	creature::Alignment
};


#[derive(Debug, Clone)]
pub struct Weapon {
	cooldown: i64,
	ammo: Ammo,
	nbullets: i64,
	spread_pct: i64
}

impl Weapon {

	
	pub fn shoot(&self, pos: Pos, mut direction: Pos, alignment: Alignment) -> Vec<Bullet> {
		if self.spread_pct != 0 {
			let mut rng = thread_rng();
			let deviation = self.spread_pct * direction.size();
			direction = direction * 100 + Pos::new(rng.gen_range(-deviation..=deviation), rng.gen_range(-deviation..=deviation));
		}
		vec![Bullet {
			direction,
			pos,
			alignment: alignment,
			ammo: self.ammo.clone(),
			steps: Pos::new(0, 0)
		}]
	}
	
	pub fn get_range(&self) -> i64 {
		self.ammo.range
	}
	
	pub fn get_cooldown(&self) -> i64 {
		self.cooldown
	}
	
	pub fn bite(damage: i64, cooldown: i64) -> Self {
		Weapon {
			cooldown,
			ammo: Ammo {
				damage,
				range: 1,
				speed: 2,
				sprites: vec![Sprite("bite")],
				spreading: false
			},
			nbullets: 1,
			spread_pct: 0
		}
	}
	
	pub fn cast(damage: i64, range: i64, spread_pct: i64, cooldown: i64) -> Self {
		Weapon {
			cooldown,
			nbullets: 1,
			spread_pct,
			ammo: Ammo {
				damage,
				range,
				speed: 1,
				sprites: vec![Sprite("bullet")],
				spreading: false
			}
		}
	}
	
	pub fn smg() -> Self {
		Weapon {
			cooldown: 0,
			nbullets: 1,
			spread_pct: 0,
			ammo: Ammo {
				damage: 10,
				range: 28,
				speed: 3,
				sprites: vec![Sprite("bulletvert"), Sprite("bullethor")],
				spreading: true
			}
		}
	}
	
	pub fn none() -> Self {
		Weapon {
			cooldown: 0,
			nbullets: 0,
			spread_pct: 0,
			ammo: Ammo {
				damage: 0,
				range: 0,
				speed: 1,
				sprites: vec![],
				spreading: false
			}
		}
	}
}


#[derive(Debug, Clone)]
pub struct Ammo {
	pub damage: i64,
	pub range: i64,
	pub sprites: Vec<Sprite>,
	pub speed: i64,
	pub spreading: bool
}



#[derive(Debug, Clone)]
pub struct Bullet {
	pub direction: Pos,
	pub steps: Pos,
	pub pos: Pos,
	pub alignment: Alignment,
	pub ammo: Ammo
}


impl Bullet {
	
	pub fn do_move(&mut self){
		if self.ammo.spreading {
			self.pos += self.inaccurate_movement();
		}
		let d = self.movement();
		self.pos += d;
		self.steps.x += if d.x == 0 { 0 } else { 1 };
		self.steps.y += if d.y == 0 { 0 } else { 1 };
	}
	
	fn inaccurate_movement(&self) -> Pos {
		/* sometimes move sideways to simulate inaccuracy */
		if self.steps.size() == 1 && rand::random() {
			let r = if rand::random() { 1 } else { -1 };
			if self.direction.y.abs() > self.direction.x.abs() {
				Pos::new(r, 0)
			} else {
				Pos::new(0, r)
			}
		} else {
			Pos::new(0, 0)
		}
	}
	
	fn movement(&self) -> Pos {
		/* regular movement */
		let dabs = self.direction.abs();
		
		if quadrant_move_y(dabs, self.steps) {
			Pos::new(0, self.direction.y.signum())
		} else {
			Pos::new(self.direction.x.signum(), 0)
		}
	}
	
	pub fn out_of_range(&self) -> bool {
		self.steps.size() > self.ammo.range
	}
	
	
	pub fn sprite(&self) -> Sprite {
		let sprites = &self.ammo.sprites;
		if sprites.len() > 1 && (self.direction.x.abs() > self.direction.y.abs()) {
			sprites[1]
		} else {
			sprites[0]
		}
	}
}


fn quadrant_move_y(dir: Pos, steps: Pos) -> bool {
	if dir.size() == 0 {
		// doesn't matter what gets returned; the result is 0,0 anyways
		false
	} else if dir.y > dir.x || dir.x == dir.y && rand::random() { 
		!octant_move_y(Pos::new(dir.y, dir.x), Pos::new(steps.y, steps.x))
	} else {
		octant_move_y(dir, steps)
	}
}

fn octant_move_y(dir: Pos, steps: Pos) -> bool {
	// 0 < dir.x
	// 0 <= dir.y <= dir.x
	// 0 <= steps.x
	// 0 <= steps.y
	dir.y * steps.x > steps.y * dir.x + dir.x / 2
}
