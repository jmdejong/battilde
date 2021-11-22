
use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};

use crate::{
	sprite::Sprite,
	Pos,
	Direction,
	weapon::Weapon,
	PlayerId
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId),
	Zombie,
	Destroyer,
	Pillar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureType {
	Zombie,
	Ymp,
	Worm,
	Troll,
	Pillar,
	Player,
	Vargr,
	Xiangliu
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
	pub walk_cooldown: i64,
	pub max_health: i64,
	pub sprite: Sprite,
	pub alignment: Alignment,
	pub weapon: Weapon,
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
			walk_cooldown: 0,
			sprite,
			weapon: Weapon::smg(),
			alignment: 
				if pvp {
					Alignment::Player(playerid)
				} else {
					Alignment::Players
				},
			is_building: false
		}
	}
	
	pub fn is_player(&self) -> bool {
		matches!(self.mind, Mind::Player(_))
	}
	
	pub fn new_pillar(pos: Pos) -> Self {
		Self {
			mind: Mind::Pillar,
			pos,
			dir: Direction::North,
			health: 200,
			max_health: 200,
			cooldown: rand::random::<i64>() % 3,
			walk_cooldown: 2,
			sprite: Sprite("pillar"),
			weapon: Weapon::none(),
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
			walk_cooldown: 2,
			sprite: Sprite("zombie"),
			weapon: Weapon::bite(10, 2),
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
			walk_cooldown: 2,
			sprite: Sprite("ymp"),
			weapon: Weapon::cast(10, 24, 2),
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
			walk_cooldown: 4,
			sprite: Sprite("troll"),
			weapon: Weapon::cast(50, 2, 4),
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
			walk_cooldown: 3,
			sprite: Sprite("worm"),
			weapon: Weapon::cast(10, 2, 3),
			alignment: Alignment::Monsters,
			is_building: false
		}
	}
	
	fn new_xiangliu(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::Zombie,
			50,
			2,
			Sprite("xiangliu"),
			Weapon::cast(10, 12, 0),
		)
	}
	
	fn new_vargr(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::Zombie,
			30,
			1,
			Sprite("vargr"),
			Weapon::bite(20, 3)
		)
	}
	
	fn new_monster(pos: Pos, mind: Mind, health: i64, cooldown: i64, sprite: Sprite, weapon: Weapon) -> Self {
		Self {
			mind,
			pos,
			dir: Direction::North,
			health,
			max_health: health,
			cooldown: thread_rng().gen_range(0..=cooldown),
			walk_cooldown: cooldown,
			sprite,
			weapon,
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
			CreatureType::Xiangliu => Self::new_xiangliu(pos),
			CreatureType::Vargr => Self::new_vargr(pos)
			
		}
	}
}
