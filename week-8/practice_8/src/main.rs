fn main() {
    let mut mountain_heights = ("Everest", "K2", "Kangchenjunga");

    mountain_heights.2 = "Lhotse";

    println!("{:?}", mountain_heights);
}