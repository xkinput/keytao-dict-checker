use crate::{Single, keytao::*};
use ahash::AHashSet;

pub(crate) fn check(singles: &[Single]) -> Vec<&str> {
    let mut used = AHashSet::with_capacity(singles.len());
    let mut pairs = AHashSet::with_capacity(singles.len());

    for e in singles {
        if e.code.len() <= MAX_CODE_N {
            let k = e.code.bytes().fold(0, append_key);
            used.insert(k);
            pairs.insert(pack_pair(e.text, k));
        }
    }

    let mut seen = AHashSet::new();
    let mut result = vec![];

    for e in singles {
        let n = e.code.len();
        if n < 2 || n > MAX_CODE_N {
            continue;
        }

        let mut k = 0;
        let mut m = 0;
        for (i, c) in e.code.bytes().enumerate().take(n - 1) {
            k = append_key(k, c);
            if pairs.contains(&pack_pair(e.text, k)) {
                m = i + 1;
            }
        }

        let mut k = 0;
        for (i, c) in e.code.bytes().enumerate().take(m.saturating_sub(1)) {
            k = append_key(k, c);
            if !used.contains(&k) && seen.insert(k) {
                result.push(&e.code[..i + 1]);
            }
        }
    }

    result.sort_unstable();
    result
}

fn append_key(key: u32, c: u8) -> u32 {
    key * 27 + u32::from(c - b'a' + 1)
}

fn pack_pair(text: char, key: u32) -> u64 {
    u64::from(text) << 32 | u64::from(key)
}
