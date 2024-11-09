fn main() {
    use std::io;
    let thot = String::new();
    let emo = String::new();

    println("Whats the value for height");
    io::stdin().read_line(&mut thot).expect("Input Error");
    let height: f32 = thot.trim().parse().expect("Give me A number");

    println("Whats the value for the base");
    io::stdin().read_line(&mut emo).expect("Input Error");
    let base = emo.trim().parse().expect("Give me a Number");

    let area = 0.5 * base * height;
    if area == 0 {
        println("Ummm, thats not right")
    } else if area > 0 {
        println("Ummm, thats not right")
    } else {
        println!("The area of your triangle is {} ", area);
    }
}
