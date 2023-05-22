use std::io;

use crate::utils::storage::Store;
use crate::views::menu::menu;

pub fn search_by_name(store: &mut Store) {
    println!("\nIntroduce a name");
    let mut query = String::new();
    let _query = io::stdin().read_line(&mut query);

    store.store.iter().for_each(|item| {
        if item.name == query.trim().to_string() {
            println!("-------------------");
            println!("Contact");

            println!("\nName: {}", item.name);
            println!("\nLastName: {}", item.lastname);
            println!("\nAddress: {}", item.address);
            println!("\nEmail: {}", item.email);

            println!("-------------------");
        }
    });

    println!("Press any key to return to home");
    let mut exit = String::new();
    let _exit = io::stdin().read_line(&mut exit);

    menu(store)
}
