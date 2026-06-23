use euclide::{gcd, gcd_ref};
use tools::swap;

use crate::euclide::{gcd_option, gcd_result};

mod euclide;
mod tools;



fn main() {
    let mut a = 21;
    let mut b = 15;
    // let g = euclide::gcd(a, b); // without "use"
    let g = gcd(a, b);
    let g2 = gcd_ref(&a, &b);
    println!("Gcd of {a} and {b} is {g}");
    println!("Gcd of {a} and {b} is {g2}");
    swap(&mut a, &mut b);
    println!("After swap: a = {a}, b = {b}");

    // panic case:
    // let g3 = gcd(a, 0);
    // println!("Gcd of {a} and 0 is {g3}");

    let mut opt_g = gcd_option(a, b);
    println!("Gcd of {a} and {b} is {opt_g:?}");
    opt_g = gcd_option(a, 0);
    println!("Gcd of {a} and 0 is {opt_g:?}");

    for c in 0..3 {
        println!("Case [option] c = {c}");
        opt_g = gcd_option(a, c);
        if let Some(g4) = opt_g {
            println!("\tSuccess (if): gcd of {a} and {c} is {g4}");
        }
        match opt_g {
            Some(g5) => {
                println!("\tSuccess (match): gcd of {a} and {c} is {g5}");
            },
            None => {
                println!("\tNo gcd is defined for {a} and {c} (match)");
            }
        }
    }

    for c in 0..3 {
        println!("Case [result] c = {c}");
        // let res_g = gcd_result(a, c);
        match gcd_result(a, c) {
            Ok(g6) => println!("\tSuccess (result): gcd of {a} and {c} is {g6}"),
            Err(msg) => println!("\tNo gcd is defined for {a} and {c} (match): {msg}")
        } 
    } 
}
