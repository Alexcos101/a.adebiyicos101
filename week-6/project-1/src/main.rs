use std::io;

fn main() {
    loop {
        println!("Choose a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter height, base1, and base2 (space-separated):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let values: Vec<f64> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap_or(0.0))
                    .collect();
                if values.len() == 3 {
                    let area = (values[0] / 2.0) * (values[1] + values[2]);
                    println!("Area of Trapezium: {:.2}", area);
                } else {
                    println!("Invalid input. Please enter 3 values.");
                }
            }
            "2" => {
                println!("Enter diagonal1 and diagonal2 (space-separated):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let values: Vec<f64> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap_or(0.0))
                    .collect();
                if values.len() == 2 {
                    let area = 0.5 * values[0] * values[1];
                    println!("Area of Rhombus: {:.2}", area);
                } else {
                    println!("Invalid input. Please enter 2 values.");
                }
            }
            "3" => {
                println!("Enter base and altitude (space-separated):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let values: Vec<f64> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap_or(0.0))
                    .collect();
                if values.len() == 2 {
                    let area = values[0] * values[1];
                    println!("Area of Parallelogram: {:.2}", area);
                } else {
                    println!("Invalid input. Please enter 2 values.");
                }
            }
            "4" => {
                println!("Enter the length of the side:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let side: f64 = input.trim().parse().unwrap_or(0.0);
                let area = 6.0 * side * side;
                println!("Area of Cube: {:.2}", area);
            }
            "5" => {
                println!("Enter radius and height (space-separated):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let values: Vec<f64> = input
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap_or(0.0))
                    .collect();
                if values.len() == 2 {
                    let volume = std::f64::consts::PI * values[0] * values[0] * values[1];
                    println!("Volume of Cylinder: {:.2}", volume);
                } else {
                    println!("Invalid input. Please enter 2 values.");
                }
            }
            "6" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
        println!(); // Print a blank line for better readability
    }
}