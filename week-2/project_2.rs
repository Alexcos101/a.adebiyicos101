fn main() {
    let sales = vec![
        ("Toshiba", 450_000.0),
        ("Mac", 1_500_000.0),
        ("HP", 750_000.0),
        ("Dell", 2_850_000.0),
        ("Acer", 250_000.0),
    ];

    let mut total: f64 = 0.0;

    for (_, amount) in &sales {
        total += amount;
    }

    let average = total / sales.len() as f64;

    println!("Total Sales: ₦{:.2}", total);
    println!("Average Sales: ₦{:.2}", average);
}