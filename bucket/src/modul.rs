pub mod modul {
    #[derive(Debug)]
    pub struct Breakfast {
        pub food: String,
        drink: String,
    }

    impl Breakfast {
        pub fn summer_menu() -> Breakfast {
            Breakfast {
                food: String::from("Toast"),
                drink: String::from("Orange juice"),
            }
        }

        pub fn update_drink(&mut self) {
            self.drink = String::from("coke");
        }
    }
}
