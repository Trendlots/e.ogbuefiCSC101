fn main() {
    let num: i32 = 5;
    seven(num);
    println!("The value of no is: {}", num);
}

fn seven(mut param_num: i32) {
    param_num = param_num * 0;
    println!("Param_num value is: {}", param_num);
}
