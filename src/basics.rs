//! This module contains basic math functions
//! # Examples
//! ```
//! use MATH_UTILS::basics::{sum, subtract, multiply, divide};
//! let result = sum(5, 3);
//! assert_eq!(result, 8);
//! ```


/// This function adds two numbers
/// # Examples
/// ```
/// use MATH_UTILS::basic::sum;
/// let result = sum(5, 3);
/// assert_eq!(result, 8);
/// ```
pub fn sum(a: u64, b: u64) -> u64 {
    a + b
}

/// This function subtracts two numbers
/// # Examples    
/// ```	
/// use MATH_UTILS::basics::subtract;
/// let result = subtract(5, 3);
/// assert_eq!(result, 2);
/// ```
pub fn subtract(a: u64, b: u64) -> u64 {
    a - b
}

/// This function multiplies two numbers
/// # Examples
/// ```
/// use MATH_UTILS::basics::multiply;
/// let result = multiply(5, 3);
/// assert_eq!(result, 15);
/// ```
pub fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

/// This function divides two numbers
/// # Examples
///  ```
/// use MATH_UTILS::basics::divide;
/// let result = divide(6, 3);
/// assert_eq!(result, 2);
/// ```
/// # Panics
/// The function will panic if the second argument is zero
pub fn divide(a: u64, b: u64) -> u64 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[should_panic]
    fn test_divide_by_zero() {
        divide(6, 0);
    }
}