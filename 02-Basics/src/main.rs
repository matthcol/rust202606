fn play_with_text(){
    println!("***** Texts ******");
    let city = "Toulouse";  // reference non mutable
    println!("My city is {city}");
    println!("My city is {}", city);
    println!("My city is {0}", city);
    println!("My city is : {0}, {0:?}", city);

    // cannot mutate immutable variable `city`
    // city = "Pau"

    let mut city_travel = "Pau";
    println!("Start at: {city_travel}");
    city_travel = "Toulouse";
    println!("Arrived at: {city_travel}");

    // a str object is immutable, mutable reference can be changed
    // city_travel[0] = 'B';
    
    // Unicode strings
    city_travel = "東京";
    println!("Holidays in: {city_travel}");

    println!("Nb of chars: {}", city_travel.chars().count());
    println!("Nb of bytes: {}", city_travel.len());

    println!("Chart at #0: {:?}", city_travel.chars().nth(0));
    println!("Chart at #1: {:?}", city_travel.chars().nth(1));
    println!("Chart at #2: {:?}", city_travel.chars().nth(2));

    for index in [1, 3] {
        let opt_char = city_travel.chars().nth(index);
        if let Some(char_i) = opt_char { // pattern matching
            println!("Char found at index {index}: {char_i}")
        } else { // None
            println!("No char found at index {index} (out of bounds)")
        }
    }

    println!("Iteration on chars:");
    for char_i in city_travel.chars(){
        println!("- {char_i}")
    }

    println!("Uppercase: {}", city.to_uppercase());     // -> String
    println!("Lowercase: {}", city.to_lowercase());     // -> String
    println!("Contains 'ou': {}", city.contains("ou")); // -> bool

    let city_u = city.to_uppercase();
    // city_u.as_str() => &str

    let city_s: String = String::from("L'Haÿ-les-Roses");
    let mut extract = &city[..3];
    println!("Slice [..3] => {}", extract);
    extract = &city[3..6];
    println!("Slice [3..6] => {}", extract);
    extract = &city[6..];
    println!("Slice [6..] => {}", extract);
    extract = &city_u[..4];
    println!("Slice on String => {}", extract);
    extract = &city_s[..4];
    println!("Slice on String => {}", extract);

    // NB:
    let c: char = '京';
    println!("Some char: {}", c)
}

fn play_with_integers(){
    println!("***** Integers ******");
    // integers : i8, i16, i32, i64, u8, u16, u32, u64
    let x = 12; // i32 (default)
    let x2 = 12i32;
    let x3: i32 = 12;
    let code = 255u8;
    let index: usize = 4000;
    println!("Some integers: {x}, {x2}, {x3}, {code}, {index}");

    let y = x + x2 + x3;
    let z = y * i32::from(code); // cast with function
    let w = code - (x as u8); // cast with as 'keyword'
    println!("Some results: {y}, {z}, {w}");

    let a = 23; // inference de type resolved with nex line
    let b = a + 3_000_000_000u32;
    println!("Other ones: {a}, {b}");

    let mut c = (b - a) + (a / 2 * 4);
    c += 20;
    c -= 5;
    c /= 10;
    c *= 2;
    
    // pas de ++, --
    println!("Result: c = {c}");
    c = a ^ 2; // xor (bitwise) : 10111 ^ 00010 = 10101 (see also | & << >>)
    println!("Result: c = {c}");

    c = a.pow(2);
    println!("Power 2: {c}");
    c = (c + 1).isqrt();
    println!("Square root: {c}");

    // c = c / 0; // compilation error: attempt to divide `_` by zero
    // c = 1 / (c - c); // dynamic error: thread 'main' (12612) panicked at src\main.rs:82:9: attempt to divide by zero
    // println!("Divide by 0: {c}");
}

fn play_with_floats(){
    println!("***** Floats ******");
    // IEEE 754
    let x1 = 12.5;
    let x2 = 3.14f32;
    let x3 = 1E308f64;
    let x4: f64 = -1.456E-48;

    let mut y = x1 + (x2 as f64) + (x3 / 1E300) + x4;
    println!("Some computation with floats: {y}");

    y = y.sqrt();
    y *= 2.;
    y = (y + 1.).powi(3);
    y = y.powf(2.5);
    println!("Result: {y}");

    y = x3.powi(2);
    println!("Result: {y}");
    y = 1. / y;
    println!("Result: {y}");
    y = 1. / y;
    println!("Result: {y}");
    y = y / y;
    println!("Result: {y}");
}

fn play_with_float_precision(){
    println!("***** Floats (precision) ******");
    let price = 0.1; // base 2 : 0.00011001100110011...
    let total1 = price * 2.;
    let total2 = price * 3.;
    println!("Price u = {price}; by 2 = {total1} ; by 3 = {total2}") 
}

fn play_with_tuples(){
    println!("***** Tuples ******");
    let coords_2d = (12.5, 3.5);
    let coords_3d = (23.4, 45.6, 78.9);
    let named_coords = ("A", 21.3, 65.4);
    println!("Coords 2D : {coords_2d:?}");
    println!("Coords 3D : {coords_3d:?}");
    println!("Coords with name : {named_coords:?}");
    println!("One coord: {}", coords_2d.0);

    let (x, y) = coords_2d;
    println!("Average coord: {}", (x + y) / 2.);

    let (_, x0, y0) = named_coords;
    println!("Coords withoiut name: x={x0}, y={y0}")
}

fn helper_slice(number_slice: &[u64]){
    println!("Values: {:?} ({})", number_slice, number_slice.len());
    let values: Vec<u64> = number_slice.iter()
        .filter(|v| **v > 10)
        .map(|v| v * v + 1)
        .collect();
    println!("Collect #1: {:?} ({})", values, values.len());
    let values2 = number_slice.iter()
        .filter(|x| **x % 2 == 1)
        .collect::<Vec<&u64>>();
    println!("Collect #2: {:?} ({})", values2, values2.len());
}

fn helper_mutable_slice(number_slice: &mut [u64]){
    println!("Values (before sort): {:?} ({})", number_slice, number_slice.len());
    number_slice.sort();
    println!("Values (after sort): {:?} ({})", number_slice, number_slice.len());
}

fn play_with_vector_array(){
    println!("***** Arrays and Vectors ******");
    let numbers = [12, 14, 17, 18, 4, 5, 7u64]; // type [u64; 7], fixed size
    let mut numbers_m = numbers.clone();
    let number_vector = vec![12, 14, 17, 18, 4, 5, 7u64]; // Vec<u64>
    let mut other_numbers = vec![12, 14, 17, 18, 4, 5, 7u64];
    other_numbers.push(7);
    numbers_m[0] = 33;
    println!("numbers (array): {:?} ({})", numbers, numbers.len());
    println!("numbers (array mutable): {:?} ({})", numbers_m, numbers.len());
    println!("numbers (vec): {:?} ({})", number_vector, numbers.len());
    println!("numbers (mut vec): {:?} ({})", other_numbers, numbers.len());

    // loops

    // foreach
    println!("Numbers:");
    for nb in numbers{
        println!("- {nb}")
    }

    println!("Numbers:");
    for nb in numbers_m{
        println!("# {nb}")
    }

    println!("Numbers:");
    for nb in &number_vector{
        println!("~ {nb}")
    }

    println!("Numbers:");
    for nb in &other_numbers{
        println!("@ {nb}")
    }

    // operator[] => 1 element
    println!("Element #0: {}", numbers[0]);
    println!("Element #0: {}", numbers_m[0]);
    println!("Element #0: {}", number_vector[0]);
    println!("Element #0: {}", other_numbers[0]);

    // operator[] => slice
    let mut extract = &numbers[..3];
    println!("Extract: {:?} ({})", extract, extract.len());
    extract = &numbers_m[3..6];
    println!("Extract: {:?} ({})", extract, extract.len());
    extract = &number_vector[5..];
    println!("Extract: {:?} ({})", extract, extract.len());

    helper_slice(&numbers);
    helper_slice(&numbers_m);
    helper_slice(&number_vector);
    helper_slice(&other_numbers);
    helper_slice(&other_numbers[3..]);
    helper_slice(&other_numbers[..]);

    helper_mutable_slice(&mut numbers_m[..3]);
    helper_mutable_slice(&mut numbers_m);

    // helper_mutable_slice(&mut numbers);
    // helper_mutable_slice(&mut numbers[..3]);
    // helper_mutable_slice(&mut number_vector);
    // helper_mutable_slice(&mut numbers[..3]);
}

fn play_with_reference(){
    let x = 32u64;
    let y = &x;
    let z = &&x;
    println!("x and its references: {x}, {y}, {z}");

    let mut res = x + 1;  // u64 + u64
    println!("result: {res}");

    res = x + y;                // u64 + &u64
    println!("result: {res}");
    
    // res = x + z;  // not implmented: u64 + &&u64
    res = x + *z;    // u64 + &u64
    println!("result: {res}");

    res = x + **z;    // u64 + u64
    println!("result: {res}");

    res = y + 1;  // &u64 + u64
    println!("result: {res}");

    res = y + y;  // &u64 + &u64
    println!("result: {res}");

    res = y + *z;  // &u64 + &u64
    println!("result: {res}");
}

fn main() {
    play_with_text();
    play_with_integers();
    play_with_floats();
    play_with_float_precision();
    play_with_tuples();
    play_with_vector_array();
    play_with_reference()
}
