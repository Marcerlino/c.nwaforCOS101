
fn main() {
    let candidates = vec![
        ("John", 3),
        ("Mary", 5),
        ("Peter", 8),
        ("David", 4),
        ("Grace", 10),
    ];

    let mut highest = candidates[0];

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    println!("Candidate with the highest experience:");
    println!("Name: {}", highest.0);
    println!("Experience: {} years", highest.1);
}
