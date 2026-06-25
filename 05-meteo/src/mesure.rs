use std::collections::HashMap;

use chrono::{NaiveDateTime, ParseResult};

#[derive(Debug)]
pub struct Mesure{
    pub num_poste: String,
    pub nom_usuel: String,
    pub horaire: NaiveDateTime, 
    pub temperature: f32
}

fn parse_tag_datetime(tag: &str) -> ParseResult<NaiveDateTime>{
    let datetime_str = format!("{tag}00");
    let dt = chrono::NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%d%H%M")?;
    // println!("{dt:?}");
    Ok(dt)
}

impl From<HashMap<&str, &str>> for Mesure {
    fn from(value: HashMap<&str, &str>) -> Self {
        Mesure{
            num_poste: value.get("NUM_POSTE").unwrap().to_string(),
            nom_usuel: value.get("NOM_USUEL").unwrap().to_string(),
            horaire: parse_tag_datetime(&value.get("AAAAMMJJHH").unwrap().to_string()).unwrap(),
            temperature: value.get("T").unwrap().parse().unwrap_or(f32::NAN)
        }
    }
}