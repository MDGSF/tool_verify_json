use std::{fs, io};
use clap::Parser;

/// Verify json is valid or not
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify one json filename
    #[arg(short, long)]
    file: Option<String>,

    /// Specify one directory include json files
    #[arg(short, long)]
    dir: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(file) = args.file {
        process_file(&file);
    } else if let Some(directory) = args.dir {
        process_dir(&directory)?;
    }

    Ok(())
}

fn process_file(json_file: &str) {
    if !std::path::Path::new(json_file).is_file() {
        println!("No such file");
        return;
    }
    let data = fs::read_to_string(json_file).expect("Error in reading the file");
    match json::parse(&data) {
        Ok(_) => print!("true"),
        Err(_) => print!("false"),
    }
}

fn process_dir(directory: &str) -> io::Result<()> {
    if !std::path::Path::new(&directory).is_dir() {
        println!("No such directory");
        std::process::exit(0);
    }

    let mut entries = fs::read_dir(directory)?
        .map(|e| e.unwrap().path())
        .filter(|p| {
            match p.extension() {
                Some(ext) => ext == "json",
                None => false
            }
        })
        .collect::<Vec<_>>();

    entries.sort();

    for entry in entries.iter() {
        let data = fs::read_to_string(entry).expect("Error in reading the file");
        match json::parse(&data) {
            Ok(_) => println!("{:?}: true", entry.file_name().unwrap()),
            Err(_) => println!("{:?}: false", entry.file_name().unwrap()),
        }
    }
    Ok(())
}
