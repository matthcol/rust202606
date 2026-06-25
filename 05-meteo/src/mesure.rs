use std::collections::HashMap;

#[derive(Debug)]
pub struct Mesure{
    pub num_poste: String,
    pub nom_usuel: String,
    pub horaire: String, // TODO
    pub temperature: f32
}

impl From<HashMap<&str, &str>> for Mesure {
    fn from(value: HashMap<&str, &str>) -> Self {
        Mesure{
            num_poste: value.get("NUM_POSTE").unwrap().to_string(),
            nom_usuel: value.get("NOM_USUEL").unwrap().to_string(),
            horaire: value.get("AAAAMMJJHH").unwrap().to_string(), // TODO: convert into temporal type
            temperature: value.get("T").unwrap().parse().unwrap()
        }
    }
}