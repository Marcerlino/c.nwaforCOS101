fn print(x: (i32, bool, f64)) {
    println!("{:?}", x);
}

fn main() {
    let b = (20, true, 3.75);

    print(b);
}