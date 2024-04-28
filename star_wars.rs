use std::any::Any;

mod battleship {
    use std::any::Any;

    pub trait Battleship {
        fn get_name(&self) -> &String;
        fn is_rebel(&self) -> &bool;
        fn get_shield_capacity(&self) -> &u32;
        fn get_blaster_damage(&self) -> &u32;
        fn get_speed(&self) -> &u32;
        fn has_lightspeed_module(&self) -> &bool;
        fn travel(&self, destination: String);
        fn fight(&self, ship: &dyn Battleship);

        fn as_any(&self) -> &dyn Any;
    }
}

mod xwing {
    use std::any::Any;
    use crate::battleship::Battleship;

    pub struct XWing {
        name: String,
        is_rebel: bool,
        shield_capacity: u32,
        blaster_damage: u32,
        speed: u32,
        has_lightspeed_module: bool,
        maneuverability: u32,
    }

    impl XWing {
        pub fn new(name: String, is_rebel: bool, shield_capacity: u32, blaster_damage: u32, speed: u32, has_lightspeed_module: bool, maneuverability: u32) -> Self {
            XWing { name, is_rebel, shield_capacity, blaster_damage, speed, has_lightspeed_module, maneuverability }
        }
        fn get_maneuverability(&self) -> &u32 { &self.maneuverability }
    }

    impl Battleship for XWing {
        fn get_name(&self) -> &String { &self.name }
        fn is_rebel(&self) -> &bool { &self.is_rebel }
        fn get_shield_capacity(&self) -> &u32 { &self.shield_capacity }
        fn get_blaster_damage(&self) -> &u32 { &self.blaster_damage }
        fn get_speed(&self) -> &u32 { &self.speed }
        fn has_lightspeed_module(&self) -> &bool { &self.has_lightspeed_module }
        fn travel(&self, destination: String) {
            println!("Xwing {} en route to {}", self.get_name(), destination);
        }
        fn fight(&self, ship: &dyn Battleship) {
            let ship_as_any = ship.as_any();
            if let Some(other_xwing) = ship_as_any.downcast_ref::<XWing>() {
                if self.is_rebel() != other_xwing.is_rebel() {
                    if self.get_blaster_damage() > other_xwing.get_shield_capacity() {
                        println!("Victory!");
                    } else if self.get_shield_capacity() < other_xwing.get_blaster_damage() {
                        println!("Defeat!");
                    } else {
                        if self.get_maneuverability() > other_xwing.get_maneuverability() {
                            println!("Battle evaded!");
                        } else {
                            println!("Defeat!")
                        }
                    }
                }
            }
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

mod stardestroyer {
    use std::any::Any;
    use crate::battleship::Battleship;

    pub struct StarDestroyer {
        name: String,
        is_rebel: bool,
        shield_capacity: u32,
        blaster_damage: u32,
        speed: u32,
        has_lightspeed_module: bool,
        crew_capacity: u32,
        amount_of_guns: u32,
    }

    impl StarDestroyer {
        pub fn new(name: String, is_rebel: bool, shield_capacity: u32, blaster_damage: u32, speed: u32, has_lightspeed_module: bool, crew_capacity: u32, amount_of_guns: u32) -> Self {
            StarDestroyer { name, is_rebel, shield_capacity, blaster_damage, speed, has_lightspeed_module, crew_capacity, amount_of_guns }
        }
        fn get_crew_capacity(&self) -> &u32 { &self.crew_capacity }
        fn get_amount_of_guns(&self) -> &u32 { &self.amount_of_guns }
    }

    impl Battleship for StarDestroyer {
        fn get_name(&self) -> &String { &self.name }
        fn is_rebel(&self) -> &bool { &self.is_rebel }
        fn get_shield_capacity(&self) -> &u32 { &self.shield_capacity }
        fn get_blaster_damage(&self) -> &u32 { &self.blaster_damage }
        fn get_speed(&self) -> &u32 { &self.speed }
        fn has_lightspeed_module(&self) -> &bool { &self.has_lightspeed_module }
        fn travel(&self, destination: String) {
            println!("Xwing {} en route to {}", self.get_name(), destination);
        }
        fn fight(&self, ship: &dyn Battleship) {
            let ship_as_any = ship.as_any();
            if let Some(other_stardestroyer) = ship_as_any.downcast_ref::<StarDestroyer>() {
                if self.is_rebel() != other_stardestroyer.is_rebel() {
                    if self.get_amount_of_guns() * self.get_blaster_damage() > *other_stardestroyer.get_shield_capacity() {
                        println!("Victory!");
                    } else if *self.get_shield_capacity() < other_stardestroyer.get_amount_of_guns() * other_stardestroyer.get_blaster_damage() {
                        println!("Defeat!");
                    } else {
                        println!("Both ships destroyed");
                    }
                }
            }
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
}

mod spaceshipfactory {
    use crate::{battleship::Battleship, stardestroyer::StarDestroyer, xwing::XWing};

    fn create_xwing(name: String, is_rebel: bool, shield_capacity: u32, blaster_damage: u32, speed: u32, has_lightspeed_module: bool, maneuverability: u32) -> XWing {
        XWing::new(name, is_rebel, shield_capacity, blaster_damage, speed, has_lightspeed_module, maneuverability)
    }

    fn create_stardestroyer(name: String, is_rebel: bool, shield_capacity: u32, blaster_damage: u32, speed: u32, has_lightspeed_module: bool, crew_capacity: u32, amount_of_guns: u32) -> StarDestroyer {
        StarDestroyer::new(name, is_rebel, shield_capacity, blaster_damage, speed, has_lightspeed_module, crew_capacity, amount_of_guns)
    }
    pub fn main() {
        let xwing1 = create_xwing("Pesho".to_string(), false, 100, 50, 40, false, 10);
        let xwing2 = create_xwing("Dimitur".to_string(), true, 50, 100, 40, false, 20);

        xwing1.fight(&xwing2);

        let star_destroyer1 = create_stardestroyer("Nikolay".to_string(), true, 1000, 30, 10, true, 30, 500);
        let star_destroyer2 = create_stardestroyer("Gosho".to_string(), false, 1001, 10, 10, true, 30, 1000);

        star_destroyer1.fight(&star_destroyer2);
    }
}

fn main() {
    spaceshipfactory::main()
}