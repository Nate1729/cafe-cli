#[derive(PartialEq, Debug)]
struct PourOver {
    /// ratio of grams of water to coffee; a ratio of 16 would
    /// mean 16 grams of water for every 1 gram of water.
    ratio: u32,
    coffee_mass: u32,
    water_mass: u32,
    bloom_mass: u32,
    bloom_mass_limit: u32,
    main_pour_mass: u32,
}

pub enum InputMass {
    CoffeeMass(u32),
    WaterMass(u32)
}

impl PourOver {
    /// Construct the `PourOver` struct. This validates that both
    /// `coffee_mass` and `opt_ratio` are non-zero.
    fn calculate_brew_from_coffee(coffee_mass: u32, opt_ratio: Option<u32>) -> Self {
        let ratio = opt_ratio.unwrap_or(16);

        // Validating inputs
        if coffee_mass == 0 {
            panic!("Error coffee_mass cannot be zero!");
        }
        if ratio == 0 {
            panic!("Error! coffee/water ratio cannot be zero");
        }

        let water_mass = coffee_mass * ratio;
        let bloom_mass = coffee_mass * 2;
        let bloom_mass_limit = coffee_mass * 3;
        let main_pour_mass = water_mass * 3 / 5; // Have to use integer arthimetic
        
        PourOver {
            ratio,
            coffee_mass,
            water_mass,
            bloom_mass,
            bloom_mass_limit,
            main_pour_mass
        }
    }

    fn print_brew_instructions(&self) {
        println!("Coffee amount: {}g", self.coffee_mass);
        println!("Water required: {}g", self.water_mass);

        println!("\n\n=== Blooming ===");
        println!("Add {}g-{}g of bloom water (0-45s)", self.bloom_mass, self.bloom_mass_limit);

        println!("\n=== Main Pour ===");
        println!("Add up to {} in 30 seconds (45s - 1m15s)", self.main_pour_mass);

        println!("\n=== Final Pour ===");
        println!("Add up to {} over next 30 seconds (1m15s - 1m45s)", self.water_mass);
        println!("Stir clockwise and use spoon to knock grinds from sid-wall");
    }
}

pub fn generate_pour_over_instructions(mass: InputMass) {
    let pourover = match mass {
       InputMass::CoffeeMass(m) => PourOver::calculate_brew_from_coffee(m, None),
       InputMass::WaterMass(_) => panic!("From water mass is not implemented yet!"),
    };

    pourover.print_brew_instructions();
}


#[cfg(test)]
mod tests {
    use super::PourOver;
    #[test]
    fn test_calculate_brew_from_coffee_default_ratio() {
        // Not using a specific ratio should default to using
        // 16 as the ratio.
        let coffee_mass: u32 = 15;

        let expected = PourOver {
            ratio: 16,
            coffee_mass,
            water_mass: coffee_mass * 16,
            bloom_mass: coffee_mass * 2,
            bloom_mass_limit: coffee_mass * 3,
            main_pour_mass: (coffee_mass * 16) * 3 / 5,
        };

        assert_eq!(PourOver::calculate_brew_from_coffee(coffee_mass, None),  expected); 
    }

    #[test]
    fn test_calculate_brew_from_coffee_custom_ratio() {
        let coffee_mass: u32 = 15;
        let ratio = 10;

        let expected = PourOver {
            ratio,
            coffee_mass,
            water_mass: coffee_mass * ratio,
            bloom_mass: coffee_mass * 2,
            bloom_mass_limit: coffee_mass * 3,
            main_pour_mass: (coffee_mass * ratio) * 3 / 5,
        };

        assert_eq!(PourOver::calculate_brew_from_coffee(coffee_mass, Some(ratio)),  expected);
    }

    #[test]
    #[should_panic]
    fn test_calculate_brew_from_coffee_panic_with_zero_coffee() { 
        PourOver::calculate_brew_from_coffee(0, None);
    }

    #[test]
    #[should_panic]
    fn test_calculate_brew_from_coffee_panic_with_zero_ratio() {
        PourOver::calculate_brew_from_coffee(12, Some(0));
    }
        
}
