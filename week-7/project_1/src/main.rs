use std::io;

fn main() {
    loop {
        println!("Select a calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        };

        match choice {
            1 => calculate_trapezium_area(),
            2 => calculate_rhombus_area(),
            3 => calculate_parallelogram_area(),
            4 => calculate_cube_area(),
            5 => calculate_cylinder_volume(),
            6 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn calculate_trapezium_area() {
    let height = get_input("Enter the height: ");
    let base1 = get_input("Enter the first base: ");
    let base2 = get_input("Enter the second base: ");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

fn calculate_rhombus_area() {
    let diagonal1 = get_input("Enter the first diagonal: ");
    let diagonal2 = get_input("Enter the second diagonal: ");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area);
}

fn calculate_parallelogram_area() {
    let base = get_input("Enter the base: ");
    let altitude = get_input("Enter the altitude: ");
    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}

fn calculate_cube_area() {
    let side = get_input("Enter the length of the side: ");
    let area = 6.0 * side * side;
    println!("The area of the cube is: {}", area);
}

fn calculate_cylinder_volume() {
    let radius = get_input("Enter the radius: ");
    let height = get_input("Enter the height: ");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume);
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}  
