use clap::{Arg, Command};

mod pourover;

fn cli() -> Command<'static> {
    Command::new("cafe")
        .about("A coffee brewing helper!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_invalid_utf8_for_external_subcommands(false)
        .subcommand(
            Command::new("brew")
            .about("Brew constructor")
            .arg(Arg::new("pourover")
                .long("pour-over")
                .short('p'))
            .arg(Arg::new("coffee-mass")
                 .long("coffee-mass")
                 .value_parser(clap::value_parser!(u32))
                 .short('c')
                 .required(true)
                 .takes_value(true)
                 .value_name("COFFEE_MASS"))
        )
}

fn main() {
    let matches = cli().get_matches();

    if let Some(("brew", sub_matches)) = matches.subcommand() {
        // Getting the coffee mass
        let coffee_mass: u32  = *sub_matches.get_one("coffee-mass")
            .expect("Error reading coffee mass.");
        
        pourover::generate_pour_over_instructions(pourover::InputMass::CoffeeMass(coffee_mass));
    }
}
