fn main() {
    let x = (20, true, 3.75);

    let (age, is_male, cgpa) = x;

    println!("Age is {}", age);
    println!("Is male: {}", is_male);
    println!("CGPA is {}", cgpa);
}