fn main () {
    Let p:f64 = 520,000,000.0;
    Let r:f64 = 10.0;
    Let n:f64 = 5.0;

    // compound interest
    Let a = p * ( 1.0 + (r / 100)) * n;
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compound Interest is {}", ci);

}
