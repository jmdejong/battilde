
use std::collections::HashMap;
use specs::Builder;
use rand::Rng;

use crate::{
	PlayerId,
	RoomId,
	Sprite,
	playerstate::RoomPos,
	components::{
		AttackType,
		Clan
	},
	parameter::{Parameter, ParameterType},
	Timestamp
};


macro_rules! components {
	(post: $($comp: ident ($($paramname: ident : $paramtype: ident, $extraction: expr),*) $creation: expr);*;) => {
		#[derive(Clone)]
		pub enum ComponentWrapper{
			$(
				$comp(crate::components::$comp),
			)*
		}
		impl ComponentWrapper {
			pub fn build<A: Builder>(&self, builder: A ) -> A {
				match self.clone() {
					$(
						Self::$comp(c) => builder.with(c),
					)*
				}
			}
			pub fn load_component(comptype: ComponentType, mut parameters: HashMap<&str, Parameter>) -> Option<Self> {
				#[allow(unused_imports, unreachable_code)]
				match comptype {
					$(
						ComponentType::$comp => Some(Self::$comp({
							use crate::components::$comp;
							$(
								let $paramname = match parameters.remove(stringify!($paramname))? {
									Parameter::$paramtype(p) => p,
									_ => {return None}
								};
							)*
							$creation
						})),
					)*
				}
			}
		}
		#[derive(Debug, PartialEq, Eq, Clone, Copy)]
		pub enum ComponentType {
			$(
				$comp,
			)*
		}
		impl ComponentType {
			pub fn from_str(typename: &str) -> Option<ComponentType>{
				match typename {
					$(
						stringify!($comp) => Some(Self::$comp),
					)*
					_ => None
				}
			}
			pub fn parameters(&self) -> HashMap<&str, ParameterType> {
				match self {
					$(
						Self::$comp => {
							#[allow(unused_mut)]
							let mut h = HashMap::new();
							$(
								h.insert(stringify!($paramname), ParameterType::$paramtype);
							)*
							h
						},
					)*
				}
			}
		}
		
		use specs::{World, Entity, WorldExt};
		pub fn extract_parameter(typ: ComponentType, parameter: &str, world: &World, ent: Entity) -> Option<Parameter> {
			match typ {
				$(
					
					#[allow(path_statements)]
					ComponentType::$comp => {
						$(
							if parameter == stringify!($paramname) {
								#[allow(unreachable_code, non_snake_case)]
								return Some(Parameter::$paramtype({
									let components = world.read_component::<crate::components::$comp>();
									#[allow(unused_variables)]
									let $comp = components.get(ent)?;
									$extraction
								}))
							}
						)*
						None::<Parameter> 
					}
				)*
			}
		}
	};
	// no parameters: make unit struct
	(pre: ($($done: tt)*) $comp: ident; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp () {$comp};) $($tail)*);
	};
	// struct is just parameters
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident),*);$($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {$comp.$paramname.clone()}),*) {$comp{$($paramname,)*}};) $($tail)*);
	};
	// full definition minus variable exraction
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, {None?}),*) $creation;) $($tail)*);
	};
	// full definition
	(pre: ($($done: tt)*) $comp: ident ($($paramname: ident : $paramtype: ident ($extraction: expr)),*) $creation: expr; $($tail:tt)*) => {
		components!(pre: ($($done)* $comp ($($paramname : $paramtype, $extraction),*) $creation;) $($tail)*);
	};
	(pre: ($($done: tt)*)) => {
		components!(post: $($done)*);
	};
	($($all: tt)*) => {components!(pre: () $($all)*);};
}

components!(
	Visible (name: String, sprite: String, height: Float) {
		Visible {
			sprite: Sprite{name: sprite},
			height,
			name
		}
	};
	Movable (cooldown: Int);
	Blocking;
	Floor;
	Player (name: String) {Player::new(PlayerId{name})};
	Item (ent: Template, name: String, action: Action);
	Inventory () {panic!("inventory from parameters not implemented")};
	Health (health: Int, maxhealth: Int);
	Serialise () {panic!("serialise from parameters not implemented")};
	RoomExit (destination: String, dest_pos: String) {
		RoomExit {
			destination: RoomId::from_str(&destination),
			dest_pos: if dest_pos.is_empty() {
					RoomPos::Unknown
				} else {
					RoomPos::Name(dest_pos)
				}
		}
	};
	Trap (damage: Int) {Trap{attack: AttackType::Attack(damage)}};
	Fighter (damage: Int, cooldown: Int) {Fighter{attack: AttackType::Attack(damage), cooldown, range: 1}};
	Healing (delay: Int, health: Int) {Healing{delay, health, next_heal: None}};
	Volatile (delay: Int) {Volatile{delay, end_time: None}};
	Autofight () {Autofight::default()};
	MonsterAI (move_chance: Float, homesickness: Float, view_distance: Int);
	Mortal;
	Spawner (amount: Int, delay: Int, clan: String, template: Template, initial_spawn: Bool) {
		Spawner{
			amount: amount as usize,
			delay,
			clan: Clan{name:
				if clan == "" {
					format!("$random({})", rand::thread_rng().gen::<u32>())
				} else {
					clan
				}
			},
			template: template.unsaved(),
			last_spawn: if initial_spawn {Some(Timestamp(-delay))} else {None}
		}
	};
	Clan (name: String);
	Home (home: Pos);
	Faction (faction: String) {Faction::from_str(faction.as_str())?};
	Interactable (action: String) {Interactable::from_str(action.as_str())?};
	Loot (loot: LootList);
	Grow (into: Template, delay: Int, target_time: SomeTime);
	Equipment () {panic!("equipment from parameters not implemented")};
	CreationTime (time: Int) {CreationTime{time: Timestamp(time)}};
);


pub type PreEntity = Vec<ComponentWrapper>;




