
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
	util::randomize,
	creature::{Creature, Mind},
	tile::Tile,
	bullet::Bullet,
	tile::FloorType
};


#[derive(Debug, Clone)]
pub struct Player {
	pub plan: Option<Control>,
	pub sprite: Sprite,
	pub body: usize
}

#[derive(Debug, Clone)]
pub enum Item {
	Health
}

impl Item {
	pub fn sprite(&self) -> Sprite {
		Sprite(match self {
			Self::Health => "health"
		}.to_string())
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
	items: HashMap<Pos, Item>
}

impl World {
	
	pub fn new() -> Self {
		
		let size = (64, 64);
		let spawnpoint = Pos::new(size.0 / 2, size.1 / 2);
		let mut ground = HashMap::new();
		for x in 0..size.0 {
			for y in 0..size.1 {
				let dspawn = (Pos::new(x, y) - spawnpoint).abs();
				let floor = if dspawn.x <= 3 && dspawn.y <= 3 {
					Tile::Sanctuary
				} else if dspawn.x <= 4 && dspawn.y <= 4 && dspawn.x != dspawn.y{
					Tile::Gate
				} else if dspawn.x <= 1 || dspawn.y <= 1 {
					Tile::Floor(FloorType::Dirt)
				} else {
					Tile::Floor([FloorType::Grass1, FloorType::Grass2, FloorType::Grass3][randomize(x as u32 + randomize(y as u32)) as usize % 3])
				};
				ground.insert(Pos::new(x, y), floor);
			}
		}
		let p: Vec<(i64, i64)> = vec![(4, 4), (3, 3), (4, 3), (4, 2)];
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
			monsterspawn: vec![Pos::new(0,0), Pos::new(size.0-1, 0), Pos::new(0, size.1-1), Pos::new(size.0-1, size.1-1)],
			items: HashMap::new()
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
				body: 0
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
					if player.alignment != creature.alignment && !(self.ground.get(&player.pos) == Some(&Tile::Sanctuary)){
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
			if creature.health <= 0 {
				continue;
			}
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
									tile == &Tile::Gate && self.ground.get(&creature.pos) == Some(&Tile::Sanctuary) && creature.health >= creature.max_health) &&
								!creature_map.contains_key(&newpos) {
							if creature_map.get(&creature.pos) == Some(id){
								creature_map.remove(&creature.pos);
							}
							creature_map.insert(newpos, *id);
							creature.pos = newpos;
							match self.items.get(&creature.pos) {
								Some(Item::Health) => {
									creature.health = creature.max_health;
									self.items.remove(&creature.pos);
								}
								None => {}
							}
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
					creature.health = -1;
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
					if bullet.ammo.aim == 0 {
						let d = bullet.direction.to_position();
						let ds = Pos::new(d.y, d.x);
						let r: u8 = thread_rng().gen_range(0..4);
						if r == 1 {
							bullet.pos = bullet.pos + ds;
						} else if r == 2 {
							bullet.pos = bullet.pos - ds;
						}
						bullet.ammo.aim = bullet.ammo.accuracy
					}
					bullet.ammo.range -= 1;
					bullet.ammo.aim -=1;
					bullet.pos = bullet.pos + bullet.direction;
					self.particles.insert(bullet.pos, bullet.ammo.sprite.clone());
				}
				if let Some(creatureid) = creature_map.get(&bullet.pos){
					if let Some(creature) = self.creatures.get_mut(creatureid){
						if creature.alignment != bullet.alignment {
							creature.health -= bullet.ammo.damage;
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
	
	fn spawn(&mut self, dead_positions: Vec<Pos>){
		
		// spawn players
		for (playerid, player) in self.players.iter_mut() {
			if !self.creatures.contains_key(&player.body) {
				let body = self.creatures.insert(Creature::new_player(playerid.clone(), player.sprite.clone(), self.spawnpoint));
				player.body = body
			}
			player.plan = None;
		}
		
		// spawn monsters
		let nmonsters = self.creatures.len() - self.players.len();
		if self.time.0 % 10 == 0 && nmonsters < 6 + 2*self.players.len() {
			self.creatures.insert(Creature::new_zombie(
				self.monsterspawn[thread_rng().gen_range(0..self.monsterspawn.len())],
			));
		}
		
		// spawn items
		for pos in dead_positions {
			if self.items.len() < 2  && thread_rng().gen_range(0..10) == 0{
				self.items.insert(pos, Item::Health);
			}
		}
	}
	
	pub fn update(&mut self) {
		self.particles.clear();
		self.update_creatures();
		self.update_bullets();
		let mut dead_positions = Vec::new();
		self.creatures.retain(|_id, creature| if creature.health < 0 {dead_positions.push(creature.pos); false} else {true});
		self.spawn(dead_positions);
		self.time.0 += 1;
	}
	
	
	fn draw(&self) -> FieldMessage {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = self.ground.iter()
			.map(|(pos, tile)| (*pos, vec![tile.sprite()]))
			.collect();
		for (pos, item) in self.items.iter() {
			sprites.entry(*pos).or_insert(Vec::new()).insert(0, item.sprite());
		}
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

