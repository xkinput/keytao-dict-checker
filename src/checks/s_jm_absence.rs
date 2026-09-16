use crate::{entry::Single, keytao::*};

const BASE: u32 = CODE_ELEMS as u32 + 1;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut items = Vec::with_capacity(singles.len());
    let mut used = ahash::AHashSet::with_capacity(singles.len());
    for e in singles {
        let k = pack(&e.code);
        used.insert(k);
        items.push((e.text, k, e));
    }

    items.sort_unstable_by_key(|r| (r.0, r.1));

    let mut res = Vec::new();
    for g in items.chunk_by(|a, b| a.0 == b.0) {
        let max = g.last().unwrap().2.code.len();
        for &(_, k, e) in g {
            if e.code.len() != max {
                continue;
            }

            let mut p = k / BASE;
            let mut free = false;
            while p != 0 {
                if !used.contains(&p) {
                    free = true;
                } else if g.binary_search_by_key(&p, |r| r.1).is_ok() {
                    free = false;
                    break;
                }
                p /= BASE;
            }

            if free {
                res.push(e);
            }
        }
    }
    res
}

fn pack(code: &str) -> u32 {
    code.bytes()
        .fold(0, |v, b| v * BASE + u32::from(b - b'a' + 1))
}
