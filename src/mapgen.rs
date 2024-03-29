
use std::str::FromStr;
use serde::{Serialize, de, Deserialize, Deserializer};
use rand::Rng;
use crate::{
	Pos,
	Direction,
	tile::{Tile, FloorType, WallType, ObstacleType},
	creature::CreatureType,
	util::randomize,
	errors::AnyError,
	aerr,
	grid::Grid,
	gamemode::GameMode,
	pos::Distance
};



#[derive(Debug, Clone)]
pub struct MapTemplate {
	pub size: Pos,
	pub ground: Grid<Tile>,
	pub creatures: Vec<(Pos, CreatureType)>,
	pub spawnpoint: Pos,
	pub monsterspawn: Vec<Pos>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinMap{
	Square
}

impl FromStr for BuiltinMap {
	type Err = AnyError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"square" => Ok(Self::Square),
			_ => Err(aerr!("'{}' is not a valid map", s))
		}
	}
}

#[derive(Debug, Clone)]
pub enum MapType {
	Builtin(BuiltinMap),
	Custom(MapTemplate)
}

pub fn create_map(typ: &MapType, gamemode: GameMode) -> MapTemplate {
	match typ {
		MapType::Builtin(BuiltinMap::Square) => create_square_map(gamemode),
		MapType::Custom(template) => template.clone()
	}
}


fn create_square_map(gamemode: GameMode) -> MapTemplate {
	let size = Pos::new(64, 64);
	let mut map = MapTemplate {
		size,
		ground: Grid::new(size, Tile::Floor(FloorType::Dirt)),
		creatures: Vec::new(),
		spawnpoint: Pos::new(size.x / 2, size.y / 2),
		monsterspawn: vec![Pos::new(0,0), Pos::new(size.x - 1, 0), Pos::new(0, size.y - 1), Pos::new(size.x - 1, size.y - 1)],
	};

	for x in 0..map.size.x {
		for y in 0..map.size.y {
			let pos = Pos::new(x, y);
			let dspawn = (Pos::new(x, y) - map.spawnpoint).abs();
			let floor = if dspawn.x <= 3 && dspawn.y <= 3 {
				Tile::Sanctuary
			} else if dspawn.x <= 4 && dspawn.y <= 4 && dspawn.x != dspawn.y{
				Tile::Gate
			} else if dspawn.x <= 1 || dspawn.y <= 1 {
				Tile::Floor(FloorType::Dirt)
			} else {
				Tile::Floor([FloorType::Grass1, FloorType::Grass2, FloorType::Grass3][randomize((x+1) as u32 + randomize((y+1) as u32)) as usize % 3])
			};
			map.ground.set(pos, floor);
		}
	}
	
	let d: Vec<(i64, i64)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
	for (dx, dy) in d {
		for (px, py) in &[(3, 3), (4, 3), (4, 2), (3, 4), (2, 4), (4, 4)] {
			map.ground.set(map.spawnpoint + Pos::new(px * dx, py * dy), Tile::Wall(WallType::Wall));
		}
		if gamemode.has_pillars() {
			map.ground.set(map.spawnpoint + Pos::new(4 * dx, 4 * dy), Tile::Wall(WallType::Rubble));
			map.creatures.push((map.spawnpoint + Pos::new(4*dx, 4*dy), CreatureType::Pillar));
		}
		
		if rand::random() {
			let lakepos = Pos::new(
					rand::thread_rng().gen_range(12..size.x / 2 - 8) * dx,
					rand::thread_rng().gen_range(12..size.y / 2 - 8) * dy
				) + map.spawnpoint;
			let mut p = lakepos;
			for _i in 0..16 {
				map.ground.set(p, Tile::Obstacle(ObstacleType::Water));
				p = p + Direction::DIRECTIONS[rand::thread_rng().gen_range(0..4)];
				if lakepos.distance_to(p) > Distance(4){
					break;
				}
			}
		}
	}
	map
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MapTemplateSave {
	pub size: Pos,
	pub ground: Vec<String>,
	pub creatures: Vec<(Pos, CreatureType)>,
	pub spawnpoint: Pos,
	pub monsterspawn: Vec<Pos>,
}

impl<'de> Deserialize<'de> for MapTemplate {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where D: Deserializer<'de> {
		let MapTemplateSave{size, ground, creatures, spawnpoint, monsterspawn} =
			MapTemplateSave::deserialize(deserializer)?;
		let mut groundmap = Grid::new(size, Tile::Floor(FloorType::Dirt));
		for (y, line) in ground.iter().enumerate(){
			for (x, c) in line.chars().enumerate(){
				let tile = Tile::from_char(c).ok_or_else(||de::Error::custom(format!("Invalid tile character '{}'", c)))?;
				groundmap.set(Pos::new(x as i64, y as i64), tile);
			}
		}
		Ok(MapTemplate {
			size,
			spawnpoint,
			creatures,
			monsterspawn,
			ground: groundmap
		})
	}
}


