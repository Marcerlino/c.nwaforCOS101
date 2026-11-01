fn main() {
    let t: (&str, f32, u8) = ("Rust", 3.14, 100);

    println!("{:?}", t);

    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);
}