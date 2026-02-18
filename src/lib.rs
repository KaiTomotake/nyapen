pub mod combinator;
pub mod parser;
pub mod prelude;
pub mod primitive;

#[cfg(test)]
mod tests {
    #[test]
    fn hw() {
        use crate::prelude::*;
        use crate::primitive::{lit, re};

        assert_eq!(
            lit("Hello")
                .then(lit(",").opt())
                .then(lit("World"))
                .skip(re("\\s+").unwrap())
                .parse(" Hello  World")
                .unwrap()
                .parsed,
            &["Hello", "World"]
        );
    }
}
