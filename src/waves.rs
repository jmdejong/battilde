
use crate::creature::CreatureType;
use CreatureType::*;


pub fn wave_composition(wave: usize) -> Vec<CreatureType> {
	let mut monsters = Vec::new();
	let wavedef = [
		vec![(Zombie, 5)], // wave 0 is normally skipped, but just in case
		vec![(Zombie, 8)],
		vec![(Zombie, 12), (Worm, 1), (Ymp, 1)],
		vec![(Zombie, 14 ), (Ymp, 1), (Worm, 2)],
		vec![(Zombie, 16), (Xiangliu, 2), (Worm, 1)],
		vec![(Zombie, 16), (Ymp, 2), (Worm, 1)],
		vec![(Zombie, 16 ), (Ymp, 2), (Worm, 2), (Xiangliu, 2)],
		vec![(Zombie, 12), (Worm, 1), (Troll, 1), (Vargr, 1)],
		vec![(Zombie, 16), (Ymp, 2), (Worm, 2), (Vargr, 1)],
		vec![(Zombie, 16), (Troll, 2), (Xiangliu, 3), (Vargr, 3)],
	];
	let postwavenum = if wave >= wavedef.len(){wave - wavedef.len()} else {0};
	let postwaves = vec![
		(Vargr, 2 + (5 + postwavenum) / 5),
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
