use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::process;
use std::io::{BufRead, BufReader, Result};

use chrono::{NaiveDate, NaiveDateTime};

use crate::mesure::Mesure;

mod mesure;

// stack with a lot of match Ok/Err
fn _handle_meteo_file(f: File) -> std::result::Result<(String, Vec<String>), String>{
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

fn _read_meteo_file(filename: &str){
    let file_result = File::open(filename);
    match file_result {
        Ok(f) => {
            match _handle_meteo_file(f) {
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
fn parse_lines(headers: String, lines: Vec<String>, columns: &[&str]) -> Option<Vec<Mesure>>{
    let mut column_indexes: HashMap<&str, usize> = HashMap::new();
    let header_vec: Vec<&str> = headers.split(";").collect();
    for column in columns{
        if let Some(index) = header_vec.iter().position(|header| header == column){
            column_indexes.insert(column, index);
        } else {
            // TODO: gestion d'erreur
            eprintln!("[warning] {column} not found in headers");
        }
    }
    println!("{column_indexes:?}");
    let data: Vec<HashMap<&str, &str>> = lines.iter()
        .map(|line| {
            let line_vec: Vec<&str> = line.split(";").collect();
            let mesure: HashMap<&str, &str> = column_indexes.iter()
                .map(|(col, index)| Some((*col, *line_vec.get(*index)?)))
                .collect::<Option<HashMap<_, _>>>()?;
            Some(mesure)
        })
        .collect::<Option<Vec<_>>>()?;
    println!("5 first data parsed: {:#?}", &data[..5]);
    let mesures: Vec<Mesure> = data.into_iter()
       // .filter(|map| !map.get("T").unwrap().is_empty())  // when from don't handle empty string for temperatures
        .map(Mesure::from)
        .filter(|m| !m.temperature.is_nan())
        .collect();
    Some(mesures)
}

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
    if let Some(mesures) = parse_lines(headers, lines, &["NUM_POSTE", "NOM_USUEL", "AAAAMMJJHH", "T"]){
        println!("{:#?}", &mesures[..5]);

        // exo : afficher les stations uniques triees
        let noms: BTreeSet<&str> = mesures.iter()
            .map(|m| m.nom_usuel.as_str())
            .collect();
        println!("Stations: {noms:?}");
        
        // exo : en choisir une et sortir les extrema par jours du mois de juin 2026
        let dt_min = NaiveDateTime::from(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap());
        let mesure_max = mesures.iter()
            .filter(|m| m.nom_usuel == "TOULOUSE-BLAGNAC")
            .filter(|m| m.horaire >= dt_min)
            // .max_by_key(|m| m.temperature); // KO: pas d'ordre total sur les f32
            // .max_by(|m1, m2| m1.temperature.partial_cmp(&m2.temperature).unwrap()) // OK
            .max_by_key(|m| (m.temperature * 10.) as u16)
            .unwrap();
        println!("Max: {mesure_max:?}")
    }
    Ok(())
}



fn main() {
    // _read_meteo_file("data/H_31_latest-2025-2026.csv");
    if let Err(error) = read_meteo_file_simple("data/H_31_latest-2025-2026.csv") {
        eprintln!("Error while reading csv: {error}");
    }
}
