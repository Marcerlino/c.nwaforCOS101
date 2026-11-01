use std::io::Write;

fn main() {
    // Read from the vector of student details[cite: 13]
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("students.txt").expect("file error");

    // Display on screen and write to file[cite: 13]
    for i in 0..names.len() {
        let line = format!("{} | {} | {} | {}\n", names[i], matric[i], dept[i], level[i]);
        print!("{}", line);
        file.write_all(line.as_bytes()).expect("write error");
    }

    println!("Finished saving students to file.");
}