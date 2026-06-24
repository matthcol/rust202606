use flightplan::FlightPlan;

use crate::geometry::Point;

mod flightplan;
mod geometry;

fn play_with_flights() {
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

fn play_with_points(){
    let pt1 = Point{x: 3.5, y: 13.25, name: String::from("A")};
    let pt2 = Point{x: 7.5, y: 10.25, name: String::from("B")};
    let d = pt1.distance(&pt2);
    println!("Distance between {pt1:?} and {pt2:?} = {d}")
}

fn main(){
    play_with_flights();
    play_with_points();
}