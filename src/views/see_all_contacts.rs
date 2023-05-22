use std::io;

use crate::utils::storage::Store;

pub fn all_contacts(store: &mut Store) {
        
    for person in store.store.iter() {
        println!("{:?}", person);
    }

    println!("{:?}", store.store);

    println!("Press any to go to the menu!");
    let mut ui = String::new();
    let _ui = io::stdin().read_line(&mut ui);


}
