// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
use std::{error, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}
impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            DivisionError::NotDivisible(strct) => format!(
                "{} is not evenly divisible by {}",
                strct.dividend.clone(), strct.divisor.clone()
            ),
            DivisionError::DivideByZero => "divisor is zero".to_string(),
        };
        f.write_str(&description)
    }
}
impl error::Error for DivisionError {}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    // todo!();
    match (a, b) {
        _ if b == 0 => Err(DivisionError::DivideByZero),
        _ if a % b == 0 => Ok(a / b),
        _ => Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        })),
    }
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: Ok([1, 11, 1426, 3])
//
// NOTE: Result implements FromIterator so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>). Once an Result::Err is found, the iteration will terminate.
fn result_with_list() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results: Result<Vec<i32>, _> =
        numbers.into_iter().map(|n| divide(n, 27)).collect();
    // let outish = division_results
    //     .map(|okay| okay.unwrap())
    //     .collect::<Vec<i32>>();
    // Ok(outish)
    // `?` returns `From::from(E)`, which is why it works with signature
    Ok(division_results?)
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
