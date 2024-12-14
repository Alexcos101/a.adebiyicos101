// Define a struct to represent a developer
struct Developer {
    name: String,
    experience: u32,
}

fn main() {
    // List of developers
    let developers = vec![
        Developer { name: String::from("Alice"), experience: 5 },
        Developer { name: String::from("Bob"), experience: 8 },
        Developer { name: String::from("Charlie"), experience: 6 },
    ];

    // Find the developer with the most experience
    let mut most_experienced = &developers[0]; // Assume first developer initially

    for dev in &developers {
        if dev.experience > most_experienced.experience {
            most_experienced = dev;
        }
    }

    // Print the result
    println!(
        "The developer with the most experience is {} with {} years.",
        most_experienced.name, most_experienced.experience
    );
}