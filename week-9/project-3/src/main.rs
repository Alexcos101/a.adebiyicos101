struct Minister {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Create separate vectors for each dataset
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let geopolitical_zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Merge the datasets into a vector of Minister structs
    let mut merged_data = Vec::new();
    for i in 0..names.len() {
        merged_data.push(Minister {
            name: names[i].to_string(),
            ministry: ministries[i].to_string(),
            geopolitical_zone: geopolitical_zones[i].to_string(),
        });
    }

    // Print the merged data
    println!("S/N\tNAME OF COMMISSIONER\tMINISTRY\t\tGEOPOLITICAL ZONE");
    for (i, minister) in merged_data.iter().enumerate() {
        println!("{}\t{}\t\t{}\t\t{}", i + 1, minister.name, minister.ministry, minister.geopolitical_zone);
    }
}