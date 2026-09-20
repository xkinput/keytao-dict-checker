use ahash::AHashMap;

pub(crate) fn check(singles: &[crate::Single]) -> Vec<&str> {
    let mut cnt = AHashMap::<_, usize>::with_capacity(singles.len());
    let mut grp = AHashMap::<_, Vec<_>>::with_capacity(singles.len());

    for e in singles {
        let code = &*e.code;
        *cnt.entry(code).or_default() += 1;
        grp.entry(e.text).or_default().push(code);
    }

    let mut result = vec![];

    for (_, mut codes) in grp {
        codes.sort_unstable();
        codes.dedup();

        if codes.len() < 2 || !codes.iter().any(|code| cnt[code] > 1) {
            continue;
        }

        for code in &codes {
            for n in 1..code.len() {
                let s = &code[..n];
                if codes.binary_search(&s).is_ok() && cnt[s] > 1 {
                    result.push(s);
                }
            }
        }
    }

    result.sort_unstable();
    result.dedup();
    result
}
