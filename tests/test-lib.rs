use MATH_UTILS::{factorial, gcd, lcm, is_prime};
use MATH_UTILS::basics::{sum, subtract, multiply, divide};


#[test]
fn test_fn(){
    assert!(true);
}


#[test]
fn test_sum() {
    assert_eq!(sum(5, 3), 8);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(5, 3), 15);
}

#[test]
fn test_divide() {
    assert_eq!(divide(6, 3), 2);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(5), 120);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 18), 6);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(12, 18), 36);
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(7), true);
}


