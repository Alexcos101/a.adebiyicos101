fn main() {
    let initial_value: f64 = 510_000.0; // Initial cost of the TV
    let depreciation_rate: f64 = 5.0; // Annual depreciation rate in percentage
    let years: u32 = 3; // Number of years

    // Depreciation formula: A = P[1 - (R/100)]^n
    let depreciated_value = initial_value * (1.0 - (depreciation_rate / 100.0)).powf(years as f64);

    println!("Depreciated Value after {} years: ₦{:.2}", years, depreciated_value);
}