use crate::{entry::Single, keytao::*};

pub(crate) fn check(dict: &[Single]) -> Vec<&Single> {
    dict.iter().filter(|e| !valid(e.code.as_bytes())).collect()
}

fn valid(b: &[u8]) -> bool {
    matches!(b.len(), 2..=MAX_CODE_N)
        && is_ym(b[0])
        && is_ym(b[1])
        && b[2..].iter().all(|&c| is_xm(c))
}
