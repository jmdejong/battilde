
use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};

use crate::{
	sprite::Sprite,
	Pos,
	Direction,
	bullet::Ammo,
	PlayerId
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId),
	Zombie,
	Destroyer,
	Pillar
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureType {
	Zombie,
	Ymp,
	Worm,
	Troll,
	Pillar,
	Player
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
	#[allow(dead_code)]
	Players,
	#[allow(dead_code)]
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
	pub is_building: bool
}

impl Creature {
	pub fn new_player(playerid: PlayerId, sprite: Sprite, pos: Pos, pvp: bool) -> Self {
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
				sprites: vec![Sprite("bulletvert"), Sprite("bullethor")],
				aim: 1,
				accuracy: 12
			},
			alignment: if pvp {Alignment::Player(playerid.clone())} else {Alignment::Players},
			is_building: false
		}
	}
	
	pub fn new_pillar(pos: Pos) -> Self {
		Self {
			mind: Mind::Pillar,
			pos,
			dir: Direction::North,
			health: 200,
			max_health: 200,
			cooldown: rand::random::<i64>() % 3,
			max_cooldown: 2,
			sprite: Sprite("pillar"),
			ammo: Ammo {
				damage: 10,
				range: 1,
				speed: 2,
				sprites: vec![Sprite("bite")],
				aim: 10,
				accuracy: 10
			},
			alignment: Alignment::Players,
			is_building: true
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
			sprite: Sprite("zombie"),
			ammo: Ammo {
				damage: 10,
				range: 1,
				speed: 2,
				sprites: vec![Sprite("bite")],
				aim: 10,
				accuracy: 10
			},
			alignment: Alignment::Monsters,
			is_building: false
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
			sprite: Sprite("ymp"),
			ammo: Ammo {
				damage: 10,
				range: 24,
				speed: 1,
				sprites: vec![Sprite("bullet")],
				aim: 120,
				accuracy: 20
			},
			alignment: Alignment::Monsters,
			is_building: false
		}
	}
	
	pub fn new_troll(pos: Pos) -> Self {
		Self {
			mind: Mind::Destroyer,
			pos,
			dir: Direction::North,
			health: 100,
			max_health: 100,
			cooldown: thread_rng().gen_range(0..3),
			max_cooldown: 4,
			sprite: Sprite("troll"),
			ammo: Ammo {
				damage: 50,
				range: 2,
				speed: 1,
				sprites: vec![Sprite("bullet")],
				aim: 120,
				accuracy: 20
			},
			alignment: Alignment::Monsters,
			is_building: false
		}
	}
	
	pub fn new_worm(pos: Pos) -> Self {
		Self {
			mind: Mind::Destroyer,
			pos,
			dir: Direction::North,
			health: 12,
			max_health: 12,
			cooldown: thread_rng().gen_range(0..3),
			max_cooldown: 3,
			sprite: Sprite("worm"),
			ammo: Ammo {
				damage: 10,
				range: 2,
				speed: 1,
				sprites: vec![Sprite("bullet")],
				aim: 120,
				accuracy: 20
			},
			alignment: Alignment::Monsters,
			is_building: false
		}
	}
	
	
	pub fn create_creature(typ: CreatureType, pos: Pos) -> Self{
		match typ {
			CreatureType::Player => Self::new_player(PlayerId("".to_string()), Sprite("player_g:X"), pos, true), // will probably commite suicide immediately
			CreatureType::Zombie => Self::new_zombie(pos),
			CreatureType::Ymp => Self::new_ymp(pos),
			CreatureType::Worm => Self::new_worm(pos),
			CreatureType::Troll => Self::new_troll(pos),
			CreatureType::Pillar => Self::new_pillar(pos),
		}
	}
}
