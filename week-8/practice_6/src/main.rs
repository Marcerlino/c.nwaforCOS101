fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let x = vec![6, 7, 8, 9, 10];

    for index in 0..v.len() {
        let sum = v[index] + x[index];

        println!("{}", sum);
    }
}