fn main() {
    // Initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    // Change elements of the mutable tuple at index 2 and 3
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("Changed tuple = {:?}", mountain_heights);
}