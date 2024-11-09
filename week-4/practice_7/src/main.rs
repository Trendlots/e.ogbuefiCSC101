fn main() {
    use std::io;
    let mut you = String::new();
    println!("Give me a Number");
    io::stdin().read_line(&mut you).expect("Error");
    let mut nice: i32 = you.trim().parse().expect("Give me an Integer");
    while nice < 10 {
        println!("The count is {}", nice);
        nice += 1;
    }
    println!("Outside limits reached")
}
