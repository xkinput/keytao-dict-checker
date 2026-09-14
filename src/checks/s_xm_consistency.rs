use crate::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut states = ahash::AHashMap::with_capacity(singles.len() / 2);

    for e in singles {
        let (longest, is_bad) = states.entry(e.text).or_default();
        if *is_bad {
            continue;
        }

        let code = e.code.as_bytes();
        if code.len() < 3 {
            continue;
        }
        let xm = &code[2..];

        match *longest {
            None => *longest = Some(xm),
            Some(l) if l.starts_with(xm) => {}
            Some(l) if xm.starts_with(l) => *longest = Some(xm),
            _ => *is_bad = true,
        }
    }

    let mut res: Vec<_> = singles.iter().filter(|e| states[&e.text].1).collect();
    res.sort_unstable_by_key(|e| e.text);
    res
}
