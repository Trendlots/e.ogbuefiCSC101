fn main() {
    use std::io;
    let mut why = String::new();
    let mut now = String::new();
    println!("What is your Lower Bound");
    io::stdin().read_line(&mut why).expect("Error");
    let lb: i32 = why.trim().parse().expect("I need an Integer");
    println!("");
    println!("Now give me your Upper bound:");
    io::stdin().read_line(&mut now).expect("Error");
    let ub: i32 = now.trim().parse().expect("I need an Integer");

    for y in lb..=ub {
        println!("Your count is {}", y)
    }
}
