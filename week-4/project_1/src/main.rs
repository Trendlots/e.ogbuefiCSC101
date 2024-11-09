fn main() {
    use std::io;
    let mut q = String::new();
    let mut w = String::new();
    let mut e = String::new();

    println!("What is the value of a");
    io::stdin().read_line(&mut q).expect("Error");
    let mut a: f64 = q.trim().parse().expect("Give me an integer");

    println!("");
    println!("What is the value of b:");
    io::stdin().read_line(&mut w).expect("Error");
    let mut b: f64 = w.trim().parse().expect("Give me an integer");

    println!("");
    println!("What is the value of c:");
    io::stdin().read_line(&mut e).expect("Error");
    let mut c: f64 = e.trim().parse().expect("Give me an integer");
    let discri = b.powf(2.0) - 4.0 * a * c;

    if discri >= 0.0 {
        let answer_1 = (-b + (discri).sqrt()) / (2.0 * a);
        let answer_2 = (-b - (discri).sqrt()) / (2.0 * a);

        println!("Your answer is {} and {}", answer_1, answer_2);
    } else {
        println!("No real roots for the equation")
    }
}
