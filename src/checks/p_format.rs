use crate::{Phrase, keytao::*};

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&Phrase> {
    phrases
        .iter()
        .filter(|e| !valid(&e.code, &e.text))
        .collect()
}

fn valid(code: &str, text: &str) -> bool {
    match (phrase_min_code_len(text), code.as_bytes()) {
        (3, [a, b, c, xm @ ..]) if xm.len() <= 3 => {
            is_ym(*a) && is_ym(*b) && is_ym(*c) && xm.iter().all(|&c| is_xm(c))
        }
        (4, [a, b, c, d, xm @ ..]) if xm.len() <= 2 => {
            is_ym(*a) && is_ym(*b) && is_ym(*c) && is_ym(*d) && xm.iter().all(|&c| is_xm(c))
        }
        _ => false,
    }
}
