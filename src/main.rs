fn banner() { 
    const BANNER: &str = include_str!("banner.txt");
    print!("{BANNER}");
}

fn main() {
    // Print out 1337 banner
    banner();
    
    println!("Pestering user for save...");

    // Ask for path to file
    let Some(file) = rfd::FileDialog::new()
        .add_filter("Save file", &["db"])
        .set_directory(".")
        .pick_file() else { 
            panic!("Bruh");
        };

    println!("Opening file at path {}", file.display());
    println!("Bypassing firewall... Done");
    println!("Reticulating splines... Done");
    println!("Defragulating papacy.... Done");

    // Open connection to DB file
    let Ok(conn) = rusqlite::Connection::open(file) else { 
        panic!("Bruh: could not connect to file");
    };

    let balance: f64 = 690_000_000_000.0;

    if let Ok(_) = conn.execute(
        "update Balance set Balance = (?1)",
        (&balance,)
    ) { 
        println!("Balance updated successfully");
    } else {
        println!("Balance could not be updated :c");
    }
}
