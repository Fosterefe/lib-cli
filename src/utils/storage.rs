use crate::person::models::Person;

#[derive(Debug)] #[allow(dead_code)]
pub struct Store {
    pub store: Vec<Person>
}

impl Store {
    pub fn new() -> Self {
        Self {
            store: Vec::new()
        }
    }

    pub fn add_contact(&mut self, person: Person) {
        self.store.push(person); 
    }

    pub fn length(&self) -> usize {
        self.store.len()
    }
}

/* pub fn storage(n_person: Person) -> Vec<Person> {
    let mut store: Vec<Person> = Vec::new();

    store.push(n_person);
    
    store
} */
