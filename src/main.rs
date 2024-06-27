use bdb_parser::parsers::parse_track_database;

use std::env;
use std::path::Path;
use std::process;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_bdb_file>", args[0]);
        process::exit(1);
    }

    let file_path = Path::new(&args[1]);

    if !file_path.exists() {
        eprintln!("Error: File '{}' does not exist.", file_path.display());
        process::exit(1);
    }

    if file_path.extension().and_then(|s| s.to_str()) != Some("BDB") {
        eprintln!("Warning: File does not have a .BDB extension. Continuing anyway.");
    }

    let data = std::fs::read(file_path)?;

    match parse_track_database(&data) {
        Ok((_, track_database)) => {
            // println!("Total regions: {}", track_database.regions.len());
            // println!(
            //     "Total tracks: {}",
            //     track_database
            //         .regions
            //         .iter()
            //         .map(|r| r.tracks.len())
            //         .sum::<usize>()
            // );
            let json = serde_json::to_string_pretty(&track_database)?;
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("Error parsing file: {:?}", e);
            process::exit(1);
        }
    }

    Ok(())
}
