use crate::{entry::Single, keytao::*};

const BASE: u32 = CODE_ELEMS as u32 + 1;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut items: Vec<_> = singles.iter().map(|e| (e, pack(&e.code))).collect();
    items.sort_unstable_by_key(|&(e, k)| (e.text, k));
    let used: ahash::AHashSet<_> = items.iter().map(|&(_, k)| k).collect();

    let mut res = vec![];
    let mut keys = vec![];
    let mut covered = vec![];

    for g in items.chunk_by(|a, b| a.0.text == b.0.text) {
        keys.clear();
        keys.extend(g.iter().map(|&(_, k)| k));
        keys.dedup();

        covered.clear();
        covered.extend(g.iter().flat_map(|&(e, _)| prefixes(&e.code)));
        covered.sort_unstable();
        covered.dedup();

        for &(e, k) in g {
            if covered.binary_search(&k).is_ok() {
                continue;
            }
            if prefixes(&e.code).any(|p| keys.binary_search(&p).is_ok()) {
                continue;
            }
            if prefixes(&e.code).any(|p| !used.contains(&p)) {
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

fn prefixes(code: &str) -> impl Iterator<Item = u32> {
    let mut v = 0;
    code.bytes().take(code.len() - 1).map(move |b| {
        v = v * BASE + u32::from(b - b'a' + 1);
        v
    })
}
