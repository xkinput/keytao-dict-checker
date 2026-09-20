use crate::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut entries = Vec::from_iter(singles);
    entries.sort_unstable_by_key(|e| (e.text, &e.code));

    let mut result = vec![];
    let mut codes = ahash::AHashSet::new();

    for grp in entries.chunk_by(|a, b| a.text == b.text) {
        codes.clear();
        let mut max_len = 0;

        for e in grp {
            codes.insert(&*e.code);
            max_len = max_len.max(e.code.len());
        }

        for &e in grp {
            if e.code.len() == max_len && (1..max_len).all(|n| !codes.contains(&e.code[..n])) {
                result.push(e);
            }
        }
    }

    result
}
