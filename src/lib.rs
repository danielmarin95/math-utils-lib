//! # MATH_UTILS
//! A library with some basic math functions
//! # Examples:
//! ```
//! use MATH_UTILS::{factorial, gcd, lcm, is_prime};
//! let result = factorial(5);
//! assert_eq!(result, 120);
//! ```

use basics:: {sum, subtract, multiply, divide};

pub mod basics;


/// Get the factorial of a number
/// # Examples
/// ```
/// use MATH_UTILS::factorial;
/// let result = factorial(5);
/// assert_eq!(result, 120);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        multiply(n, factorial(subtract(n, 1)))
    }
}

/// Get the greatest common divisor of two numbers
/// # Examples
/// ```
/// use MATH_UTILS::gcd;
/// let result = gcd(12, 18);
/// assert_eq!(result, 6);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Get the least common multiple of two numbers
/// # Examples
/// ```
/// use MATH_UTILS::lcm;
/// let result = lcm(12, 18);
/// assert_eq!(result, 36);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    divide(multiply(a, b), gcd(a, b))
}

/// Check if a number is prime
/// # Examples
/// ```
/// use MATH_UTILS::is_prime;
/// let result = is_prime(7);
/// assert_eq!(result, true);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}



#[cfg(test)]
mod tests {
    use super::*;

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
}