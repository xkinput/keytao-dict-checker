use crate::entry::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let n = singles.len();
    let mut cnt = ahash::AHashMap::<_, u32>::with_capacity(n);
    for e in singles {
        *cnt.entry(&*e.code).or_default() += 1;
    }

    let mut idx = Vec::from_iter(0..n);
    idx.sort_unstable_by_key(|&i| (singles[i].text, &singles[i].code));

    let mut hit = vec![false; n];
    let mut i = 0;

    while i < n {
        let start = i;
        let c = singles[idx[i]].text;
        let code = &*singles[idx[i]].code;

        while i < n && singles[idx[i]].text == c && &*singles[idx[i]].code == code {
            i += 1;
        }

        if i < n
            && singles[idx[i]].text == c
            && singles[idx[i]].code.starts_with(code)
            && (cnt[code] > 1 || (1..code.len()).any(|j| !cnt.contains_key(&code[..j])))
        {
            for &j in &idx[start..i] {
                hit[j] = true;
            }
        }
    }

    singles
        .iter()
        .zip(hit)
        .filter_map(|(e, h)| h.then_some(e))
        .collect()
}
