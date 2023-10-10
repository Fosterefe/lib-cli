use std::io;

use crate::utils::storage::Store;
use crate::views::menu::menu;

pub fn search_by_name(store: &mut Store) {
    println!("\nIntroduce a name");
    let mut query = String::new();
    let _query = io::stdin().read_line(&mut query);
    let mut found = false;
    // iter store vec with the iter() method
    store.store.iter().for_each(|item| {
        if item.name == query.trim().to_string() {
            found = true;
            println!("-------------------");
            println!("Contact");

            println!("\nName: {}", item.name);
            println!("\nLastName: {}", item.lastname);
            println!("\nAddress: {}", item.address);
            println!("\nEmail: {}", item.email);

            println!("-------------------");
        }
    });

    if !found { println!("No contacts with that name found!\n") };

    println!("Press any key to return to home");
    let mut exit = String::new();
    let _exit = io::stdin().read_line(&mut exit);

    menu(store)
}
