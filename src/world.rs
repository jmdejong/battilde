
use std::collections::HashMap;
use rand::{Rng, thread_rng, seq::SliceRandom};

use crate::{
	PlayerId,
	controls::Control,
	Result,
	aerr,
	Pos,
	Direction,
	holder::Holder,
	sprite::Sprite,
	worldmessages::{WorldMessage, FieldMessage},
	Timestamp,
	util::randomize
};


#[derive(Debug, Clone)]
pub struct Player {
	pub plan: Option<Control>,
	pub sprite: Sprite,
	pub body: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mind {
	Player(PlayerId),
	Zombie
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
	#[allow(dead_code)]
	Players,
	Player(PlayerId),
	Monsters
}

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
	pub ammo: Ammo
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
				speed: 2,
				sprite: Sprite("bullet".to_string()),
				aim: 10,
				spread: 1
			},
			alignment: Alignment::Player(playerid)
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
				sprite: Sprite("bite".to_string()),
				aim: 10,
				spread: 10
			},
			alignment: Alignment::Monsters
		}
	}
}


#[derive(Debug, Clone)]
pub struct Bullet{
	direction: Direction,
	pos: Pos,
	alignment: Alignment,
	ammo: Ammo
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorType{
	#[allow(dead_code)]
	Stone,
	Dirt,
	Grass1,
	Grass2,
	Grass3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
	Floor(FloorType),
	Sanctuary,
	Gate,
	Wall
}

impl Tile {
	fn sprite(&self) -> Sprite{
		Sprite(match self {
			Tile::Floor(FloorType::Stone) => "floor",
			Tile::Floor(FloorType::Dirt) => "ground",
			Tile::Floor(FloorType::Grass1) => "grass1",
			Tile::Floor(FloorType::Grass2) => "grass2",
			Tile::Floor(FloorType::Grass3) => "grass3",
			Tile::Gate => "gate",
			Tile::Sanctuary => "sanctuary",
			Tile::Wall => "wall"
		}.to_string())
	}
	fn blocking(&self) -> bool {
		match self {
			Tile::Floor(_) => false,
			Tile::Sanctuary => false,
			Tile::Wall => true,
			Tile::Gate => true
		}
	}
}


pub struct World {
	pub time: Timestamp,
	size: (i64, i64),
	ground: HashMap<Pos, Tile>,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<Creature>,
	bullets: Vec<Bullet>,
	particles: HashMap<Pos, Sprite>,
	spawnpoint: Pos,
	monsterspawn: Vec<Pos>,
}

impl World {
	
	pub fn new() -> Self {
		
		let size = (64, 64);
		let spawnpoint = Pos::new(size.0 / 2, size.1 / 2);
		let mut ground = HashMap::new();
		for x in 0..size.0 {
			for y in 0..size.1 {
				let dspawn = (Pos::new(x, y) - spawnpoint).abs();
				let floor = if dspawn.x <= 4 && dspawn.y <= 4 {
					Tile::Sanctuary
				} else if dspawn.x <= 5 && dspawn.y <= 5 {
					Tile::Gate
				} else if dspawn.x <= 2 || dspawn.y <= 2 {
					Tile::Floor(FloorType::Dirt)
				} else {
					Tile::Floor([FloorType::Grass1, FloorType::Grass2, FloorType::Grass3][randomize(x as u32 + randomize(y as u32)) as usize % 3])
				};
				ground.insert(Pos::new(x, y), floor);
			}
		}
		let p: Vec<(i64, i64)> = vec![(5, 5), (5, 4), (5, 3), (5, 2), (4, 4)];
		for (x, y) in p {
			ground.insert(spawnpoint + Pos::new(x, y), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(-x, -y), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(x, -y), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(-x, y), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(y, x), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(-y, -x), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(y, -x), Tile::Wall);
			ground.insert(spawnpoint + Pos::new(-y, x), Tile::Wall);
		}
		
		World {
			size,
			spawnpoint,
			ground,
			players: HashMap::new(),
			creatures: Holder::new(),
			bullets: Vec::new(),
			time: Timestamp(0),
			particles: HashMap::new(),
			monsterspawn: vec![Pos::new(0,0), Pos::new(size.0-1, 0), Pos::new(0, size.1-1), Pos::new(size.0-1, size.1-1)]
		}
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId, sprite: Sprite) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		self.players.insert(
			playerid.clone(),
			Player{
				plan: None,
				sprite: sprite.clone(),
				body: self.creatures.insert(Creature::new_player(playerid.clone(), sprite, self.spawnpoint))
			}
		);
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		let player = self.players.remove(playerid).ok_or(aerr!("player {} not found", playerid))?;
		self.creatures.remove(&player.body);
		Ok(())
	}
	
	
	pub fn control_player(&mut self, playerid: PlayerId, control: Control) -> Result<()>{
		let player = self.players.get_mut(&playerid).ok_or(aerr!("player not found"))?;
		player.plan = Some(control);
		Ok(())
	}
	
	fn creature_plan(&self, creature: &Creature) -> Option<Control> {
		match &creature.mind {
			Mind::Player(playerid) => {
				if let Some(player) = self.players.get(&playerid) {
					player.plan.clone()
				} else {Some(Control::Suicide)}
			}
			Mind::Zombie => {
				// find nearest attackable target
				let mut target = None;
				for player in self.creatures.values() {
					if player.alignment != creature.alignment{
						if let Some(target_pos) = target {
							if creature.pos.distance_to(player.pos) < creature.pos.distance_to(target_pos) {
								target = Some(player.pos);
							}
						} else {
							target = Some(player.pos);
						}
					}
				}
				let mut dirs = Vec::new();
				if let Some(target_pos) = target {
					dirs = creature.pos.directions_to(target_pos);
					if creature.pos.distance_to(target_pos) == 1 {
						let dir = dirs[0];
						return Some(Control::Shoot(Some(dir)))
					}
				}
				if dirs.len() == 0 {
					dirs = vec![Direction::North, Direction::South, Direction::East, Direction::West];
				}
				dirs.shuffle(&mut thread_rng());
				for dir in dirs{
					let newpos = creature.pos + dir;
					if let Some(tile) = self.ground.get(&newpos) {
						if !tile.blocking() {
							return Some(Control::Move(dir));
						}
					}
				}
				return None;
			}
		}
	}
	
	fn update_creatures(&mut self) {
		let mut creature_map: HashMap<Pos, usize> = self.creatures.iter()
			.map(|(creatureid, creature)| (creature.pos, *creatureid))
			.collect();
		let plans: HashMap<usize, Control> = self.creatures.iter().filter_map(|(k, c)|Some((*k, self.creature_plan(c)?))).collect();
		for (id, creature) in self.creatures.iter_mut() {
			if self.ground.get(&creature.pos) == Some(&Tile::Sanctuary) {
				creature.health += 4;
				if creature.health > creature.max_health {
					creature.health = creature.max_health;
				}
			}
			if creature.cooldown > 0 {
				creature.cooldown -= 1;
				continue;
			}
			creature.cooldown = creature.max_cooldown;
			match plans.get(id) {
				Some(Control::Move(direction)) => {
					creature.dir = *direction;
					let newpos = creature.pos + *direction;
					if let Some(tile) = self.ground.get(&newpos) {
						if (
									!tile.blocking() ||
									tile == &Tile::Gate && self.ground.get(&creature.pos) == Some(&Tile::Sanctuary)) &&
								!creature_map.contains_key(&newpos){
							if creature_map.get(&creature.pos) == Some(id){
								creature_map.remove(&creature.pos);
							}
							creature_map.insert(newpos, *id);
							creature.pos = newpos;
						}
					}
				}
				Some(Control::Shoot(dir)) => {
					if let Some(direction) = dir {
						creature.dir = *direction;
					}
					if !self.ground.get(&creature.pos).unwrap().blocking(){
						self.bullets.push(Bullet{
							direction: creature.dir,
							pos: creature.pos,
							alignment: creature.alignment.clone(),
							ammo: creature.ammo.clone()
						});
					}
				}
				Some(Control::Suicide) => {
				
				}
				None => {}
			}
		}
	}
	
	
	fn update_bullets(&mut self) {
		let creature_map: HashMap<Pos, usize> = self.creatures.iter()
			.map(|(creatureid, creature)| (creature.pos, *creatureid))
			.collect();
		self.bullets = self.bullets.clone().into_iter().filter_map(|mut bullet| {
			for i in 0..(bullet.ammo.speed + 1) {
				if i != 0 {
					if bullet.ammo.range == 0 {
						return None;
					}
					if bullet.ammo.spread == 0 {
						let d = bullet.direction.to_position();
						let ds = Pos::new(d.y, d.x);
						let r: u8 = thread_rng().gen_range(0..4);
						if r == 1 {
							bullet.pos = bullet.pos + ds;
						} else if r == 2 {
							bullet.pos = bullet.pos - ds;
						}
						bullet.ammo.spread = bullet.ammo.aim
					}
					bullet.ammo.range -= 1;
					bullet.ammo.spread -=1;
					bullet.pos = bullet.pos + bullet.direction;
					self.particles.insert(bullet.pos, bullet.ammo.sprite.clone());
				}
				if let Some(creatureid) = creature_map.get(&bullet.pos){
					if let Some(creature) = self.creatures.get_mut(creatureid){
						if creature.alignment != bullet.alignment {
							creature.health -= bullet.ammo.damage;
							if creature.health <= 0 {
								self.creatures.remove(creatureid);
							}
							return None;
						}
					}
				}
				if let Some(tile) = self.ground.get(&bullet.pos) {
					if tile.blocking(){
						return None;
					}
				}
			}
			Some(bullet)
		}).collect();
	}
	
	pub fn update(&mut self) {
		self.particles.clear();
		self.update_bullets();
		self.update_creatures();
		if self.time.0 % 10 == 0 && self.creatures.len() - self.players.len() < 8 {
			self.creatures.insert(Creature::new_zombie(
				self.monsterspawn[randomize(self.time.0 as u32) as usize % self.monsterspawn.len()],
			));
		}
		for (playerid, player) in self.players.iter_mut() {
			if !self.creatures.contains_key(&player.body) {
				let body = self.creatures.insert(Creature::new_player(playerid.clone(), player.sprite.clone(), self.spawnpoint));
				player.body = body
			}
			player.plan = None;
		}
		self.time.0 += 1;
	}
	
	
	fn draw(&self) -> FieldMessage {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = self.ground.iter()
			.map(|(pos, tile)| (*pos, vec![tile.sprite()]))
			.collect();
		for creature in self.creatures.values() {
			sprites.entry(creature.pos).or_insert(Vec::new()).insert(0, creature.sprite.clone());
		}
		for (pos, sprite) in self.particles.iter() {
			sprites.entry(*pos).or_insert(Vec::new()).insert(0, sprite.clone());
		}
		
		let (width, height) = self.size;
		let size = width * height;
		let mut values :Vec<usize> = Vec::with_capacity(size as usize);
		let mut mapping: Vec<Vec<Sprite>> = Vec::new();
		let emptyvec = Vec::new();
		for y in 0..height {
			for x in 0..width {
				let sprs: &Vec<Sprite> = sprites.get(&Pos{x, y}).unwrap_or(&emptyvec);
				values.push(
					match mapping.iter().position(|x| x == sprs) {
						Some(index) => {
							index
						}
						None => {
							mapping.push(sprs.to_vec());
							mapping.len() - 1
						}
					}
				)
			}
		}
		
		FieldMessage {
			width: self.size.0,
			height: self.size.1,
			field: values,
			mapping
		}
	}
	
	pub fn view(&self) -> HashMap<PlayerId, WorldMessage> {
		let fm = self.draw();
		let mut views: HashMap<PlayerId, WorldMessage> = HashMap::new();
		for (playerid, player) in self.players.iter() {
			let mut wm = WorldMessage::default();
			wm.field = Some(fm.clone());
			if let Some(body) = self.creatures.get(&player.body){
				wm.pos = Some(body.pos);
				wm.health = Some((body.health, body.max_health));
				views.insert(playerid.clone(), wm);
			}
		}
		views
	}
}

