use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::process;
use std::env;

use flightplan::FlightPlan;

use crate::flightplan::FlightLevelParity;
use crate::geometry::Point;

mod flightplan;
mod geometry;

fn play_with_flights_1() {
    let fp1  = FlightPlan{
        adep: String::from("LFBO"),
        ades: String::from("LFST"),
        fl: 300
    };
    let fp2 = FlightPlan::new("LFST", "LFBO",  310);
    let fp3 = FlightPlan::from(("LFPG", "LFBO",  350));
    let fp4 = FlightPlan::from(("LFPG", "FMEE"));
    let mut fp5: FlightPlan = ("FMEE", "FAAA", 450).into();
    fp5 += 20;
    let fp6: FlightPlan = ("FAAA", "KLAX").into();
    // fp6 += 20; // not mutable
    let flightplans = vec![fp1, fp2, fp3, fp4, fp5, fp6]; 
    // NB vecteur flightplans prend possesion des 2 flightplans (borrow)
    // println!("{fp1:?}"); // borrowed
    // println!("{fp2:?}"); // borrowed
    println!("{flightplans:?}");
    for fp in &flightplans{ // NB: mode slice
        println!("Flight plan: {fp:?}");
        println!("- from: {}", fp.adep);
        println!("- to: {}", fp.ades);
        println!("- level: {}", fp.fl)
    }
    println!("{flightplans:?}"); // NB: OK si parcours en mode slice

    let mut flight_levels: Vec<u16> = Vec::new();
    for fp in &flightplans{
        flight_levels.push(fp.fl);
    }
    println!("Flight levels: {flight_levels:?}");

    for fp in &flightplans{
        println!("Flight plan (Display): {fp}");
        println!("Flight plan (ToString): {}", fp.to_string()); // NB: Display => ToString
    }
}

fn display_flight_plans(slice_flight_plan: &[FlightPlan], intro: &str){
    println!("{} [{}]", intro, slice_flight_plan.len());
    for fp in slice_flight_plan{ // &vec_flightplan => slice sur tout le vector
        println!(" - {fp}")
    }
}

fn play_with_flight_2_collection(){
    let vec_flightplan = vec![
        FlightPlan{adep: String::from("LFBO"), ades: String::from("LFST"), fl: 300},
        FlightPlan::new("LFST", "LFBO",  310),
        FlightPlan::from(("LFPG", "LFBO",  350)),
        FlightPlan::from(("LFPG", "FMEE")),
        ("FMEE", "FAAA", 450).into(),
        ("FAAA", "KLAX").into(),
        FlightPlan::new("WXYZ", "ZYXW", 1234),
    ];
    println!("{vec_flightplan:#?}"); // pretty debug display
    // Vec => index + access O(1)
    // - get by index => Option<FP>
    let fp = vec_flightplan.get(3).unwrap();
    println!("FP #3: {fp}");
    // - slices
    display_flight_plans(vec_flightplan.as_slice(), "All Flight Plans");
    display_flight_plans(&vec_flightplan, "All Flight Plans (2)");
    display_flight_plans(&vec_flightplan[..], "All Flight Plans (3)");
    display_flight_plans(&vec_flightplan[..3], "First 3");
    display_flight_plans(&vec_flightplan[3..5], "Flight plans from #3 to #4");
    display_flight_plans(&vec_flightplan[5..], "Flight plans from #5");
    display_flight_plans(&vec_flightplan[6..], "Flight from #6");
    display_flight_plans(&vec_flightplan[3..3], "Empty slice at index #3");
    // - map/filter/reduce
    let adeps: BTreeSet<&str> = vec_flightplan.iter()
        .filter(|fp| fp.fl < 400)
        .map(|fp| fp.adep.as_str())
        .collect();
    println!("Departure Airports with flight level under 400: {adeps:?}");
    let fp_under_300: Vec<(usize, &FlightPlan)> =vec_flightplan.iter()
        .enumerate()
        // .filter(|i_fp| i_fp.1.fl < 300)
        .filter(|(_, fp)| fp.fl < 300)
        .collect();
    println!("Flight plans with fl under 300: {fp_under_300:?}");

    let fp_under_300_ii: Vec<(usize, usize, &FlightPlan)> = vec_flightplan.iter()
        .enumerate()
        .filter(|(_, fp)| fp.fl < 300)
        .enumerate()
        .map(|(index_new, (index_orig, fp))| (index_new, index_orig, fp))  // => (index_new, index_orig, FP)
        .collect();
    println!("Flight plans with fl under 300: {fp_under_300_ii:?}");

    // TODO: somme des flight level
    let total_fl: u32 = vec_flightplan.iter()
        .map(|fp| fp.fl as u32)
        .sum();
        // .sum::<u16>();
    println!("Total fl: {total_fl}");
    for airport in ["LFPG", "XXXX"]{
        let opt_min_fp = vec_flightplan.iter()
            .filter(|fp| fp.adep == airport)
            .min_by_key(|fp| fp.fl);
        if let Some(min_fp) = opt_min_fp {
            println!("Min FP from {airport}: {min_fp}")
        } else {
            println!("No FP from {airport}")
        }
    }

    vec_flightplan.iter()
        .for_each(|fp| println!("{} => {:?}", fp, fp.flight_level_parity()));   
    
    let parities: Vec<FlightLevelParity> = vec_flightplan.iter()
        .map(FlightPlan::flight_level_parity)
        .collect();
    println!("FL parities: {parities:?}");

    let fp_odd: Vec<&FlightPlan> = vec_flightplan.iter()
        .filter(|fp| fp.is_fl_parity_odd())
        .collect();
    println!("{fp_odd:?}");

    // NB: ok but last iteration => borrowed elements from vector
    // let fp_odd: Vec<FlightPlan> = vec_flightplan.into_iter()
    //     .filter(FlightPlan::is_fl_parity_odd)
    //     .collect();
    // println!("{vec_flightplan:?}")

    // Exo: 
    // - map1 : FP => level_parity
    // - map2 : level_parity => liste des FP

    let mut parity_fps: HashMap<FlightLevelParity, Vec<&FlightPlan>> = HashMap::new();
    for fp in &vec_flightplan{
        parity_fps.entry(fp.flight_level_parity())
            .or_default()
            .push(fp)
    }
    println!("{parity_fps:#?}");

    let mut fps: Vec<&FlightPlan> = vec_flightplan.iter().collect();
    fps.sort_by_key(|fp| fp.flight_level_parity());
    println!("{fps:?}")
    
}

fn play_with_geometry_1(){
    let pt1 = Point{x: 3.5, y: 13.25, name: Some(String::from("A"))};
    let pt2 = Point{x: 7.5, y: 10.25, name: Some(String::from("B"))};
    let pt3 = Point{x: 7.5, y: 13.25, name: None};
    let points = vec![pt1, pt2, pt3];
    let pt_ref = points.get(0).unwrap();
    for pt in &points{
        let d = pt.distance(pt_ref);
        println!("- {pt:?}");
        println!("\t* name: {:?}", pt.name); // pt.name.unwrap_or(String::from("?")));
        println!("\t* distance to {pt_ref:?}: {d}");
    }
}

fn play_with_simple_maps(){
    let mut day_note: BTreeMap<&str, u8> =  BTreeMap::new();
    day_note.insert("MONDAY", 12);
    day_note.insert("TUESDAY", 20);
    day_note.insert("WEDNESDAY", 15);
    println!("{day_note:?}");
    day_note.insert("MONDAY", 17); // replace
    println!("{day_note:?}");

    let day_note2: BTreeMap<&str, u8> = BTreeMap::from([
        ("MONDAY", 3),
        ("TUESDAY", 20),
        ("WEDNESDAY", 8),
        ("THURSDAY", 13),
        ("FRIDAY", 17),
        ("SATURDAY", 8),
        ("SUNDAY", 15),
    ]);
    println!("{day_note2:?}");

    let note = day_note2.get("MONDAY").unwrap(); // I'm sure
    println!("Note: {note}");

    for (day, note) in day_note2.iter(){ // iteration sur les tuples (k,v)
        println!("{day}: {note}")
    }

    // other iterations:
    // - day_note2.key()
    // - day_note2.values()
}

fn main(){
    let args: Vec<String> = env::args()
        .collect();
    // println!("{args:?}");
    match args.get(1).map(String::as_str){
        Some("flight1") => {
            println!("scenario flight plan 1");
            play_with_flights_1();
        },
        Some("flight2") => {
            println!("scenario flight plan 2: collections");
            play_with_flight_2_collection()
        }
        Some("geom1") => {
            println!("scenario geometry 1");
            play_with_geometry_1();
        },
        Some("map") => {
            println!("scenario map");
            play_with_simple_maps();
        },
        Some(scenario) => {
            eprintln!("[ERROR] scenario not handled: {scenario}");
            process::exit(1)
        },
        None => {
            eprintln!("[ERROR] no scenario was given");
            process::exit(2)
        }
    }
}