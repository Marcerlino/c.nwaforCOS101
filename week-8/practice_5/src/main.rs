use std::io;

fn main() {
    let mut city: Vec<String> = Vec::new();

    for _ in 0..3 {
        let mut new_city = String::new();

        println!("Enter a city:");
        io::stdin().read_line(&mut new_city).unwrap();

        city.push(new_city.trim().to_string());
    }

    println!("{:?}", city);
}