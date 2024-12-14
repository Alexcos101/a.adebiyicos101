fn main() {
    let b: (i32, bool, f64) = (110, true, 10.9);
    print_tuple(b); // Call the function with tuple as a parameter
}

// Function to print tuple
fn print_tuple(x: (i32, bool, f64)) {
    println!("Inside print_tuple method");
    println!("{:?}", x);
}