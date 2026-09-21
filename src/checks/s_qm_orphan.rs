use crate::{Single, keytao::*};
use ahash::AHashSet;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut used = AHashSet::with_capacity(singles.len());
    let mut pairs = AHashSet::with_capacity(singles.len());
    let mut entries = Vec::with_capacity(singles.len());

    for e in singles {
        if e.code.len() <= MAX_CODE_N {
            let k = e.code.bytes().fold(0, append_key);
            used.insert(k);
            pairs.insert(pack_pair(e.text, k));
            entries.push(e);
        }
    }

    entries.sort_unstable_by_key(|e| (e.text, &e.code));

    let mut result = vec![];
    for t_grp in entries.chunk_by(|a, b| a.text == b.text) {
        let mut c_grp = t_grp.chunk_by(|a, b| a.code == b.code).peekable();

        while let Some(grp) = c_grp.next() {
            let cur = grp[0];

            if c_grp
                .peek()
                .is_some_and(|n| n[0].code.starts_with(&*cur.code))
            {
                continue;
            }

            if is_orphan(cur.text, &cur.code, &used, &pairs) {
                result.extend_from_slice(grp);
            }
        }
    }
    result
}

fn is_orphan(text: char, code: &str, used: &AHashSet<u32>, pairs: &AHashSet<u64>) -> bool {
    let mut k = 0;
    let mut idle = false;

    for b in code.bytes().take(code.len().saturating_sub(1)) {
        k = append_key(k, b);
        if pairs.contains(&pack_pair(text, k)) {
            return false;
        }
        idle |= !used.contains(&k);
    }

    idle
}

fn append_key(key: u32, c: u8) -> u32 {
    key * 27 + u32::from(c - b'a' + 1)
}

fn pack_pair(text: char, key: u32) -> u64 {
    u64::from(text) << 32 | u64::from(key)
}
