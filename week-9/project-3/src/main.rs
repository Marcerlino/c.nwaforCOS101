use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    let mut file = std::fs::File::create("efcc_merged.txt").expect("failed to create");

    // Merge by index, one line per minister[cite: 14]
    for i in 0..commissioners.len() {
        let record = format!("{} | {} | {}\n", commissioners[i], ministries[i], zones[i]);
        print!("{}", record);
        file.write_all(record.as_bytes()).expect("failed to write");
    }

    println!("Merge complete!");
}