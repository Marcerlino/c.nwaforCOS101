fn main() {
    println!("Welcome to the APS level checker");
    println!("Enter your name");
    let mut name = string::new();
    std::io::stdin().read_line(&mut name).expect("Please enter a valid name");
    let name = name.trim().parse().expect("Invalid inpit")
}
