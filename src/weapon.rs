
use rand::{thread_rng, Rng};

use crate::{
	sprite::Sprite,
	Pos,
	creature::Alignment,
	util::clamp
};

#[derive(Debug, Clone)]
pub struct Ammo {
	pub damage: i64,
	pub range: i64,
	pub sprites: Vec<Sprite>,
	pub speed: i64,
	pub aim: i64,
	pub accuracy: i64
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
	pub fn movement(&mut self) {
		/* sometimes move sideways to simulate inaccuracy */
		if self.ammo.aim == 0 {
			let d = self.direction;
			let ds = Pos::new(d.y, -d.x).clamp(Pos::new(-1, -1), Pos::new(1, 1));
			let r: u8 = thread_rng().gen_range(0..4);
			self.pos = self.pos + match (ds.size(), r) {
				(1, 1) => ds,
				(1, 2) => -ds,
				(2, 1) => Pos::new(ds.x, 0),
				(2, 2) => Pos::new(0, ds.y),
				_ => Pos::new(0, 0)
			};
			self.ammo.aim = self.ammo.accuracy
		}
		self.ammo.aim -=1;
		/* regular movement */
		let dabs = self.direction.abs();
		let dpos = if // todo: check if this is correct
				self.steps.size() == 0 && dabs.y > dabs.x 
				|| dabs.x == 0
				|| self.steps.x * dabs.y > dabs.x * self.steps.y
				|| dabs.x == dabs.y && self.steps.x == self.steps.y && rand::random() {
			self.steps.y += 1;
			Pos::new(0, clamp(self.direction.y, -1, 1))
		} else {
			self.steps.x += 1;
			Pos::new(clamp(self.direction.x, -1, 1), 0)
		};
		self.pos = self.pos + dpos;
	
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

#[derive(Debug, Clone)]
pub struct Weapon {
	cooldown: i64,
	ammo: Ammo,
	nbullets: i64
}

impl Weapon {

	
	pub fn shoot(&self, pos: Pos, direction: Pos, alignment: Alignment) -> Vec<Bullet> {
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
	
	pub fn bite(damage: i64, cooldown: i64) -> Self {
		Weapon {
			cooldown,
			ammo: Ammo {
				damage,
				range: 1,
				speed: 2,
				sprites: vec![Sprite("bite")],
				aim: 10,
				accuracy: 10
			},
			nbullets: 1
		}
	}
	
	pub fn cast(damage: i64, range: i64, cooldown: i64) -> Self {
		Weapon {
			cooldown,
			nbullets: 1,
			ammo: Ammo {
				damage,
				range,
				speed: 1,
				sprites: vec![Sprite("bullet")],
				aim: 120,
				accuracy: 20
			}
		}
	}
	
	pub fn smg() -> Self {
		Weapon {
			cooldown: 0,
			nbullets: 1,
			ammo: Ammo {
				damage: 10,
				range: 32,
				speed: 3,
				sprites: vec![Sprite("bulletvert"), Sprite("bullethor")],
				aim: 1,
				accuracy: 12
			}
		}
	}
	
	pub fn none() -> Self {
		Weapon {
			cooldown: 0,
			nbullets: 0,
			ammo: Ammo {
				damage: 0,
				range: 0,
				speed: 1,
				sprites: vec![],
				aim: 1,
				accuracy:1
			}
		}
	}
}
