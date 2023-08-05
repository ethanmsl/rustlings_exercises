// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    match time_of_day {
        // // apparently the exclusive range syntax (0..22) is still experimental
        // // ... which seems bizzare, so we've kludigily assumed that all values
        // // will be in whole hours for time of day
        // // with the result that 21..22 gives None .... blech!!!
        // 0..=21 => Some(5),
        // 22..=24 => Some(0),
        // _ => None,

        // here's an unfortunate alternate approach using if statements
        _ if 0 <= time_of_day && time_of_day < 22 => Some(5),
        _ if 22 <= time_of_day && time_of_day < 24 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12).expect("Shoudl have been a numeric value.");
        assert_eq!(icecreams, 5);
    }
}
