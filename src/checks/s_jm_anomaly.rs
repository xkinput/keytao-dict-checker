use crate::entry::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut occ = ahash::AHashMap::with_capacity(singles.len());
    for e in singles {
        *occ.entry(&*e.code).or_insert(0) += 1;
    }

    let mut items: Vec<_> = singles.iter().collect();
    items.sort_unstable_by_key(|e| e.text);
    let mut hit = vec![false; items.len()];
    let mut start = 0;

    for g in items.chunk_by(|a, b| a.text == b.text) {
        for f in g {
            if g.iter()
                .any(|e| e.code.len() > f.code.len() && e.code.starts_with(&*f.code))
            {
                continue;
            }

            let n = g
                .iter()
                .filter(|e| e.code.len() < f.code.len() && f.code.starts_with(&*e.code))
                .count();

            for (j, e) in g.iter().enumerate() {
                if e.code.len() < f.code.len()
                    && f.code.starts_with(&*e.code)
                    && (n > 1
                        || occ[&*e.code] > 1
                        || (1..e.code.len()).any(|l| !occ.contains_key(&f.code[..l])))
                {
                    hit[start + j] = true;
                }
            }
        }
        start += g.len();
    }

    items
        .into_iter()
        .zip(hit)
        .filter_map(|(e, h)| h.then_some(e))
        .collect()
}
