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

#[derive(Debug, Clone)]
pub struct Re {
    pub(crate) pattern: regex::Regex,
}

impl Parser for Re {
    type Mapped = NoMap;

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        if let Some(mat) = self.pattern.find(src.get(pos..).ok_or(ParseError {
            rule: "re".to_string(),
            pos,
        })?) {
            if mat.start() == 0 {
                Ok(Output {
                    mapped: None,
                    parsed: vec![mat.as_str().to_string()],
                    pos: skipper(src, pos + mat.len(), skip),
                })
            } else {
                Err(ParseError {
                    rule: "re".to_string(),
                    pos,
                })
            }
        } else {
            Err(ParseError {
                rule: "re".to_string(),
                pos,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Repeated<P: Parser> {
    pub(crate) parser: P,
}

impl<P: Parser> Parser for Repeated<P> {
    type Mapped = Vec<P::Mapped>;

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        mut pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        let mut mapped_values = Vec::new();
        let mut parsed = Vec::new();
        while let Ok(out) = self.parser.parse_with_position(src, pos, skip) {
            if let Some(val) = out.mapped {
                mapped_values.push(val);
            }
            parsed.extend(out.parsed);
            if out.pos == pos {
                break;
            }
            pos = out.pos;
        }
        if mapped_values.is_empty() {
            Ok(Output {
                mapped: None,
                parsed,
                pos,
            })
        } else {
            Ok(Output {
                mapped: Some(mapped_values),
                parsed,
                pos,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Opt<P: Parser> {
    pub(crate) parser: P,
}

impl<P: Parser> Parser for Opt<P> {
    type Mapped = P::Mapped;

    fn parse_with_position<S: Parser>(
        &self,
        src: &str,
        pos: usize,
        skip: &Option<S>,
    ) -> Result<Output<Self::Mapped>, ParseError> {
        if let Ok(out) = self.parser.parse_with_position(src, pos, skip) {
            Ok(Output {
                mapped: out.mapped,
                parsed: out.parsed,
                pos: out.pos,
            })
        } else {
            Ok(Output {
                mapped: None,
                parsed: Vec::new(),
                pos,
            })
        }
    }
}
