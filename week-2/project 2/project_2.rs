fn main() {
    // Array holding the sales amounts in Naira
    let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    // Adding up all the sales amounts to get the total
    let sum: f64 = amounts.iter().sum();

    // Calculating the average by dividing the total by the number of items
    let average = sum / amounts.len() as f64;

    // Printing the total sales amount with 2 decimal places
    println!("Total Sales Amount: {:.2} Naira", sum);

    // Printing the average sales amount with 2 decimal places
    println!("Average Sales Amount: {:.2} Naira", average);
}
