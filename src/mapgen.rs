
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, de, Deserialize, Deserializer};
use crate::{
	Pos,
	tile::{Tile, FloorType, WallType},
	creature::CreatureType,
	util::randomize,
	errors::AnyError,
	aerr
};



#[derive(Debug, Clone, PartialEq)]
pub struct MapTemplate {
	pub size: (i64, i64),
	pub ground: HashMap<Pos, Tile>,
	pub creatures: Vec<(Pos, CreatureType)>,
	pub spawnpoint: Pos,
	pub monsterspawn: Vec<Pos>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum MapType {
	Builtin(BuiltinMap),
	Custom(MapTemplate)
}

pub fn create_map(typ: &MapType) -> MapTemplate {
	match typ {
		MapType::Builtin(BuiltinMap::Square) => create_square_map(),
		MapType::Custom(template) => template.clone()
	}
}


fn create_square_map() -> MapTemplate {
	let mut map = MapTemplate {
		size: (64, 64),
		ground: HashMap::new(),
		creatures: Vec::new(),
		spawnpoint: Pos::new(32, 32),
		monsterspawn: vec![Pos::new(0,0), Pos::new(63, 0), Pos::new(0, 63), Pos::new(63, 63)],
	};

	for x in 0..map.size.0 {
		for y in 0..map.size.1 {
			let dspawn = (Pos::new(x, y) - map.spawnpoint).abs();
			let floor = if dspawn.x <= 3 && dspawn.y <= 3 {
				Tile::Sanctuary
			} else if dspawn.x <= 4 && dspawn.y <= 4 && dspawn.x != dspawn.y{
				Tile::Gate
			} else if dspawn.x <= 1 || dspawn.y <= 1 {
				Tile::Floor(FloorType::Dirt)
			} else {
				Tile::Floor([FloorType::Grass1, FloorType::Grass2, FloorType::Grass3][randomize(x as u32 + randomize(y as u32)) as usize % 3])
			};
			map.ground.insert(Pos::new(x, y), floor);
		}
	}
	let d: Vec<(i64, i64)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
	let p: Vec<(i64, i64)> = vec![(3, 3), (4, 3), (4, 2), (3, 4), (2, 4)];
	for (dx, dy) in d {
		for (px, py) in p.iter() {
			map.ground.insert(map.spawnpoint + Pos::new(px * dx, py * dy), Tile::Wall(WallType::Wall));
		}
		map.ground.insert(map.spawnpoint + Pos::new(4 * dx, 4 * dy), Tile::Wall(WallType::Rubble));
		map.creatures.push((map.spawnpoint + Pos::new(4*dx, 4*dy), CreatureType::Pillar));
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
		let mut groundmap = HashMap::new();
		for (y, line) in ground.iter().enumerate(){
			for (x, c) in line.chars().enumerate(){
				let tile = Tile::from_char(c).ok_or(de::Error::custom(format!("Invalid tile character '{}'", c)))?;
				groundmap.insert(Pos::new(x as i64, y as i64), tile);
			}
		}
		Ok(MapTemplate {
			size: (size.x, size.y),
			spawnpoint,
			creatures,
			monsterspawn,
			ground: groundmap
		})
	}
}


