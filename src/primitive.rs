use crate::combinator::{Lit, Re};

pub fn lit(text: &str) -> Lit {
    Lit {
        text: text.to_string(),
    }
}

pub fn re(pattern: &str) -> Result<Re, regex::Error> {
    Ok(Re {
        pattern: regex::Regex::new(pattern)?,
    })
}
