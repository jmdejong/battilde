
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
	util::{randomize},
	creature::{Creature, Mind, MonsterType, Alignment},
	tile::{Tile, FloorType, WallType},
	bullet::Bullet,
	item::Item,
	player::Player,
	waves::wave_composition,
};



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
	items: HashMap<Pos, Item>,
	wave: usize,
	to_spawn: Vec<MonsterType>,
	pause: i64,
	gameover: i64,
}

impl World {

	fn generate_map(&mut self) {
		for x in 0..self.size.0 {
			for y in 0..self.size.1 {
				let dspawn = (Pos::new(x, y) - self.spawnpoint).abs();
				let floor = if dspawn.x <= 3 && dspawn.y <= 3 {
					Tile::Sanctuary
				} else if dspawn.x <= 4 && dspawn.y <= 4 && dspawn.x != dspawn.y{
					Tile::Gate
				} else if dspawn.x <= 1 || dspawn.y <= 1 {
					Tile::Floor(FloorType::Dirt)
				} else {
					Tile::Floor([FloorType::Grass1, FloorType::Grass2, FloorType::Grass3][randomize(x as u32 + randomize(y as u32)) as usize % 3])
				};
				self.ground.insert(Pos::new(x, y), floor);
			}
		}
		let d: Vec<(i64, i64)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
		let p: Vec<(i64, i64)> = vec![(3, 3), (4, 3), (4, 2), (3, 4), (2, 4)];
		for (dx, dy) in d {
			for (px, py) in p.iter() {
				self.ground.insert(self.spawnpoint + Pos::new(px * dx, py * dy), Tile::Wall(WallType::Wall));
			}
			self.ground.insert(self.spawnpoint + Pos::new(4 * dx, 4 * dy), Tile::Wall(WallType::Rubble));
			self.creatures.insert(Creature::new_pillar(self.spawnpoint + Pos::new(4*dx, 4*dy)));
		} 
	}
	
	pub fn new() -> Self {
		
		let size = (64, 64);
		
		let mut world = World {
			size,
			spawnpoint: Pos::new(size.0 / 2, size.1 / 2),
			ground: HashMap::new(),
			players: HashMap::new(),
			creatures: Holder::new(),
			bullets: Vec::new(),
			time: Timestamp(0),
			particles: HashMap::new(),
			monsterspawn: vec![Pos::new(0,0), Pos::new(size.0-1, 0), Pos::new(0, size.1-1), Pos::new(size.0-1, size.1-1)],
			items: HashMap::new(),
			wave: 0,
			to_spawn: Vec::new(),
			pause: 0,
			gameover: 0
		};
		world.generate_map();
		world
	}
	
	pub fn reset(&mut self) {
		self.creatures.clear();
		self.bullets.clear();
		self.particles.clear();
		self.items.clear();
		self.wave = 0;
		self.to_spawn.clear();
		self.pause = 0;
		self.gameover = 0;
		self.generate_map();
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
					if player.alignment != creature.alignment && player.mind != Mind::Pillar && self.ground.get(&player.pos) != Some(&Tile::Sanctuary){
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
					if creature.pos.distance_to(target_pos) <= creature.ammo.range && dirs.len() > 0 {
						return Some(Control::ShootPrecise(target_pos - creature.pos))
					}
				}
				let mut default_dirs = vec![Direction::North, Direction::South, Direction::East, Direction::West];
				dirs.shuffle(&mut thread_rng());
				default_dirs.shuffle(&mut thread_rng());
				dirs.append(&mut default_dirs);
				for dir in dirs{
					let newpos = creature.pos + dir;
					if let Some(tile) = self.ground.get(&newpos) {
						if !tile.blocking() {
							return Some(Control::Move(dir));
						}
					}
				}
				None
			}
			Mind::Destroyer => {
				// find nearest attackable target
				let mut target = None;
				for player in self.creatures.values() {
					if player.alignment != creature.alignment && player.mind == Mind::Pillar{
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
					if creature.pos.distance_to(target_pos) <= creature.ammo.range && dirs.len() > 0 {
						return Some(Control::ShootPrecise(target_pos - creature.pos))
					}
				}
				let mut default_dirs = vec![Direction::North, Direction::South, Direction::East, Direction::West];
				dirs.shuffle(&mut thread_rng());
				default_dirs.shuffle(&mut thread_rng());
				dirs.append(&mut default_dirs);
				for dir in dirs{
					let newpos = creature.pos + dir;
					if let Some(tile) = self.ground.get(&newpos) {
						if !tile.blocking() {
							return Some(Control::Move(dir));
						}
					}
				}
				None
			}
			Mind::Pillar => None
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
			if self.ground.get(&creature.pos) == Some(&Tile::Sanctuary) || self.pause > 0{
				creature.health += 2;
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
								!tile.blocking()
									|| tile == &Tile::Gate
										&& self.ground.get(&creature.pos) == Some(&Tile::Sanctuary)
										&& creature.health >= creature.max_health)
								&& !creature_map.contains_key(&newpos) {
							if creature_map.get(&creature.pos) == Some(id){
								creature_map.remove(&creature.pos);
							}
							creature_map.insert(newpos, *id);
							creature.pos = newpos;
							if let Mind::Player(_) = creature.mind {
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
				}
				Some(Control::Shoot(dir)) => {
					if let Some(direction) = dir {
						creature.dir = *direction;
					}
					if !self.ground.get(&creature.pos).unwrap().blocking(){
						self.bullets.push(Bullet{
							direction: creature.dir.to_position(),
							pos: creature.pos,
							alignment: creature.alignment.clone(),
							ammo: creature.ammo.clone(),
							steps: Pos::new(0, 0)
						});
					}
				}
				Some(Control::ShootPrecise(dirvec)) => {
					if !self.ground.get(&creature.pos).unwrap().blocking(){
						self.bullets.push(Bullet{
							direction: *dirvec,
							pos: creature.pos,
							alignment: creature.alignment.clone(),
							ammo: creature.ammo.clone(),
							steps: Pos::new(0, 0)
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
				/* bullet movement */
				if i != 0 {
					if bullet.ammo.range == 0 {
						return None;
					}
					bullet.ammo.range -= 1;
					bullet.movement();
					/* draw the trail */
					self.particles.insert(bullet.pos, bullet.sprite());
				}
				/* hit creature */
				if let Some(creatureid) = creature_map.get(&bullet.pos){
					if let Some(creature) = self.creatures.get_mut(creatureid){
						if creature.alignment != bullet.alignment {
							creature.health -= bullet.ammo.damage;
							return None;
						}
					}
				}
				/* hit static geometry */
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
		let nmonsters = self.creatures.values().filter(|c| c.alignment == Alignment::Monsters).count();
		let nplayers = std::cmp::max(self.players.len(), 1);
		if nmonsters == 0 && self.to_spawn.is_empty() {
			self.wave += 1;
			self.pause = 25;
			self.to_spawn = wave_composition(self.wave, nplayers);
		}
		if self.pause > 0 {
			self.pause -= 1;
		} else if self.time.0 % 5 == 0 && !self.to_spawn.is_empty() {
			self.creatures.insert(Creature::create_monster(
				self.to_spawn.remove(0),
				self.monsterspawn[thread_rng().gen_range(0..self.monsterspawn.len())],
			));
		}
		
		// spawn items
		for pos in dead_positions {
			if self.items.len() < nplayers + 1  && thread_rng().gen_range(0..10) == 0{
				self.items.insert(pos, Item::Health);
			}
		}
	}
	
	pub fn update(&mut self) {
		if self.gameover > 0 {
			let mut rng = thread_rng();
			let gopos = Pos::new(rng.gen_range(0..(self.size.0 - 10)), rng.gen_range(0..self.size.1));
			for (i, c) in "GAME_OVER!".chars().enumerate() {
				let mut spritename = "emptyletter-".to_string();
				spritename.push(c);
				self.particles.insert(Pos::new(gopos.x + (i as i64), gopos.y), Sprite(spritename));
			}
			self.gameover -= 1;
			if self.gameover == 0 {
				self.reset();
			}
			return
		}
		self.particles.clear();
		self.update_creatures();
		self.update_bullets();
		let mut dead_positions = Vec::new();
		self.creatures.retain(|_id, creature| if creature.health <= 0 {
			if creature.alignment == Alignment::Monsters {
				dead_positions.push(creature.pos);
			}
			false
		} else {true});
		self.spawn(dead_positions);
		
		if self.creatures.values().filter(|c| c.mind == Mind::Pillar && c.alignment == Alignment::Players).count() == 0 {
			self.gameover = 50;
		}
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

