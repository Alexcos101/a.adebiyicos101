fn main() {
    let x = vec![100, 200, 300];
    borrow_vector(&x);  // Passing a reference to the vector to borrow it

    println!("Printing the value from main() x[0]={}", x[0]);
    println!("****************************");
}    

fn borrow_vector(z: &Vec<i32>) {
    println!("****************************");
    for value in z.iter() {
        println!("Value in borrowed vector: {}", value);
    }
}
