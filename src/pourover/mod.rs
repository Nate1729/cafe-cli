pub fn generate_pour_over_instructions(coffee_mass: u32) {
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
