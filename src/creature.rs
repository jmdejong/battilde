


use crate::{sprite::Sprite, Pos, Direction, bullet::Ammo, PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId),
	Zombie,
	Building
}

#[derive(Debug, Clone, PartialEq, Eq, )]
pub enum MonsterType {
	Zombie,
	Ymp,
	
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
	#[allow(dead_code)]
	Players,
	Player(PlayerId),
	Monsters
}

#[derive(Debug, Clone)]
pub struct Creature {
	pub mind: Mind,
	pub pos: Pos,
	pub dir: Direction,
	pub health: i64,
	pub cooldown: i64,
	pub max_cooldown: i64,
	pub max_health: i64,
	pub sprite: Sprite,
	pub alignment: Alignment,
	pub ammo: Ammo,
}

impl Creature {
	pub fn new_player(playerid: PlayerId, sprite: Sprite, pos: Pos) -> Self {
		Self {
			mind: Mind::Player(playerid.clone()),
			pos,
			dir: Direction::North,
			health: 1,
			max_health: 100,
			cooldown: 0,
			max_cooldown: 0,
			sprite,
			ammo: Ammo {
				damage: 10,
				range: 32,
				speed: 3,
				sprites: vec![Sprite("bulletvert".to_string()), Sprite("bullethor".to_string())],
				aim: 1,
				accuracy: 12
			},
			alignment: Alignment::Player(playerid),
		}
	}
	
	pub fn new_zombie(pos: Pos) -> Self {
		Self {
			mind: Mind::Zombie,
			pos,
			dir: Direction::North,
			health: 20,
			max_health: 20,
			cooldown: rand::random::<i64>() % 3,
			max_cooldown: 2,
			sprite: Sprite("zombie".to_string()),
			ammo: Ammo {
				damage: 10,
				range: 1,
				speed: 2,
				sprites: vec![Sprite("bite".to_string())],
				aim: 10,
				accuracy: 10
			},
			alignment: Alignment::Monsters
		}
	}
	
	pub fn new_ymp(pos: Pos) -> Self {
		Self {
			mind: Mind::Zombie,
			pos,
			dir: Direction::North,
			health: 20,
			max_health: 20,
			cooldown: rand::random::<i64>() % 3,
			max_cooldown: 2,
			sprite: Sprite("ymp".to_string()),
			ammo: Ammo {
				damage: 10,
				range: 24,
				speed: 1,
				sprites: vec![Sprite("bullet".to_string())],
				aim: 120,
				accuracy: 20
			},
			alignment: Alignment::Monsters
		}
	}
	
	pub fn create_monster(typ: MonsterType, pos: Pos) -> Self{
		match typ {
			MonsterType::Zombie => Self::new_zombie(pos),
			MonsterType::Ymp => Self::new_ymp(pos),
		}
	}
}
