fn main() {
    let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array: {:?}", colors);

    let mut sliced_colors = &mut colors[1..3]; // Correct: Use &mut for mutable slice
    println!("First slice: {:?}", sliced_colors);

    sliced_colors[0] = "purple"; // Correct: Index 0 refers to the first element in the slice
    println!("Changed slice: {:?}", sliced_colors);

    println!("Original array after slice modification: {:?}", colors); // Observe the change in the original array
}
