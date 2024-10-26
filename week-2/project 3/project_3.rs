fn main() {
    // The initial price of the TV in Naira
    let initial_value = 510_000.0_f64; // Using f64 for precision

    // The annual depreciation rate as a percentage
    let depreciation_rate = 5.0_f64;

    // The number of years we want to calculate depreciation for
    let years = 3;

    // Formula to calculate the depreciated value after a certain number of years
    let depreciated_value = initial_value * (1.0 - depreciation_rate / 100.0).powf(years as f64);

    // Print the result to see how much the TV is worth after 3 years
    println!(
        "Value of the TV after {} years: {:.2} Naira",
        years, depreciated_value
    );
}
