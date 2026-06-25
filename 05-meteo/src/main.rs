use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader, Result};

// stack with a lot of match Ok/Err
fn handle_meteo_file(f: File) -> std::result::Result<(String, Vec<String>), String>{
    let mut buf = BufReader::new(f);
    let mut headers = String::new();
    if let Ok(_size) = buf.read_line(&mut headers){
        // println!("Read {size} bytes");
        let lines: Vec<String> = buf.lines()
            .map(|r| r.unwrap())
            .collect();
        Ok((headers, lines))
    } else {
        Err(String::from("Error while reading file"))
    }
} // file is closed

fn read_meteo_file(filename: &str){
    let file_result = File::open(filename);
    match file_result {
        Ok(f) => {
            match handle_meteo_file(f) {
                Ok((headers, lines)) => {
                    println!("File read with sucess");
                    println!("Headers: {headers}");
                    println!("First line: {:?}", lines.get(0));
                    println!("Data count: {}", lines.len());
                },
                Err(msg) => eprintln!("{msg}")
            }
            // println!("{f:?}") // f borrowed
        },
        Err(error) => {
            println!("{error}");
            process::exit(1)
        }
    }
}

// Stack avec propagation d'erreur au niveau de chaque fonction
fn handle_meteo_file_simple(f: File) -> Result<(String, Vec<String>)>{
    let mut buf = BufReader::new(f);
    let mut headers = String::new();
    let size = buf.read_line(&mut headers)?;
    println!("Read {size} bytes");
    let lines: Vec<String> = buf.lines()
        .collect::<Result<Vec<_>>>()?;
    Ok((headers, lines))
} // file is closed

fn read_meteo_file_simple(filename: &str) -> Result<()>{
    let file_result = File::open(filename)?;
    let (headers, lines) = handle_meteo_file_simple(file_result)?;
    println!("File read with sucess");
    println!("Headers: {headers}");
    println!("First line: {:?}", lines.get(0));
    println!("Data count: {}", lines.len());
    Ok(())
}

fn main() {
    read_meteo_file("data/H_31_latest-2025-2026.csv");
    if let Err(error) = read_meteo_file_simple("data/H_31_latest-2025-2026.csv") {
        eprintln!("Error while reading csv: {error}");
    }
}
