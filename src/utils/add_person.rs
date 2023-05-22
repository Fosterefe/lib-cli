use std::{io, process::Command};

use crate::person::models::Person;

pub fn new_person() -> Person {
    let mut name = String::new();
    let mut lastname = String::new();
    let mut address = String::new();
    let mut email = String::new();

    let stdin = io::stdin();

    Command::new("clear").status().unwrap();

    println!("Add a new contact\n");

    println!("\nIntroduce the name: ");
    let _name_inpt = stdin.read_line(&mut name);

    println!("Introduce the lastname");
    let _lastname_inpt = stdin.read_line(&mut lastname);

    println!("Introduce the address: ");
    let _addres_inpt = stdin.read_line(&mut address);

    println!("Introduce the email: ");
    let _email_inpt = stdin.read_line(&mut email);

    let person = Person::new(
        name.trim().to_string(),
        lastname.trim().to_string(),
        address.trim().to_string(),
        email.trim().to_string(),
    );
    
    println!("\nContact created successfully!\n");

    person
}
