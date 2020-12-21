
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
				|| dabs.x == dabs.y && rand::random() {
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
			sprites[1].clone()
		} else {
			sprites[0].clone()
		}
	}
}
