fn main() {
    let name = vec!["John", "Mary", "Peter"];
    let age = vec![18, 20, 22];

    for i in 0..age.len() {
        println!("{} is {} years old", name[i], age[i]);
    }
}