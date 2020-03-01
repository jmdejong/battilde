

use specs::{
	Entities,
	ReadStorage,
	WriteStorage,
	System,
	Join,
	Write
};

use crate::{
	components::{
		Controller,
		Position,
		Inventory,
		AttackInbox,
		AttackMessage
	},
	resources::{NewEntities},
	components::item::ItemAction::{None, Build, Eat},
	controls::Control,
};


pub struct Use;
impl <'a> System<'a> for Use {
	type SystemData = (
		Entities<'a>,
		ReadStorage<'a, Controller>,
		WriteStorage<'a, Position>,
		WriteStorage<'a, Inventory>,
		Write<'a, NewEntities>,
		WriteStorage<'a, AttackInbox>
	);
	
	fn run(&mut self, (entities, controllers, positions, mut inventories, mut new, mut attacked): Self::SystemData) {
		for (ent, controller, position, inventory) in (&entities, &controllers, &positions, &mut inventories).join(){
			match &controller.control {
				Control::Use(rank) => {
					if let Some(item) = inventory.items.get(*rank) {
						match &item.action {
							Build(template) => {
								let _ = new.create(position.pos, template.clone());
								inventory.items.remove(*rank);
							}
							Eat(health_diff) => {
								AttackInbox::add_message(&mut attacked, ent, AttackMessage::new(-*health_diff));
								inventory.items.remove(*rank);
							}
							None => {}
						}
					}
				}
				_ => {}
			}
		}
	}
}
