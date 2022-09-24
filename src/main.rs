use clap::{Arg, Command};

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

//fn push_args() -> Vec<clap::Arg<'static>> {
//    vec![arg!(-m --message <MESSAGE>).required(false)]
//}

fn generate_pour_over_instructions(coffee_mass: u32) {
    // coffee to water ratio.
    let ratio = 16;
    let total_brew_mass = coffee_mass * 16;
    println!("Coffee amount: {}g", coffee_mass);
    println!("Water required: {}g", coffee_mass * ratio);

    println!("\n\n=== Blooming ===");
    println!("Add {}g-{}g of bloom water (0-45s)", coffee_mass*2, coffee_mass*3);

    println!("\n=== Main Pour ===");
    println!("Add up to {} in 30 seconds (45s - 1m15s)", total_brew_mass*3/5);

    println!("\n=== Final Pour ===");
    println!("Add up to {} over next 30 seconds (1m15s - 1m45s)", total_brew_mass);
    println!("Stir clockwise and use spoon to knock grinds from sid-wall");

}

fn main() {
    let matches = cli().get_matches();

    if let Some(("brew", sub_matches)) = matches.subcommand() {
        // Getting the coffee mass
        let coffee_mass: u32  = *sub_matches.get_one("coffee-mass")
            .expect("Error reading coffee mass.");
        
        generate_pour_over_instructions(coffee_mass);
    }
}
