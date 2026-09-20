// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    println!("\n--- Quadratic Roots Calculator ---");

    // 1. Read input for coefficient 'a'
    println!("Enter value for a: ");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");
    let a: f64 = input_a.trim().parse().expect("Please enter a valid number");

    // 2. Read input for coefficient 'b'
    println!("Enter value for b: ");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");
    let b: f64 = input_b.trim().parse().expect("Please enter a valid number");

    // 3. Read input for coefficient 'c'
    println!("Enter value for c: ");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let c: f64 = input_c.trim().parse().expect("Please enter a valid number");

    // 4. Calculate the discriminant
    let d = (b * b) - (4.0 * a * c);
    println!("\nThe discriminant (d) is: {}", d);

    // 5. Determine the roots based on the discriminant value
    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has exactly one real root:");
        println!("Root = {}", root);
    } else {
        println!("The equation has no real roots (the roots are imaginary).");}
    }
    