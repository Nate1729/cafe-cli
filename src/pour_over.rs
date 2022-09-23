use std::io;
use io::{ Write };

const POUR_OVER_GRIND_SIZE: u32 = 14;
const DEFAULT_COFEE_WATER_RATIO: u32 = 16;
/// All of the data required to make a coffee with the 
/// classic pour over process.
pub struct PourOver {
    /// Mass of the coffee used. This is the mass _before_ grinding
    coffee_mass: u32,
    water_mass: u32,
    /// The number of grams of water for every 1 gram of coffee
    ratio: u32,
}

enum WeightType {
    WATER,
    COFFEE
}

impl PourOver {
    /// Construtor based on the intended output of water. Use this if you don't
    /// know how much coffee to grind, but you know how big your mug is. It is important
    /// to note that the input for water_mass will never equal the amount of
    /// _actual_ coffee made; Some of the water stays inside the filter/beans.
    fn from_water(water_mass: u32, opt_ratio: Option<u32>) -> Self{
        let ratio = match opt_ratio {
            Some(i) => i,
            None => DEFAULT_COFEE_WATER_RATIO,
        };

        PourOver {
            coffee_mass: water_mass/ratio,
            water_mass,
            ratio,
        }
    }

    /// Constructor based on the amount of coffee being used. Use this if you
    /// don't know how much water you need to use.
    fn from_coffee(coffee_mass: u32, opt_ratio: Option<u32>) -> Self {
        let ratio = match opt_ratio {
            Some(i) => i,
            None => DEFAULT_COFEE_WATER_RATIO,
        };

        PourOver {
            coffee_mass,
            water_mass: coffee_mass * ratio,
            ratio,
        }
    } 

    pub fn configure_brew(grind_size: u32) -> Self {
        let weight_type = get_weight_type();

        let text = match weight_type {
            WeightType::WATER => "water",
            WeightType::COFFEE => "coffee"
        };
        
        let mass = loop {
            let mut buffer = String::new();

            print!("Enter {} mass: ", text);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer);

            let mass: u32 = buffer.trim().parse().expect("Error reading in mass!");
            break mass;
        };

        match weight_type {
            WeightType::WATER => PourOver::from_water(mass, None),
            WeightType::COFFEE => PourOver::from_coffee(mass, None),
        }

    }
}

fn get_weight_type() -> WeightType {
    loop {
        let mut buffer = String::new();
        print!("Do you have the coffee or water mass? [c/w]");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();

        let char_in = buffer.trim();
        match char_in {
            "c" => return WeightType::COFFEE,
            "w" => return WeightType::WATER,
            _ => {
                println!("Input not recognized!\n");
                continue;
            },
        }
    }
}
