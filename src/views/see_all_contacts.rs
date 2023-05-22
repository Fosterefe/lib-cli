use std::io;

use crate::utils::storage::Store;
use crate::views::menu::menu;

pub fn all_contacts(store: &mut Store) {
        
    for (i, person) in store.store.iter().enumerate() {
        println!("-------------------");
        println!("Contact: {}", i + 1);
        
        println!("\nName: {}", person.name);
        println!("\nLastName: {}", person.lastname);
        println!("\nAddress: {}", person.address);
        println!("\nEmail: {}", person.email);

        println!("-------------------");
    }

    println!("Press any to go to the menu!");
    let mut ui = String::new();
    let _ui = io::stdin().read_line(&mut ui);

    menu(store)

}
