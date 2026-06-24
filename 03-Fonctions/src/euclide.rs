/// Compute greater common divider of 2 natural integers (strictly positive)
///
/// # Arguments
///
/// * `a` - first integer
/// * `b` - second integer
///
/// # Returns
///
///  The gcd
///
/// # Panics
///
/// Panic if `a` or `b` is `0`
///
/// # Examples
///
/// ```
/// use fonctions::euclide::gcd;
/// assert_eq!(gcd(21, 15), 3);
/// assert_eq!(gcd(7, 13), 1);
/// ```
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

/// Compute the gcd of two natural integers, returning `None` if undefined.
///
/// # Returns
///
/// - `Some(gcd)` if both `a` and `b` are strictly positive
/// - `None` if `a` or `b` is `0`
///
/// # Examples
///
/// ```
/// use fonctions::euclide::gcd_option;
/// assert_eq!(gcd_option(21, 15), Some(3));
/// assert_eq!(gcd_option(5, 0), None);
/// ```
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

/// Compute the gcd of two natural integers, returning an error if undefined.
///
/// # Returns
///
/// `Ok(gcd)` if both `a` and `b` are strictly positive.
///
/// # Errors
///
/// Returns `Err` if `a` or `b` is `0`.
///
/// # Examples
///
/// ```
/// use fonctions::euclide::gcd_result;
/// assert_eq!(gcd_result(21, 15), Ok(3));
/// assert!(gcd_result(5, 0).is_err());
/// ```
pub fn gcd_result(a: u64, b: u64) -> Result<u64, String>{
    let mut a_m = a;
    let mut b_m = b;
    if a == 0 || b == 0{
        let msg = format!("arguments must be strictly positive, got: a = {a_m}, b = {b_m}");
        // return Err(String::from("arguments must be strictly positive"))
        return Err(msg)
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
    use rstest::rstest;

use crate::euclide::gcd_option;

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

    #[rstest]
    #[case(0, 5)]
    #[case(5, 0)]
    #[case(0, 0)]
    fn test_gcd_option_should_be_none(#[case] a: u64, #[case]b: u64){
        // let a = 0;
        // let b = 0;
        let opt_g = gcd_option(a, b);
        assert!(opt_g.is_none())
    }

    #[rstest]
    #[case(1, 5, 1)]
    #[case(5, 1, 1)]
    #[case(1, 1, 1)]
    #[case(13, 17, 1)]
    #[case(13, 13, 13)]
    #[case(21, 15, 3)]
    #[case(15, 21, 3)]
    #[case(7_540_113_804_746_346_429u64, 12_200_160_415_121_876_738u64, 1u64)]
    fn test_gcd_option_should_be_some(#[case] a: u64, #[case] b: u64, #[case] expected_gcd: u64){
        let opt_gcd = gcd_option(a, b);
        assert!(opt_gcd.is_some());
        let actual_gcd = opt_gcd.unwrap();
        assert_eq!(expected_gcd, actual_gcd)
    }

}