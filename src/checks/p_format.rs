use crate::{entry::Phrase, keytao::*};

pub(crate) fn check(dict: &[Phrase]) -> Vec<&Phrase> {
    dict.iter().filter(|e| !valid(e)).collect()
}

fn valid(e: &Phrase) -> bool {
    let min = phrase_min_code_len(&e.text);
    let b = e.code.as_bytes();
    (min..=MAX_CODE_N).contains(&b.len())
        && b[..min].iter().all(|&c| is_ym(c))
        && b[min..].iter().all(|&c| is_xm(c))
}
