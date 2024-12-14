use std::fs::File;
use std::io::Write;

fn main() {
    // Define categories and drinks as vectors
    let categories = vec!["Lager", "Stout", "Non-Alcoholic"];
    let drinks = vec![
        vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"],
        vec!["Legend", "Turbo King", "Williams"],
        vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"],
    ];

    // Create a file to write the data
    let mut file = File::create("drinks.txt").expect("Failed to create file");

    // Write the data to the file
    for (i, category) in categories.iter().enumerate() {
        writeln!(file, "{}", category).expect("Failed to write to file");
        for drink in &drinks[i] {
            writeln!(file, " - {}", drink).expect("Failed to write to file");
        }
        writeln!(file, "").expect("Failed to write to file");
    }
}