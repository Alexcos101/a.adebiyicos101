fn main() {
    let v = vec![101, 250, 350, 400];

    let v2 = v.clone(); // Create a clone of `v` to move into `v2`

    println!("{:?}", v);  // `v` is still valid because `v2` is a clone
    println!("{:?}", v2); // Print `v2`
}    
