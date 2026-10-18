
use std::io;

fn main() {
    println!("SHAPE CALCULATOR");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();

    println!("Enter your choice:");
    io::stdin().read_line(&mut choice).unwrap();

    let choice: i32 = choice.trim().parse().unwrap();

    if choice == 1 {
        let answer = trapezium();
        println!("Area of trapezium = {}", answer);
    } else if choice == 2 {
        let answer = rhombus();
        println!("Area of rhombus = {}", answer);
    } else if choice == 3 {
        let answer = parallelogram();
        println!("Area of parallelogram = {}", answer);
    } else if choice == 4 {
        let answer = cube();
        println!("Surface area of cube = {}", answer);
    } else if choice == 5 {
        let answer = cylinder();
        println!("Volume of cylinder = {}", answer);
    } else {
        println!("Invalid choice");
    }
}

fn trapezium() -> f64 {
    let mut input = String::new();

    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter first base:");
    io::stdin().read_line(&mut input).unwrap();
    let base1: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter second base:");
    io::stdin().read_line(&mut input).unwrap();
    let base2: f64 = input.trim().parse().unwrap();

    height / 2.0 * (base1 + base2)
}

fn rhombus() -> f64 {
    let mut input = String::new();

    println!("Enter first diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d1: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter second diagonal:");
    io::stdin().read_line(&mut input).unwrap();
    let d2: f64 = input.trim().parse().unwrap();

    0.5 * d1 * d2
}

fn parallelogram() -> f64 {
    let mut input = String::new();

    println!("Enter base:");
    io::stdin().read_line(&mut input).unwrap();
    let base: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    base * height
}

fn cube() -> f64 {
    let mut input = String::new();

    println!("Enter side of cube:");
    io::stdin().read_line(&mut input).unwrap();
    let side: f64 = input.trim().parse().unwrap();

    6.0 * side * side
}

fn cylinder() -> f64 {
    let mut input = String::new();

    println!("Enter radius:");
    io::stdin().read_line(&mut input).unwrap();
    let radius: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height: f64 = input.trim().parse().unwrap();

    3.142 * radius * radius * height
}
