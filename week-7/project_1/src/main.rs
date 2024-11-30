use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    (diagonal1 * diagonal2) / 2.0
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    3.14159 * radius.powi(2) * height
}

fn main() {
    println!("Welcome to the Multi-Equation Calculator!");
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("Enter your choice (1-5):")
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid number");

    match choice {
        1 => {
            let height = read_input("Enter the height:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            let base1 = read_input("Enter base1:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            let base2 = read_input("Enter base2:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            println!(
                "The area of the trapezium is: {:.2}",
                area_of_trapezium(height, base1, base2)
            );
        }
        2 => {
            let diagonal1 = read_input("Enter diagonal1:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            let diagonal2 = read_input("Enter diagonal2:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            println!(
                "The area of the rhombus is: {:.2}",
                area_of_rhombus(diagonal1, diagonal2)
            );
        }
        3 => {
            let base = read_input("Enter the base:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            let altitude = read_input("Enter the altitude:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            println!(
                "The area of the parallelogram is: {:.2}",
                area_of_parallelogram(base, altitude)
            );
        }
        4 => {
            let side = read_input("Enter the side length of the cube:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            println!("The area of the cube is: {:.2}", area_of_cube(side));
        }
        5 => {
            let radius = read_input("Enter the radius of the cylinder:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            let height = read_input("Enter the height of the cylinder:")
                .trim()
                .parse::<f64>()
                .expect("Invalid number");
            println!(
                "The volume of the cylinder is: {:.2}",
                volume_of_cylinder(radius, height)
            );
        }
        _ => println!("Invalid choice! Please restart the program."),
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
