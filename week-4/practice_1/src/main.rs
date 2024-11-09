use std::io;
fn main() {
    println!("\nStudent Information Managememt System!");

    //input name
    println!("\nPlease Enter Your Name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read try again");
    println!("So Your name is {}", name);

    //input age
    println!("\nPlease input your age");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to Process your name");
    let age: i32 = age
        .trim()
        .parse()
        .expect("Use Your head and Input an Integer");
    println!("\nSo you're {} years old, huh?, nice!", age);
}
