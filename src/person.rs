pub mod models {
    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct Person {
        pub name: String,
        pub lastname: String,
        pub address: String,
        pub email: String,
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
