fn main() {
    use std::io;
    let mut nice = String::new();
    println!("Whats your height?(in cm)");
    io::stdin()
        .read_line(&mut nice)
        .expect("Unable to read value");
    let height: f32 = nice.trim().parse().expect("Give me a number!");
    if height > 150.0 && height <= 170.0 {
        println!("You're average heighted")
    } else if height > 170.0 && height <= 195.0 {
        println!("You're tall!")
    } else if height > 100.0 && height <= 150.0 {
        println!("YOu're short!")
    } else {
        println!("Abnormal Height")
    }
}
