use std::io;

fn checker() {
    let mut input = String::new();

    println!("Enter a character:");
    io::stdin().read_line(&mut input).unwrap();

    let ch = input.trim().chars().next().unwrap();

    if ch >= '0' && ch <= '9' {
        println!("It is a digit");
    } else {
        println!("It is not a digit");
    }
}

fn main() {
    checker();
}