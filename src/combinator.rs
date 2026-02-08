use crate::parser::{Output, ParseError, Parser};

pub(crate) fn skipper<S: Parser>(src: &str, pos: usize, skip: &Option<S>) -> usize {
    if let Some(s) = skip
        && let Ok(result) = s.parse_with_position::<NoSkip>(src, pos, &None)
    {
        return result.pos;
    }
    pos
}

#[derive(Debug, Clone)]
pub struct NoSkip {}

impl Parser for NoSkip {
    type Mapped = NoMap;

    fn parse_with_position<S: Parser>(
        &self,
        _: &str,
        pos: usize,
        _: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        Err(ParseError {
            rule: "noskip".to_string(),
            pos,
        })
    }
}

#[derive(Debug, Clone)]
pub struct NoMap;

#[derive(Debug, Clone)]
pub struct Lit {
    pub(crate) text: String,
}

impl Parser for Lit {
    type Mapped = NoMap;

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        mut pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        let slice = src.get(pos..).ok_or(ParseError {
            rule: "lit".to_string(),
            pos,
        })?;
        if slice.starts_with(self.text.as_str()) {
            pos += self.text.len();
        } else {
            return Err(ParseError {
                rule: "lit".to_string(),
                pos,
            });
        }
        Ok(Output {
            mapped: None,
            parsed: vec![self.text.to_string()],
            pos: skipper(src, pos, skip),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Map<P, F, R>
where
    P: Parser,
    F: Fn(Option<<P as Parser>::Mapped>, Vec<String>) -> R,
{
    pub(crate) parser: P,
    pub(crate) func: F,
}

impl<P, F, R> Parser for Map<P, F, R>
where
    P: Parser,
    F: Fn(Option<<P as Parser>::Mapped>, Vec<String>) -> R,
{
    type Mapped = R;

    fn parse_with_position<S>(
        &self,
        src: &str,
        pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError>
    where
        S: Parser,
    {
        let out = self.parser.parse_with_position(src, pos, skip)?;
        Ok(Output {
            mapped: Some((self.func)(out.mapped, out.parsed)),
            parsed: Vec::new(),
            pos: out.pos,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Then<A: Parser, B: Parser> {
    pub(crate) parser_a: A,
    pub(crate) parser_b: B,
}

impl<A: Parser, B: Parser> Parser for Then<A, B> {
    type Mapped = (Option<A::Mapped>, Option<B::Mapped>);

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        let output_a = self.parser_a.parse_with_position(src, pos, skip)?;
        let output_b = self.parser_b.parse_with_position(src, output_a.pos, skip)?;
        if output_a.mapped.is_none() {
            if output_b.mapped.is_none() {
                Ok(Output {
                    mapped: Some((output_a.mapped, output_b.mapped)),
                    parsed: output_a.parsed.into_iter().chain(output_b.parsed).collect(),
                    pos: output_b.pos,
                })
            } else {
                Ok(Output {
                    mapped: Some((None, output_b.mapped)),
                    parsed: output_a.parsed,
                    pos: output_b.pos,
                })
            }
        } else {
            if output_b.mapped.is_none() {
                Ok(Output {
                    mapped: Some((output_a.mapped, None)),
                    parsed: output_b.parsed,
                    pos: output_b.pos,
                })
            } else {
                Ok(Output {
                    mapped: None,
                    parsed: output_a.parsed.into_iter().chain(output_b.parsed).collect(),
                    pos: output_b.pos,
                })
            }
        }
    }
}
