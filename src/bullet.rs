
use crate::{sprite::Sprite, Pos, Direction, creature::Alignment};

#[derive(Debug, Clone)]
pub struct Ammo {
	pub damage: i64,
	pub range: i64,
	pub sprite: Sprite,
	pub speed: i64,
	pub aim: i64,
	pub spread: i64
}



#[derive(Debug, Clone)]
pub struct Bullet{
	pub direction: Direction,
	pub pos: Pos,
	pub alignment: Alignment,
	pub ammo: Ammo
}
