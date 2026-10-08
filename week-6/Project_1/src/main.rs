use std::io;

fn main() {
    println!("           Restaurant Menu     ");
    println!(" P  - Poundo Yam / Edinkaiko Soup - N3200");
    println!(" F  - Fried Rice & Chicken        - N3000");
    println!(" A  - Amala & Ewedu Soup          - N2500");
    println!(" E  - Eba & Egusi Soup            - N2000");
    println!(" W  - White Rice & Stew           - N2500");
    println!();
    println!("Get a 5% discount for purchases above N10000 NOW");
    println!();
    println!("Enter your food choice:");

    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed too read input");
    let food = food.trim().to_uppercase();


    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");

    let quantity: i32 = quantity.trim().parse().expect("Please enter a valid number");

    let price:i32;

    if food == "P" {
        price = 3200;
    } else if food == "F" {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "E" {
        price = 2000;
    } else if food == "W" {
        price = 2500;
    } else {
        println!("Invalid food choice");
        return;
    }

    let total = price * quantity;

    println!("Total = N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        let amount = total - discount;

        println!("Discount = N{}", discount);
        println!("Amount to pay = N{}", amount);
        println!("Thank you for making a purchase");
    } else {
        println!("No discount ");
        println!("Amount to pay = N{}", total);
        println!("Thank you for making a purchase");
    }
}