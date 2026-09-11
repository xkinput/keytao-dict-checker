use crate::{entry::Single, keytao::*};

pub(crate) fn check(dict: &[Single]) -> Vec<&Single> {
    dict.iter().filter(|e| !valid(&e.code)).collect()
}

fn valid(code: &str) -> bool {
    match code.as_bytes() {
        [c] => is_ym(*c),
        [a, b, xm @ ..] if xm.len() <= 4 => is_ym(*a) && is_ym(*b) && xm.iter().all(|&c| is_xm(c)),
        _ => false,
    }
}
