
use std::collections::{HashMap, VecDeque};
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
	worldmessages::{WorldMessage, FieldMessage, ChangeMessage},
	timestamp::{Timestamp, Duration},
	creature::{Creature, Mind, CreatureType, Alignment, Health},
	tile::Tile,
	weapon::Bullet,
	item::Item,
	player::Player,
	waves::wave_composition,
	gamemode::GameMode,
	mapgen::{MapTemplate, MapType, create_map},
	grid::Grid,
	pos::Distance,
	util::Percentage
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RoundState {
	Running,
	GameOver(Duration),
	Paused(Duration)
}

impl RoundState {	
	fn is_paused(&self) -> bool {
		matches!(self, Self::Paused(_))
	}
}

pub struct World {
	time: Timestamp,
	size: Pos,
	ground: Grid<Tile>,
	players: HashMap<PlayerId, Player>,
	creatures: Holder<Creature>,
	bullets: Vec<Bullet>,
	particles: HashMap<Pos, Sprite>,
	spawnpoint: Pos,
	monsterspawn: Vec<Pos>,
	items: HashMap<Pos, Item>,
	wave: usize,
	to_spawn: Vec<CreatureType>,
	round_state: RoundState,
	gamemode: GameMode,
	map: MapType,
	building_distances: Grid<Option<usize>>,
	player_distances: Grid<Option<usize>>,
	drawing: Option<HashMap<Pos, Vec<Sprite>>>,
}

impl World {
	
	pub fn new(gamemode: GameMode, map: MapType) -> Self {
		
		let mut world = World {
			size: Pos::new(0, 0),
			spawnpoint: Pos::new(0, 0),
			ground: Grid::empty(),
			players: HashMap::new(),
			creatures: Holder::new(),
			bullets: Vec::new(),
			time: Timestamp(0),
			particles: HashMap::new(),
			monsterspawn: Vec::new(),
			items: HashMap::new(),
			wave: 0,
			to_spawn: Vec::new(),
			round_state: RoundState::Running,
			gamemode,
			map,
			building_distances: Grid::empty(),
			player_distances: Grid::empty(),
			drawing: None,
		};
		world.reset();
		world
	}
	
	pub fn reset(&mut self) {
		self.creatures.clear();
		self.bullets.clear();
		self.particles.clear();
		self.items.clear();
		self.wave = 0;
		self.to_spawn.clear();
		self.round_state = RoundState::Running;
		let template: MapTemplate = create_map(&self.map, self.gamemode);
		self.size = template.size;
		self.ground = template.ground;
		self.spawnpoint = template.spawnpoint;
		self.monsterspawn = template.monsterspawn;
		for (pos, creature) in template.creatures {
			self.creatures.insert(Creature::create_creature(creature, pos));
		}
		self.drawing = None;
		for player in self.players.values_mut() {
			player.is_new = true;
		}
		self.compute_building_distances();
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId, sprite: Sprite) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		self.players.insert(
			playerid.clone(),
			Player{
				plan: None,
				sprite: sprite,
				body: 0,
				is_new: true
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
	
	fn compute_player_distances(&mut self) {
		self.player_distances = self.distance_map(
			&self.creatures.values()
				.filter(|c| !c.is_building && c.alignment != Alignment::Monsters)
				.map(|c| c.pos)
				.collect::<Vec<Pos>>()
		);
	}
	fn compute_building_distances(&mut self) {
		self.building_distances = self.distance_map(
			&self.creatures.values()
				.filter(|c| c.is_building && c.alignment != Alignment::Monsters)
				.map(|c| c.pos)
				.collect::<Vec<Pos>>()
		);
	}
	
	fn distance_map(&self, targets: &[Pos]) -> Grid<Option<usize>>{
		let mut frontier: VecDeque<(Pos, usize)> = targets.iter().map(|pos| (*pos, 0)).collect();
		let mut known: Grid<Option<usize>> = Grid::new(self.size, None);
		while let Some((pos, cost)) = frontier.pop_front() {
			if known.get_unchecked(pos).is_some(){
				continue;
			}
			known.set_unchecked(pos, Some(cost));
			for dir in &Direction::DIRECTIONS {
				if let Some(tile) = self.ground.get(pos + *dir) {
					if !tile.blocking(){
						frontier.push_back((pos + *dir, cost + 1));
					}
				}
			}
		}
		known
	}
	
	fn monster_plan<F>(&self, creature: &Creature, distance_map: &Grid<Option<usize>>, is_target: F, deviation: &Percentage) -> Option<Control>
			where F: Fn(&Creature) -> bool {
		// find nearest attackable target
		let mut target = None;
		for player in self.creatures.values() {
			if is_target(player) {
				if let Some(target_pos) = target {
					if creature.pos.distance_to(player.pos) < creature.pos.distance_to(target_pos) {
						target = Some(player.pos);
					}
				} else {
					target = Some(player.pos);
				}
			}
		}
		if let Some(target_pos) = target {
			let range = creature.range();
			let distance = creature.pos.distance_to(target_pos);
			if range <= Distance(5) && distance <= range
					|| distance.0 * 11 <= range.0 * 10 {
				return Some(Control::ShootPrecise(target_pos - creature.pos))
			}
		}
		let mut dirs: Vec<Direction> = Direction::DIRECTIONS.iter()
			.filter_map(|dir| {
				let newpos = creature.pos + *dir;
				if let Some(tile) = self.ground.get(newpos) {
					if !tile.blocking() {
						return Some(*dir);
					}
				}
				None
			})
			.collect();
		let mut rng = thread_rng();
		dirs.shuffle(&mut rng);
		if rng.gen_range(0..100) >= deviation.0 {
			dirs.sort_by_key(|dir| distance_map.get(creature.pos + *dir).unwrap_or(&None).unwrap_or(std::usize::MAX));
		}
		Some(Control::Move(*dirs.first()?))
	}
	
	fn creature_plan(&self, creature: &Creature) -> Option<Control> {
		match &creature.mind {
			Mind::Player(playerid) => {
				if let Some(player) = self.players.get(&playerid) {
					player.plan.clone()
				} else {Some(Control::Suicide)}
			}
			Mind::BloodThirst(deviation) => {
				self.monster_plan(
					creature,
					&self.player_distances,
					|player| 
						player.alignment != creature.alignment 
						&& !player.is_building 
						&& self.ground.get(player.pos) != Some(&Tile::Sanctuary),
					deviation
				)
			}
			Mind::Destroyer => {
				self.monster_plan(
					creature,
					&self.building_distances,
					|player| 
						player.alignment != creature.alignment 
						&& player.is_building,
					&Percentage(0)
				)
			}
			Mind::Pillar => None
		}
	}
	
	fn update_creatures(&mut self) {
		let mut creature_map: HashMap<Pos, usize> = self.creatures.iter()
			.map(|(creatureid, creature)| (creature.pos, *creatureid))
			.collect();
		self.compute_player_distances();
		let plans: HashMap<usize, Control> = self.creatures.iter()
			.filter(|(_k, c)| c.cooldown.0 <= 0)
			.filter_map(|(k, c)|
				Some((*k, self.creature_plan(c)?))
			).collect();
		for (id, creature) in self.creatures.iter_mut() {
			if creature.is_dead() {
				continue;
			}
			if self.ground.get(creature.pos) == Some(&Tile::Sanctuary) {
				creature.heal(Health(2));
			} else if self.round_state.is_paused() {
				creature.heal(Health(if creature.is_building {20} else {2}));
			}
			if creature.cooldown.0 > 0 {
				creature.cooldown.0 -= 1;
				continue;
			}
			match plans.get(id) {
				Some(Control::Move(direction)) => {
					creature.cooldown = creature.walk_cooldown;
					creature.dir = *direction;
					let newpos = creature.pos + *direction;
					if let Some(tile) = self.ground.get(newpos) {
						if (
								!tile.blocking()
									|| tile == &Tile::Gate
										&& self.ground.get(creature.pos) == Some(&Tile::Sanctuary)
										&& creature.has_full_health())
								&& !creature_map.contains_key(&newpos) {
							if creature_map.get(&creature.pos) == Some(id){
								creature_map.remove(&creature.pos);
							}
							creature_map.insert(newpos, *id);
							creature.pos = newpos;
							if let Mind::Player(_) = creature.mind {
								match self.items.get(&creature.pos) {
									Some(Item::Health) => {
										creature.heal(Health(100));
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
					creature.cooldown = if let Some(weapon) = creature.weapon() {
						if !self.ground.get(creature.pos).unwrap().blocking(){
							self.bullets.append(
								&mut weapon.shoot(
									creature.pos,
									creature.dir.to_position(),
									creature.alignment.clone()
								)
							);
						}
						weapon.get_cooldown()
					} else { Duration(0) }
				}
				Some(Control::ShootPrecise(dirvec)) => {
					creature.cooldown = if let Some(weapon) = creature.weapon() {
						if !self.ground.get(creature.pos).unwrap().blocking(){
							self.bullets.append(
								&mut weapon.shoot(
									creature.pos,
									*dirvec,
									creature.alignment.clone()
								)
							);
						}
						weapon.get_cooldown()
					} else { Duration(0) }
				}
				Some(Control::Suicide) => {
					creature.kill();
				}
				Some(Control::NextWeapon) => {
					creature.select_next_weapon();
				}
				Some(Control::PreviousWeapon) => {
					creature.select_previous_weapon();
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
					bullet.do_move();
					if bullet.out_of_range() {
						return None;
					}
					/* draw the trail */
					self.particles.insert(bullet.pos, bullet.sprite());
				}
				/* hit creature */
				if let Some(creatureid) = creature_map.get(&bullet.pos){
					if let Some(creature) = self.creatures.get_mut(creatureid){
						if creature.alignment != bullet.alignment {
							creature.damage(bullet.ammo.damage);
							return None;
						}
					}
				}
				/* hit static geometry */
				if let Some(tile) = self.ground.get(bullet.pos) {
					if tile.bullet_blocking(){
						return None;
					}
				}
			}
			Some(bullet)
		}).collect();
	}
	
	fn spawn(&mut self, dead_creatures: Vec<Creature>){
		
		// spawn players
		for (playerid, player) in self.players.iter_mut() {
			if !self.creatures.contains_key(&player.body) {
				let body = self.creatures.insert(Creature::new_player(
					playerid.clone(),
					player.sprite,
					self.spawnpoint,
					self.gamemode == GameMode::PvP
				));
				player.body = body
			}
			player.plan = None;
		}
		
		// spawn monsters
		let nmonsters = self.creatures.values().filter(|c| c.alignment == Alignment::Monsters).count();
		if self.gamemode != GameMode::PvP && nmonsters == 0 && self.to_spawn.is_empty() {
			self.wave += 1;
			self.round_state = RoundState::Paused(Duration(25));
			self.to_spawn =
				wave_composition(self.wave)
					.into_iter()
					.flat_map(|typ| self.spawn_modify(typ))
					.collect();
		}
		if let RoundState::Paused(pause) = self.round_state {
			
			self.round_state = if pause.0 <= 0 {
				RoundState::Running
			} else {
				RoundState::Paused(pause - Duration(1))
			};
		} else if self.time.0 % 5 == 0 && !self.to_spawn.is_empty() {
			self.creatures.insert(Creature::create_creature(
				self.to_spawn.remove(0),
				self.monsterspawn[thread_rng().gen_range(0..self.monsterspawn.len())],
			));
		}
		
		let nplayers = std::cmp::max(self.players.len(), 1);
		// spawn items
		for creature in dead_creatures {
			if creature.alignment != Alignment::Players && self.items.len() < nplayers + 1  && thread_rng().gen_range(0..10) == 0{
				self.items.insert(creature.pos, Item::Health);
			}
		}
	}
	
	fn spawn_modify(&self, stored: CreatureType) -> Vec<CreatureType> {
		if self.gamemode == GameMode::Survival {
			match stored {
				CreatureType::Worm => vec![CreatureType::Zombie, CreatureType::Zombie, CreatureType::Zombie],
				CreatureType::Troll => vec![CreatureType::Ymp, CreatureType::Zombie, CreatureType::Zombie],
				other => vec![other]
			}
		} else {
			vec![stored]
		}
	}
	
	pub fn update(&mut self) {
		match self.round_state {
			RoundState::Running | RoundState::Paused(_) => {
				self.particles.clear();
				self.update_creatures();
				self.update_bullets();
				let mut dead_creatures = Vec::new();
				let creatureids: Vec<usize> = self.creatures.keys().cloned().collect();
				for creatureid in creatureids {
					if self.creatures.get(&creatureid).unwrap().is_dead() {
						dead_creatures.push(self.creatures.remove(&creatureid).unwrap());
					}
				}
				if dead_creatures.iter().any(|c|c.is_building && c.alignment == Alignment::Players){
					self.compute_building_distances();
				}
				self.spawn(dead_creatures);
				
				if self.is_game_over() {
					self.round_state = RoundState::GameOver(Duration(50));
				}
				self.time.increment();
			}
			
			
			RoundState::GameOver(time_left) => {
				let mut rng = thread_rng();
				let gopos = Pos::new(rng.gen_range(0..(self.size.x - 10)), rng.gen_range(0..self.size.y));
				for (i, c) in "GAME_OVER!".chars().enumerate() {
					self.particles.insert(Pos::new(gopos.x + (i as i64), gopos.y), Sprite::letter_sprite(c).unwrap());
				}
				if time_left.0 <= 0 {
					self.reset();
				} else {
					self.round_state = RoundState::GameOver(time_left - Duration(1));
				}
			}
		}
	}
	
	fn is_game_over(&self) -> bool {
		match self.gamemode {
			GameMode::PillarDefence =>
				!self.creatures.values()
					.any(|c| c.mind == Mind::Pillar && c.alignment == Alignment::Players),
			GameMode::Survival =>
				self.wave > 1 && !self.creatures.values()
					.any(|c| c.is_player() && self.ground.get(c.pos) != Some(&Tile::Sanctuary)),
			GameMode::PvP => false
		}
	}
	
	
	fn draw_dynamic(&self) -> HashMap<Pos, Vec<Sprite>> {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = HashMap::new();
		for (pos, sprite) in self.particles.iter() {
			sprites.insert(*pos, vec![*sprite]);
		}
		for creature in self.creatures.values() {
			sprites.entry(creature.pos).or_insert_with(Vec::new).push(creature.sprite);
		}
		for (pos, item) in self.items.iter() {
			sprites.entry(*pos).or_insert_with(Vec::new).push(item.sprite());
		}
		sprites.into_iter().filter_map(|(pos, mut sprs)| {
			sprs.push(self.ground.get(pos)?.sprite());
			Some((pos, sprs))
		}).collect()
	}
	
	fn draw_changes(&self, mut sprites: HashMap<Pos, Vec<Sprite>>) -> Option<ChangeMessage> { 
		if let Some(last_drawing) = &self.drawing {
			for pos in last_drawing.keys() {
				sprites.entry(*pos).or_insert(vec![self.ground.get(*pos)?.sprite()]);
			}
			let sprs: ChangeMessage = sprites.iter()
				.filter(|(pos, spritelist)| last_drawing.get(pos) != Some(spritelist))
				.map(|(pos, spritelist)| (*pos, spritelist.clone()))
				.collect();
			Some(sprs)
		} else {None}
	}
	
	pub fn view(&mut self) -> HashMap<PlayerId, WorldMessage> {
		let dynamic_sprites = self.draw_dynamic();
		let changes = self.draw_changes(dynamic_sprites.clone());
		let mut field = None;
		let mut views: HashMap<PlayerId, WorldMessage> = HashMap::new();
		for (playerid, player) in self.players.iter_mut() {
			let mut wm = WorldMessage::default();
			if changes.is_some() && !player.is_new {
				wm.change = changes.clone();
			} else {
				if field.is_none(){
					field = Some(draw_field(self.size, &self.ground, &dynamic_sprites));
				}
				wm.field = Some(field.clone().unwrap());
				player.is_new = false;
			}
			if let Some(body) = self.creatures.get(&player.body){
				wm.pos = Some(body.pos);
				wm.health = Some((body.health, body.max_health));
				wm.weapons = Some((
					body.weapons.iter()
						.map(|weapon| weapon.name)
						.collect::<Vec<&'static str>>(),
					body.selected_weapon
				))
			}
			if self.round_state == RoundState::GameOver(Duration(1)) {
				wm.sounds = Some(vec![("restart".to_string(), "---- Starting new session ----".to_string(), None)]);
			} else if self.round_state == RoundState::Paused(Duration(1)) {
				wm.sounds = Some(vec![("wave".to_string(), format!("**** Wave {} ****", self.wave), None)]);
			}
			views.insert(playerid.clone(), wm);
		}
		self.drawing = Some(dynamic_sprites);
		views
	}
	
	pub fn nplayers(&self) -> usize {
		self.players.len()
	}
}


fn draw_field(size: Pos, tiles: &Grid<Tile>, sprites: &HashMap<Pos, Vec<Sprite>>) -> FieldMessage {
	println!("redrawing field");
	let mut values :Vec<usize> = Vec::with_capacity((size.x * size.y) as usize);
	let mut mapping: Vec<Vec<Sprite>> = Vec::new();
	for y in 0..size.y {
		for x in 0..size.x {
			let tilesprite = vec![tiles.get_unchecked(Pos::new(x, y)).sprite()];
			let sprs: &Vec<Sprite> = sprites.get(&Pos{x, y}).unwrap_or(&tilesprite);
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
		width: size.x,
		height: size.y,
		field: values,
		mapping
	}
}

