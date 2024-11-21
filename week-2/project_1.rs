fn main() {
    let principal: f64 = 520_000_000.0; // Principal amount
    let rate: f64 = 10.0; // Annual interest rate in percentage
    let years: u32 = 5; // Number of years

    // Compound interest formula: A = P[1 + (R/100)]^n
    let amount = principal * (1.0 + (rate / 100.0)).powf(years as f64);
    let compound_interest = amount - principal;

    println!("Compound Interest: ₦{:.2}", compound_interest);
    println!("Total Amount: ₦{:.2}", amount);
}