use std::io;

fn main() {
    // Input values of a, b, and c
    let mut input = String::new();
    println!("Enter the coefficients a, b, and c separated by spaces:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let coefficients: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap_or(0.0))
        .collect();

    if coefficients.len() != 3 {
        println!("Please provide three valid coefficients!");
        return;
    }

    let (a, b, c) = (coefficients[0], coefficients[1], coefficients[2]);

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: root1 = {}, root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The roots are real and equal: root = {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}

