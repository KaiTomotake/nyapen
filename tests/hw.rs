use nyapen::prelude::*;
use nyapen::primitive::{lit, re};

#[derive(Debug, Clone, PartialEq)]
enum Hw {
    Hello,
    World,
    Comma,
}

#[test]
fn hw_complete() {
    assert_eq!(
        lit("Hello")
            .map(|_, _| Hw::Hello)
            .then(lit(",").map(|_, _| Hw::Comma).opt())
            .then(lit("World").map(|_, _| Hw::World).opt())
            .map(|m, _| match m {
                Some((Some((hello, comma)), world)) => (hello.unwrap(), comma, world),
                _ => panic!("unexpected parse structure in hw_complete"),
            })
            .repeated()
            .eoi()
            .skip(re("\\s+").unwrap())
            .parse(" Hello , World Hello Hello")
            .unwrap()
            .mapped,
        Some(vec![
            (Hw::Hello, Some(Hw::Comma,), Some(Hw::World,),),
            (Hw::Hello, None, None,),
            (Hw::Hello, None, None,),
        ],)
    );
}
