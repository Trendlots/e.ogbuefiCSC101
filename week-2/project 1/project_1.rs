
fn main() {
    let principal: f64 = 520_000_000.0; // Principal amount in Naira
    let rate: f64 = 10.0;               // Interest rate per annum in percent
    let years: u32 = 5;                 // Time period in years

    // Compound interest formula
    let amount = principal * (1.0 + rate / 100.0).powf(years as f64);
    let compound_interest = amount - principal;

    // Display the results
    println!("Total Amount after {} years: {:.2} Naira", years, amount);
    println!("Compound Interest: {:.2} Naira", compound_interest);
}