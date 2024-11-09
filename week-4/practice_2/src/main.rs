fn main() {
    use std::io;
    let mut first_value = String::new();
    let mut second_value = String::new();
    let mut third_value = String::new();

    println!("Tell me something, whats the value of the first side");
    io::stdin()
        .read_line(&mut first_value)
        .expect("Unable to understand inputed value");
    let a: f32 = first_value.trim().parse().expect("I need a Number!");

    println!("Ok tell me anotherthing, whats the value of the second side");
    io::stdin()
        .read_line(&mut second_value)
        .expect("Unable to understand inputed value");
    let b: f32 = second_value.trim().parse().expect("I need a Number!");

    println!("Tell me something, whats the value of the third side");
    io::stdin()
        .read_line(&mut third_value)
        .expect("Unable to understand inputed value");
    let c: f32 = third_value.trim().parse().expect("I need a Number!");

    let s: f32 = (a + b + c) / 2.0;
    let area = s * (s - a) * (s - b) * (s - c);
    let area = area.sqrt();
    println!("So your area is {}", area);
}
