use std::io;

fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    // Input experience
    println!("Is the employee experienced? (yes/no):");
    io::stdin()
        .read_line(&mut experience_input)
        .expect("Failed to read input");
    let experienced = experience_input.trim().eq_ignore_ascii_case("yes");

    // Input age
    println!("Enter the age of the employee:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read input");
    let age: u32 = age_input.trim().parse().unwrap_or(0);

    // Determine the incentive
    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age >= 28 {
            1_300_000
        } else {
            println!("Invalid input for an experienced employee.");
            return;
        }
    } else {
        100_000
    };

    println!("The annual incentive for the employee is: N{}", incentive);
}

