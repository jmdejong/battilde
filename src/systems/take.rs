

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::components::{
	Controller,
	Position,
	Removed,
	Inventory,
	Item,
	Visible
};

use crate::controls::{Control};
use crate::resources::{Ground, NewEntities};



pub struct Take;
impl <'a> System<'a> for Take {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		Write<'a, Ground>,
		WriteStorage<'a, Removed>,
		ReadStorage<'a, Item>,
		WriteStorage<'a, Inventory>,
		Write<'a, NewEntities>,
		ReadStorage<'a, Visible>
	);
	
	fn run(&mut self, (entities, controllers, positions, ground, mut removed, items, mut inventories, mut new, visibles): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.control {
				Control::Take(rank) if inventory.items.len() < inventory.capacity => {
					let mut ents = ground.by_height(&position.pos, &visibles, &ent);
					if let Some(idx) = rank {
						if *idx >= ents.len() {
							return
						}
						ents = vec!(ents[*idx]);
					}
					for ent in ents {
						if let Some(item) = items.get(ent) {
							inventory.items.insert(0, (item.clone(), false));
							if let Err(msg) = removed.insert(ent, Removed) {
								println!("{:?}", msg);
							}
							break;
						}
					}
				}
				Control::Drop(rank) => {
					if *rank >= inventory.items.len() {
						return
					}
					let (item, _is_equipped) = inventory.items.remove(*rank);
					let _ = new.create(position.pos, item.ent);
				}
				_ => {}
			}
		}
	}
}
