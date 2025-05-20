use inventory::{Inventory, Item, TextureState};
use macroquad::prelude::*;

mod inventory;

#[macroquad::main("Display Inventory")]
async fn main() {

    let texture_state = TextureState::new().await;
    let mut inventory = Inventory::new();

    let dragon_tooth = Item::new("Dragon_Tooth");
    let rose_flower = Item::new("Rose_Flower");

    inventory.add_item(rose_flower);
    inventory.add_item(dragon_tooth);

    loop {

        inventory.render(&texture_state);

        next_frame().await;
    }
}
