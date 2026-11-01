fn main() {
    let mut colors = vec!["red", "blue", "green", "yellow"];

    let sliced_colors = &mut colors[1..3];

    sliced_colors[1] = "purple";

    println!("{:?}", colors);
}