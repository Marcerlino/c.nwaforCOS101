use std::io;

fn main() {

    let mut input1 = String::new();

    let mut input2 = String::new();

    println!("Are you experienced? (yes/no): ");

    io::stdin().read_line(&mut input1).expect("Not a valid string");

    let experience = input1.trim(); // Keeps it as a simple string comparison

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age: i32 = input2.trim().parse().expect("Not a valid number");

   
    if experience == "yes" && age >= 40 {
        println!("Annual Incentive = \u{20A6}1,560,000");
    } else if experience == "yes" && age >= 30 && age <= 39 {
        println!("Annual Incentive = \u{20A6}1,480,000");
    } else if experience == "yes" && age < 28 {
        println!("Annual Incentive = \u{20A6}1,300,000");
    } else if experience == "no" {
        println!("Annual Incentive = \u{20A6}100,000");
    } else {
        println!("Annual Incentive = \u{20A6}100,000"); 
    }
}