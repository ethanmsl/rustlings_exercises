// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar` for the type `String`.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(/* mut  */ self) -> Self {
        // this works even though `self` is not mutable!
        // it consume the string and reuses it's buffer apparently
        // it's interesting that we can destructively use self, but not alter it
        // not entirely unreasonable, but a little bit surprising
        // the fact that the following is an expression rather than a statement is
        // important here
        self + "Bar"

        // this does NOT work as it attempt to mutate self
        // self.push_str("Bar");
        // self

        // BUT we CAN change this to have `(mut self)` as part of its signature
        // as the `mut ` is considered a pattern, not type
        // we don't need the trait-impl to have mut defined
        // (and, in fact, the compiler will balk if we try to)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
