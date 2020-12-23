
use crate::creature::CreatureType;
use CreatureType::*;


pub fn wave_composition(wave: usize, nplayers: usize) -> Vec<CreatureType> {
	let mut monsters = Vec::new();
	let wavedef = [
		vec![(Zombie, 5)], // wave 0 is normally skipped, but just in case
		vec![(Zombie, 8)],
		vec![(Zombie, 12)],
		vec![(Zombie, 10 + 3 * nplayers), (Ymp, (nplayers+1)/2)],
		vec![(Zombie, 12 + 4 * nplayers), (Ymp, (nplayers+1)/2)],
		vec![(Zombie, 12 + 4 * nplayers), (Ymp, nplayers+1)],
		vec![(Zombie, 10 + 3 * nplayers), (Troll, 1)],
		vec![(Zombie, 12 + 4 * nplayers), (Ymp, (nplayers+2)/2), (Troll, 1)],
		vec![(Zombie, 12 + 4 * nplayers), (Troll, 2)],
		vec![(Zombie, wave + 5 * nplayers), (Ymp, nplayers + wave / 3), (Zombie, wave + 5), (Troll, nplayers + wave / 4)]
	];
	let idx = std::cmp::min(wave, wavedef.len() - 1);
	for (typ, num) in &wavedef[idx] {
		monsters.append(&mut [*typ].repeat(*num));
	}
	monsters
}
