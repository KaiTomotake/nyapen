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
                .parse(" Hello  World", Some(re("\\s+").unwrap()))
                .unwrap()
                .parsed,
            &["Hello", "World"]
        );
    }
}
