
use std::collections::HashMap;

use crate::{
	PlayerId,
	controls::Control,
	Result,
	aerr,
	Pos,
	Direction,
	sprite::Sprite,
	worldmessages::{WorldMessage, FieldMessage},
	Timestamp,
	util::randomize
};


const MAX_PLAYER_HEALTH: i64 = 100;


#[derive(Debug, Clone)]
pub struct Player {
	pub plan: Option<Control>,
	pub pos: Pos,
	pub dir: Direction,
	pub health: i64,
	pub sprite: Sprite
}

impl Player {
	pub fn new(pos: Pos, sprite: Sprite) -> Self {
		Self {
			plan: None,
			pos,
			dir: Direction::North,
			health: 1,
			sprite
		}
	}
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

#[derive(Debug, Clone)]
pub struct Bullet{
	direction: Direction,
	pos: Pos,
	range: i64,
	damage: i64
}

impl Bullet {
	fn sprite(&self) -> Sprite{
		Sprite("bullet".to_string())
	}
}

#[derive(Debug, Clone)]
pub struct Zombie{
	pos: Pos,
	health: i64,
	cooldown: i64
}

impl Zombie {
	fn sprite(&self) -> Sprite{
		Sprite("zombie".to_string())
	}
}

pub struct World {
	pub time: Timestamp,
	size: (i64, i64),
	ground: HashMap<Pos, Tile>,
	players: HashMap<PlayerId, Player>,
	bullets: Vec<Bullet>,
	monsters: Vec<Zombie>,
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
			monsters: Vec::new(),
			bullets: Vec::new(),
			time: Timestamp(0),
			monsterspawn: vec![Pos::new(0,0), Pos::new(size.0-1, 0), Pos::new(0, size.1-1), Pos::new(size.0-1, size.1-1)]
		}
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId, sprite: Sprite) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		self.players.insert(playerid.clone(), Player::new(self.spawnpoint, sprite));
		Ok(())
	}
	
	pub fn remove_player(&mut self, playerid: &PlayerId) -> Result<()> {
		self.players.remove(playerid).ok_or(aerr!("player {} not found", playerid))?;
		Ok(())
	}
	
	
	pub fn control_player(&mut self, playerid: PlayerId, control: Control) -> Result<()>{
		let player = self.players.get_mut(&playerid).ok_or(aerr!("player not found"))?;
		player.plan = Some(control);
		Ok(())
	}
	
	
	fn update_players(&mut self) {
		let dead: Vec<PlayerId> = self.players.iter()
			.filter_map(|(playerid, player)|
				if player.health <= 0 {
					Some(playerid.clone())
				} else {None}
			)
			.collect();
		for playerid in dead {
			let sprite = self.players.get(&playerid).unwrap().sprite.clone();
			self.players.insert(playerid, Player::new(self.spawnpoint, sprite));
		};
		for player in self.players.values_mut() {
			if self.ground.get(&player.pos) == Some(&Tile::Sanctuary) {
				player.health += 4;
				if player.health > MAX_PLAYER_HEALTH {
					player.health = MAX_PLAYER_HEALTH;
				}
			}
			if let Some(plan) = &player.plan{
				match plan {
					Control::Move(direction) => {
						player.dir = *direction;
						let newpos = player.pos + *direction;
						if let Some(tile) = self.ground.get(&newpos) {
							if !tile.blocking() || tile == &Tile::Gate && self.ground.get(&player.pos) == Some(&Tile::Sanctuary){
								player.pos = newpos;
							}
						}
					}
					Control::Shoot(dir) => {
						if let Some(direction) = dir {
							player.dir = *direction;
						}
						if !self.ground.get(&player.pos).unwrap().blocking(){
							self.bullets.push(Bullet{direction: player.dir, pos: player.pos, range: 32, damage: 10});
						}
					}
				}
				player.plan = None;
			}
		}
	}
	
	fn update_bullets(&mut self) {
		let players: HashMap<Pos, PlayerId> = self.players.iter().map(|(playerid, player)| (player.pos, playerid.clone())).collect();
		let monsters: HashMap<Pos, usize> = self.monsters.iter().enumerate().map(|(monsterid, monster)| (monster.pos, monsterid)).collect();
		self.bullets = self.bullets.clone().into_iter().filter_map(|mut bullet| {
			for _i in 0..2 {
				if bullet.range == 0 {
					return None;
				}
				bullet.pos = bullet.pos + bullet.direction;
				if let Some(playerid) = players.get(&bullet.pos){
					self.players.get_mut(playerid).unwrap().health -= bullet.damage;
					return None;
				}
				if let Some(monsterid) = monsters.get(&bullet.pos){
					self.monsters[*monsterid].health -= bullet.damage;
					if self.monsters[*monsterid].health <= 0 {
						self.monsters.remove(*monsterid);
					}
					return None;
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
	
	fn update_monsters(&mut self){
		for monster in self.monsters.iter_mut() {
			if monster.cooldown > 0 {
				monster.cooldown -= 1;
				continue;
			}
			monster.cooldown = 2;
			let mut target = None;
			for player in self.players.values() {
				if let Some(target_pos) = target {
					if monster.pos.distance_to(player.pos) < monster.pos.distance_to(target_pos) {
						target = Some(player.pos)
					}
				} else {
					target = Some(player.pos)
				}
			}
			if let Some(target_pos) = target {
				let mut dirs = monster.pos.directions_to(target_pos);
				if monster.pos.distance_to(target_pos) == 1 {
					let dir = dirs[0];
					self.bullets.push(Bullet{direction: dir, pos: monster.pos, range: 1, damage: 10});
				} else {
					if dirs.len() == 0 {
						dirs = vec![Direction::North, Direction::South, Direction::East, Direction::West];
					}
					for dir in dirs{
						let newpos = monster.pos + dir;
						if let Some(tile) = self.ground.get(&newpos) {
							if !tile.blocking() {
								monster.pos = newpos;
								break;
							}
						}
					}
				}
			}
		}
	}
	
	pub fn update(&mut self) {
		self.update_bullets();
		self.update_players();
		if self.time.0 % 3 == 0 {
			self.update_monsters();
		}
		if self.time.0 % 10 == 0 && self.monsters.len() < 4 {
			self.monsters.push(Zombie {
				pos: self.monsterspawn[randomize(self.time.0 as u32) as usize % self.monsterspawn.len()],
				health: 20,
				cooldown: 1
			})
		}
		self.time.0 += 1;
	}
	
	
	fn draw(&self) -> FieldMessage {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = self.ground.iter()
			.map(|(pos, tile)| (*pos, vec![tile.sprite()]))
			.collect();
		for monster in self.monsters.iter() {
			sprites.entry(monster.pos).or_insert(Vec::new()).insert(0, monster.sprite());
		}
		for bullet in self.bullets.iter() {
			sprites.entry(bullet.pos).or_insert(Vec::new()).insert(0, bullet.sprite());
			sprites.entry(bullet.pos + bullet.direction).or_insert(Vec::new()).insert(0, bullet.sprite());
		}
		for player in self.players.values() {
			sprites.entry(player.pos).or_insert(Vec::new()).insert(0, player.sprite.clone());
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
			wm.pos = Some(player.pos);
			wm.health = Some((player.health, MAX_PLAYER_HEALTH));
			views.insert(playerid.clone(), wm);
		}
		views
	}
}

