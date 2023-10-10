pub mod person;
pub mod utils;
pub mod views;

use crate::views::menu::menu;
use crate::utils::storage::Store;

fn main() {
    let mut store = Store::new();
    menu(&mut store);
}

//from this repo
