use std::io::Write;

fn main() {
    // Create or overwrite the file for Nigerian Breweries drinks
    let mut file = std::fs::File::create("nigerian_breweries.txt").expect("Failed to create file");

    // Table data representing categories and brands[cite: 12]
    let data = "Lager | Stout | Non-Alcoholic\n\
                33 Export | Legend | Maltina\n\
                Desperados | Turbo King | Amstel Malta\n\
                Goldberg | Williams | Malta Gold\n\
                Gulder | | Fayrouz\n\
                Heineken | | \n\
                Star | | \n";

    // Write the data bytes into the file
    file.write_all(data.as_bytes()).expect("Failed to write data");

    println!("Nigerian Breweries categories successfully saved to 'nigerian_breweries.txt'.");
}