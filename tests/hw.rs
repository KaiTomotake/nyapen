use nyapen::prelude::*;
use nyapen::primitive::{lit, re};

#[derive(Debug, Clone)]
enum Hw {
    Hello,
    World,
    Comma
}

#[test]
fn hw_complete() {
    assert_eq!(
        lit("Hello")
            .map(|_, _| Hw::Hello)
            .then(lit(",").map(|_, _| Hw::Comma).opt())
            .then(lit("World").map(|_, _| Hw::World).opt())
            .map(|m, _| {
                let f = m.clone().unwrap().0.unwrap();
                (f.0.unwrap(), f.1, m.unwrap().1)
            })
            .repeated()
            .eoi()
            .parse(" Hello , World Hello Hello", Some(re("\\s+").unwrap()))
            .unwrap()
            .parsed,
        Vec::<String>::new()
    );
}
