fn main() {
    use std::io;
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Whats Your name:");
    io::stdin().read_line(&mut input1).expect("Invalid Input");
    println!();
    println!("Now tell me, How old are you exactly?");

    io::stdin().read_line(&mut input2).expect("Invalid INput");
    let age: i32 = input2.trim().parse().expect("Input an integer please");

    let illegal_age = 18 - age;
    if age >= 18 {
        println!("Congrats You're Old Enough")
    } else {
        println!();
        print!(
            "Sorry {}You'll have to wait {} more years to be allowed",
            input1, illegal_age
        );
    }
}
