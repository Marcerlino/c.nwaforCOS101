
use std::io;

fn main() {
    let jobs = vec!["Public Servant", "Office Admin", "Academic", "Lawyer", "Teacher"];

    let mut profession = String::new();
    let mut experience = String::new();

    println!("Enter your profession:");
    io::stdin().read_line(&mut profession).unwrap();

    println!("Enter your years of experience:");
    io::stdin().read_line(&mut experience).unwrap();

    let profession = profession.trim();
    let experience: i32 = experience.trim().parse().unwrap();

    let mut position = "";
    let mut level = "";

    if profession == jobs[0] {
        if experience <= 2 {
            position = "Intern";
            level = "APS 1-2";
        } else if experience <= 5 {
            position = "Administrator";
            level = "APS 3-5";
        } else if experience <= 8 {
            position = "Senior Administrator";
            level = "APS 5-8";
        } else if experience <= 10 {
            position = "Office Manager";
            level = "EL1 8-10";
        } else if experience <= 13 {
            position = "Director";
            level = "EL2 10-13";
        } else {
            position = "CEO";
            level = "SES";
        }
    }

    if profession == jobs[3] {
        if experience <= 2 {
            position = "Paralegal";
            level = "APS 1-2";
        } else if experience <= 5 {
            position = "Junior Associate";
            level = "APS 3-5";
        } else if experience <= 8 {
            position = "Associate";
            level = "APS 5-8";
        } else if experience <= 10 {
            position = "Senior Associate 1-2";
            level = "EL1 8-10";
        } else if experience <= 13 {
            position = "Senior Associate 3-4";
            level = "EL2 10-13";
        } else {
            position = "Partner";
            level = "SES";
        }
    }

    if position == "" {
        println!("Profession not available yet.");
    } else {
        println!("Profession: {}", profession);
        println!("Experience: {} years", experience);
        println!("Position: {}", position);
        println!("APS Level: {}", level);
    }
}
