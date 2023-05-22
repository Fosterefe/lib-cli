pub mod models {
    #[derive(Debug)] #[allow(dead_code)]
        pub struct Person {
        name: String,
        lastname: String,
        address: String,
        email: String,
    }

    impl Person {
        pub fn new(name: String, lastname: String, address: String, email: String) -> Person {
            Person {
                name,
                lastname,
                address,
                email,
            }
        }
    }
}
