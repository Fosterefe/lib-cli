use std::{io, process::Command};

use crate::views::add_user_menu::add_user_menu;
use crate::views::see_all_contacts::all_contacts;
use crate::utils::storage::Store;
use crate::views::search_by_name::search_by_name;

pub fn menu(store: &mut Store) {
    Command::new("clear").status().unwrap();
    println!("\nContact agenda -Menu");
    println!("1- Add new contact");
    println!("2- See all contacts");
    println!("3- Search by name");

    println!("\nSelect the number: ");

    let mut user_input = String::new();

    let stdin = io::stdin();

    let _u_inpt = stdin.read_line(&mut user_input);

    validate_option(user_input.trim().to_string(), store);
}

fn validate_option(user_input: String, store: &mut Store) {

    if user_input != "1" && user_input != "2" && user_input != "3" {
        println!("\nThere is any option with that number, select a valid option please! \n");
        menu(store);
    }

    if user_input == "1" {
        add_user_menu(store);
    }
    if user_input == "2" {
        all_contacts(store);
    }
    if user_input == "3" {
        search_by_name(store); 
    }
}
