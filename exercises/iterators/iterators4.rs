// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


// use std::iter::successors;

use std::iter::once;

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // (1..=num).fold(1, |acc, x| acc*x)
    // ^ this is the best solution for this problem (and probably most performant
    //   due to natural fold/reduce optimizations)
    //   the other solutions below are good explorations of iteration options and
    //   possiblities however

    // // inifinite factorial calculation
    // // Whoops - LOL - no, I need to `+1` the place not the calculated value
    // // ... one can do this, but it ends up being a bit of a useless mess relative to
    // // just using range
    // // [leaving this here as it was still instructive to create]
    // let factoriator = successors(Some(1), |prev| Some(prev * (prev + 1)));
    // factoriator
    //     .take(num.try_into().unwrap())
    //     .collect::<Vec<u64>>()
    //     .last()
    //     .unwrap()
    //     .clone()

    // inifintie factorial calculation
    // `scan` is like fold, but produces an iterator
    let mut factoriator = (1..).scan(1, |acc, idx| {
        *acc = *acc * idx;
        Some(*acc)
    });
    // // creating a vector and then taking last element
    // let result = factoriator
    //     .take(num.try_into().unwrap())
    //     .collect::<Vec<u64>>();
    // *result.last().unwrap()

    // taking last element straight away
    // (previously using `as` vs `into` -- **NOT** good, or rather, unsafe.
    //    `nth(num as u64 - 1)`
    //  later doing the calc elsewhere -- about indicating output type cleanly)
    // I assume if one were to use this on a 32 bit machine with a u64 it could
    // cause unnoted problems...
    // if num == 0 {
    //     return 1;
    // } else {
    //     let location: usize = num.try_into().unwrap();
    //     let result = factoriator.nth(location - 1);
    //     result.clone().unwrap()
    // }

    // as above, but chaining iterators together so that we don't need to create
    // a special branch to deal with the 0 index should get 1 and all other indexes
    // needing to be shifted
    let mut factoriator_full = once(1).chain(factoriator);
    let result = factoriator_full.nth(num.try_into().unwrap());
    result.clone().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
