
use crate::creature::CreatureType;
use CreatureType::*;


pub fn wave_composition(wave: usize) -> Vec<CreatureType> {
	let mut monsters = Vec::new();
	let wavedef = [
		vec![(Zombie, 5)], // wave 0 is normally skipped, but just in case
		vec![(Zombie, 8)],
		vec![(Zombie, 10), (Worm, 1)],
		vec![(Zombie, 12), (Worm, 1), (Ymp, 1)],
		vec![(Zombie, 14 ), (Ymp, 1), (Worm, 2)],
		vec![(Zombie, 12 ), (Wasp, 10), (Worm, 2)],
		vec![(Zombie, 16), (Xiangliu, 2), (Worm, 1)],
		vec![(Zombie, 16), (Ymp, 2), (Worm, 1)],
		vec![(Zombie, 16), (Ymp, 1), (Worm, 1), (Xiangliu, 1)],
		vec![(Zombie, 16), (Ymp, 1), (Worm, 1), (Xiangliu, 2)],
		vec![(Zombie, 16 ), (Ymp, 2), (Worm, 2), (Xiangliu, 2)],
		vec![(Zombie, 12), (Worm, 1), (Troll, 1), (Wasp, 10)],
		vec![(Zombie, 16), (Ymp, 2), (Worm, 2), (Wasp, 10)],
		vec![(Zombie, 16), (Troll, 2), (Xiangliu, 3), (Wasp, 20)],
	];
	let postwavenum = if wave >= wavedef.len(){wave - wavedef.len()} else {0};
	let postwaves = vec![
		(Wasp, 10 + postwavenum),
		(Zombie, 18 + postwavenum),
		(Ymp, 3 + (3 + postwavenum) / 3),
		(Worm, postwavenum/2 + 1),
		(Xiangliu, 2 + (3 + postwavenum) / 3),
		(Zombie, postwavenum + 5),
		(Troll, 1)
	];
	let composition = if wave < wavedef.len(){
			&wavedef[wave]
		} else {
			&postwaves
		};
	for (typ, num) in composition {
		monsters.append(&mut [*typ].repeat(*num));
	}
	monsters
}
