use crate::combinator::{Map, Opt, Repeated, Then, skipper};

pub trait Parser: Sized {
    type Mapped;

    fn parse<S: Parser>(
        &self,
        src: &str,
        skip: Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        self.parse_with_position(src, skipper(src, 0, &skip), &skip)
    }

    fn parse_with_map<S: Parser, F: Fn(Option<Self::Mapped>, Vec<String>) -> Self::Mapped>(
        &self,
        src: &str,
        skip: Option<S>,
        f: F,
    ) -> Result<Self::Mapped, ParseError> {
        self.parse_with_position(src, skipper(src, 0, &skip), &skip)
            .map(|out| f(out.mapped, out.parsed))
    }

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError>;

    fn map<F, R>(self, f: F) -> Map<Self, F, R>
    where
        F: Fn(Option<Self::Mapped>, Vec<String>) -> R,
    {
        Map {
            parser: self,
            func: f,
        }
    }

    fn then<P: Parser>(self, parser: P) -> Then<Self, P> {
        Then {
            parser_a: self,
            parser_b: parser,
        }
    }

    fn repeated(self) -> Repeated<Self> {
        Repeated { parser: self }
    }

    fn opt(self) -> Opt<Self> {
        Opt { parser: self }
    }

    fn eoi(self) -> Eoi<Self> {
        Eoi { parser: self }
    }
}

#[derive(Debug, Clone)]
pub struct Output<T> {
    pub mapped: Option<T>,
    pub parsed: Vec<String>,
    pub pos: usize,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub rule: String,
    pub pos: usize,
}
