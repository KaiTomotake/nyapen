use crate::combinator::Lit;

pub fn lit(text: &str) -> Lit {
    Lit {
        text: text.to_string(),
    }
}
