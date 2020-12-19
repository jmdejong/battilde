
use std::collections::HashMap;

use crate::{
	PlayerId,
	controls::{Control, Direction},
	Result,
	aerr,
	Pos,
	sprite::Sprite,
	worldmessages::{WorldMessage, FieldMessage},
	Timestamp,
	util::randomize
};

pub trait GameObject {
	
	fn sprite(&self) -> Option<Sprite>;
	fn blocking(&self) -> bool;
}


const MAX_HEALTH: i64 = 100;

#[derive(Debug, Clone)]
pub struct Player {
	pub plan: Option<Control>,
	pub pos: Pos,
	pub dir: Direction,
	pub health: i64
}

impl Player {
	pub fn new(pos: Pos) -> Self {
		Self {
			plan: None,
			pos,
			dir: Direction::North,
			health: 1
		}
	}
}

impl GameObject for Player {
	fn sprite(&self) -> Option<Sprite>{
		Some(Sprite("player".to_string()))
	}
	fn blocking(&self) -> bool {
		false
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorType{
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

impl GameObject for Tile {
	fn sprite(&self) -> Option<Sprite>{
		Some(Sprite(match self {
			Tile::Floor(FloorType::Stone) => "floor",
			Tile::Floor(FloorType::Dirt) => "ground",
			Tile::Floor(FloorType::Grass1) => "grass1",
			Tile::Floor(FloorType::Grass2) => "grass2",
			Tile::Floor(FloorType::Grass3) => "grass3",
			Tile::Gate => "gate",
			Tile::Sanctuary => "sanctuary",
			Tile::Wall => "wall"
		}.to_string()))
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
	pos: Pos
}

impl GameObject for Bullet {
	fn sprite(&self) -> Option<Sprite>{
		Some(Sprite("bullet".to_string()))
	}
	fn blocking(&self) -> bool {
		false
	}
}

pub struct World {
	pub time: Timestamp,
	size: (i64, i64),
	ground: HashMap<Pos, Tile>,
	players: HashMap<PlayerId, Player>,
	bullets: Vec<Bullet>,
	spawnpoint: Pos
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
			bullets: Vec::new(),
			time: Timestamp(0)
		}
	}
	
	pub fn add_player(&mut self, playerid: &PlayerId) -> Result<()> {
		if self.players.contains_key(playerid){
			return Err(aerr!("player {} already exists", playerid));
		}
		self.players.insert(playerid.clone(), Player::new(self.spawnpoint));
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
		let mut dead: Vec<PlayerId> = self.players.iter()
			.filter_map(|(playerid, player)|
				if player.health <= 0 {
					Some(playerid.clone())
				} else {None}
			)
			.collect();
		for playerid in dead {
			self.players.insert(playerid, Player::new(self.spawnpoint));
		};
		for player in self.players.values_mut() {
			if self.ground.get(&player.pos) == Some(&Tile::Sanctuary) {
				player.health += 4;
				if player.health > MAX_HEALTH {
					player.health = MAX_HEALTH;
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
							self.bullets.push(Bullet{direction: player.dir, pos: player.pos});
						}
					}
					_ => {}
				}
				player.plan = None;
			}
		}
	}
	
	fn update_bullets(&mut self) {
		let players = self.player_map();
		self.bullets = self.bullets.clone().into_iter().filter_map(|mut bullet| {
			for i in 0..2 {
				bullet.pos = bullet.pos + bullet.direction;
				if let Some(playerid) = players.get(&bullet.pos){
					self.players.get_mut(playerid).unwrap().health -= 10;
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
	
	fn player_map(&self) -> HashMap<Pos, PlayerId> {
		self.players.iter().map(|(playerid, player)| (player.pos, playerid.clone())).collect()
	}
	
	pub fn update(&mut self) {
		self.update_bullets();
		self.update_players();
		self.time.0 += 1;
	}
	
	
	fn draw(&self) -> FieldMessage {
		let mut sprites: HashMap<Pos, Vec<Sprite>> = self.ground.iter()
			.filter_map(|(pos, tile)| Some((*pos, vec![tile.sprite()?])))
			.collect();
		for bullet in self.bullets.iter() {
			if let Some(sprite) = bullet.sprite(){
				sprites.entry(bullet.pos).or_insert(Vec::new()).insert(0, sprite.clone());
				sprites.entry(bullet.pos + bullet.direction).or_insert(Vec::new()).insert(0, sprite);
			}
		}
		for player in self.players.values() {
			if let Some(sprite) = player.sprite(){
				sprites.entry(player.pos).or_insert(Vec::new()).insert(0, sprite);
			}
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
			wm.health = Some((player.health, MAX_HEALTH));
			views.insert(playerid.clone(), wm);
		}
		views
	}
}

