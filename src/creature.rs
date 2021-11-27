
use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};

use crate::{
	sprite::Sprite,
	Pos,
	Direction,
	weapon::Weapon,
	PlayerId,
	util::Percentage,
	timestamp::Duration,
	pos::Distance
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId),
	BloodThirst(Percentage),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health(pub i64);

#[derive(Debug, Clone)]
pub struct Creature {
	pub mind: Mind,
	pub pos: Pos,
	pub dir: Direction,
	pub health: Health,
	pub max_health: Health,
	pub cooldown: Duration,
	pub walk_cooldown: Duration,
	pub sprite: Sprite,
	pub alignment: Alignment,
	pub weapons: Vec<Weapon>,
	pub selected_weapon: usize,
	pub is_building: bool
}

impl Creature {
	
	pub fn is_player(&self) -> bool {
		matches!(self.mind, Mind::Player(_))
	}
	
	pub fn is_dead(&self) -> bool {
		self.health.0 <= 0
	}
	
	pub fn kill(&mut self) {
		self.health.0 = -1;
	}
	
	pub fn heal(&mut self, amount: Health) {
		self.health.0 = self.health.0.max(self.max_health.0.min(self.health.0 + amount.0));
	}
	
	pub fn has_full_health(&mut self) -> bool {
		self.health.0 >= self.max_health.0
	}
	
	pub fn damage(&mut self, amount: Health) {
		self.health.0 -= amount.0;
	}
	
	pub fn weapon(&self) -> Option<&Weapon> {
		self.weapons.get(self.selected_weapon)
	}
	
	pub fn select_next_weapon(&mut self) {
		self.selected_weapon = (self.selected_weapon + 1 ) % self.weapons.len();
	}
	pub fn select_previous_weapon(&mut self) {
		self.selected_weapon = (self.selected_weapon - 1).min(self.weapons.len() - 1);
	}
	
	pub fn range(&self) -> Distance {
		self.weapon().map(Weapon::get_range).unwrap_or(Distance(0))
	}
	
	pub fn create_creature(typ: CreatureType, pos: Pos) -> Self{
		match typ {
			CreatureType::Player => Self::new_player(PlayerId("".to_string()), Sprite::new("player_g:X"), pos, true), // will probably commite suicide immediately
			CreatureType::Zombie => Self::new_zombie(pos),
			CreatureType::Ymp => Self::new_ymp(pos),
			CreatureType::Worm => Self::new_worm(pos),
			CreatureType::Troll => Self::new_troll(pos),
			CreatureType::Pillar => Self::new_pillar(pos),
			CreatureType::Xiangliu => Self::new_xiangliu(pos),
			CreatureType::Vargr => Self::new_vargr(pos)
		}
	}
	
	pub fn new_player(playerid: PlayerId, sprite: Sprite, pos: Pos, pvp: bool) -> Self {
		Self {
			mind: Mind::Player(playerid.clone()),
			pos,
			dir: Direction::North,
			health: Health(1),
			max_health: Health(100),
			cooldown: Duration(0),
			walk_cooldown: Duration(0),
			sprite,
			weapons: vec![
				Weapon::shotgun(),
				Weapon::rifle(),
				Weapon::smg(),
			],
			selected_weapon: 0,
			alignment: 
				if pvp {
					Alignment::Player(playerid)
				} else {
					Alignment::Players
				},
			is_building: false
		}
	}
	
	pub fn new_pillar(pos: Pos) -> Self {
		Self {
			mind: Mind::Pillar,
			pos,
			dir: Direction::North,
			health: Health(200),
			max_health: Health(200),
			cooldown: Duration(1),
			walk_cooldown: Duration(1),
			sprite: Sprite::new("pillar"),
			weapons: vec![],
			selected_weapon: 0,
			alignment: Alignment::Players,
			is_building: true
		}
	}
	
	pub fn new_zombie(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::BloodThirst(Percentage(0)),
			Health(20),
			Duration(2),
			Sprite::new("zombie"),
			Weapon::bite(Health(10), Duration(2))
		)
	}
	
	pub fn new_ymp(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::BloodThirst(Percentage(0)),
			Health(20),
			Duration(2),
			Sprite::new("ymp"),
			Weapon::cast(Health(10), Distance(30), Duration(2))
		)
	}
	
	pub fn new_troll(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::Destroyer,
			Health(100),
			Duration(4),
			Sprite::new("troll"),
			Weapon::cast(Health(50), Distance(2), Duration(4))
		)
	}
	
	pub fn new_worm(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::Destroyer,
			Health(12),
			Duration(3),
			Sprite::new("worm"),
			Weapon::cast(Health(10), Distance(2), Duration(3))
		)
	}
	
	fn new_xiangliu(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::BloodThirst(Percentage(0)),
			Health(50),
			Duration(1),
			Sprite::new("xiangliu"),
			Weapon::spit(Health(10), Distance(16), 9, Percentage(60), Duration(4))
		)
	}
	
	fn new_vargr(pos: Pos) -> Self {
		Self::new_monster(
			pos,
			Mind::BloodThirst(Percentage(20)),
			Health(50),
			Duration(0),
			Sprite::new("vargr"),
			Weapon::bite(Health(30), Duration(10))
		)
	}
	
	fn new_monster(pos: Pos, mind: Mind, health: Health, cooldown: Duration, sprite: Sprite, weapon: Weapon) -> Self {
		Self {
			mind,
			pos,
			dir: Direction::North,
			health,
			max_health: health,
			cooldown: Duration(thread_rng().gen_range(0..=cooldown.0)),
			walk_cooldown: cooldown,
			sprite,
			weapons: vec![weapon],
			selected_weapon: 0,
			alignment: Alignment::Monsters,
			is_building: false
		}
	}
}
