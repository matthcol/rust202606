/**
 * compute greater common divider of 2 natural integers (strictly positive)
 * 
 */
pub fn gcd(a: u64, b: u64) -> u64{
    let mut a_m = a;
    let mut b_m = b;
    if a == 0 || b == 0{
        panic!("arguments must be strictly positive, got: a = {a_m}, b = {b_m}")
    }
    while a_m != b_m{
        if a_m > b_m{
            a_m = a_m - b_m
        } else {
            b_m = b_m - a_m
        }
    } 
    a_m
}

pub fn gcd_ref(a: &u64, b: &u64) -> u64{
    let mut a_m = *a;
    let mut b_m = *b;
        if a_m == 0 || b_m == 0{
        panic!("arguments must be strictly positive, got: a = {a_m}, b = {b_m}")
    }
    while a_m != b_m{
        if a_m > b_m{
            a_m = a_m - b_m
        } else {
            b_m = b_m - a_m
        }
    } 
    a_m
}

pub fn gcd_option(a: u64, b: u64) -> Option<u64>{
    let mut a_m = a;
    let mut b_m = b;
    if a == 0 || b == 0{
        return None
    }
    while a_m != b_m{
        if a_m > b_m{
            a_m = a_m - b_m
        } else {
            b_m = b_m - a_m
        }
    } 
    Some(a_m)
}

pub fn gcd_result(a: u64, b: u64) -> Result<u64, String>{
    let mut a_m = a;
    let mut b_m = b;
    if a == 0 || b == 0{
        return Err(String::from("arguments must be strictly positive"))
    }
    while a_m != b_m{
        if a_m > b_m{
            a_m = a_m - b_m
        } else {
            b_m = b_m - a_m
        }
    } 
    Ok(a_m)
}

// unit tests

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd_when_both_1(){
        let g = gcd(1, 1);
        assert_eq!(1, g)
    }

    #[test]
    fn test_gcd_when_first_1(){
        let g = gcd(1, 5);
        assert_eq!(1, g)
    }

    #[test]
    fn test_gcd_when_second_1(){
        let g = gcd(7, 1);
        assert_eq!(1, g)
    }

    #[test]
    fn test_gcd_when_consecutive_fibonacci_terms(){
        let a = 7_540_113_804_746_346_429u64; // F(92)
        let b = 12_200_160_415_121_876_738u64; // F(93)
        let g = gcd(a, b);
        assert_eq!(1, g)
    }

    #[test]
    fn test_gcd_when_both_primes(){
        let g = gcd(7, 13);
        assert_eq!(1, g)
    }

     #[test]
    fn test_gcd_when_both_equals(){
        let g = gcd(13, 13);
        assert_eq!(13, g)
    }

    #[test]
    fn test_gcd_when_first_greater(){
        let g = gcd(21, 15);
        assert_eq!(3, g)
    }

    #[test]
    fn test_gcd_when_second_greater(){
        let g = gcd(15, 21);
        assert_eq!(3, g)
    }

    #[test]
    #[should_panic(expected = "arguments must be strictly positive, got: a = 0, b = 5")]
    fn test_gcd_should_panic_when_first_zero(){
        let _ = gcd(0, 5);
    }

    #[test]
    #[should_panic(expected = "arguments must be strictly positive, got: a = 5, b = 0")]
    fn test_gcd_should_panic_when_second_zero(){
        let _ = gcd(5, 0);
    }

    #[test]
    #[should_panic(expected = "arguments must be strictly positive, got: a = 0, b = 0")]
    fn test_gcd_should_panic_when_both_zero(){
        let _ = gcd(0, 0);
    }

}