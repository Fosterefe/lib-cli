use std::io;

use crate::utils::{add_person::new_person, storage::Store};
use crate::views::menu::menu;

pub fn add_user_menu(store: &mut Store) {
    let person = new_person();
        
    store.store.push(person);

    //println!("{:?}", store.store);
    println!("Press any to go to the menu!");
    let mut ui = String::new();
    let _ui = io::stdin().read_line(&mut ui);

    menu(store);
}
