use crate::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut states = ahash::AHashMap::with_capacity(singles.len() / 2);
    let mut bad = false;

    for e in singles {
        let x = e.code.as_bytes().get(2..).unwrap_or(&[]);
        if x.is_empty() {
            continue;
        }

        let (longest, is_bad) = states.entry(e.text).or_default();
        if *is_bad {
            continue;
        }

        match *longest {
            None => *longest = Some(x),
            Some(l) if l.starts_with(x) => {}
            Some(l) if x.starts_with(l) => *longest = Some(x),
            Some(_) => {
                *is_bad = true;
                bad = true;
            }
        }
    }

    if !bad {
        return vec![];
    }

    let mut result: Vec<_> = singles
        .iter()
        .filter(|e| states.get(&e.text).is_some_and(|s| s.1))
        .collect();
    result.sort_unstable_by_key(|e| (e.text, &e.code));
    result
}
