use std::io;

fn main() {
    // Get experience input
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Get age input
    let mut age = String::new();
    println!("Enter the age of the employee: ");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid age");

    // Determine incentive based on experience and age
    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 && age < 40 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0 // Default case for any age not in criteria
        }
    } else {
        100_000
    };

    println!("The annual incentive for the employee is: N{}", incentive);
}
