fn main() {
    let v = vec!['A', 'B', 'C', 'D'];

    let index = 2;

    if index < v.len() {
        let ch: char = v[index];
        println!("Character: {}", ch);
    }
}