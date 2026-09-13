use crate::entry::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut cnt = ahash::AHashMap::with_capacity(singles.len());
    for e in singles {
        *cnt.entry(&*e.code).or_insert(0) += 1;
    }

    let mut order: Vec<_> = (0..singles.len()).collect();
    order.sort_unstable_by_key(|&i| (singles[i].text, i));

    let mut bad = vec![false; singles.len()];

    for g in order.chunk_by(|&a, &b| singles[a].text == singles[b].text) {
        for (pos, &full_i) in g.iter().enumerate() {
            let full = &*singles[full_i].code;

            if g[..pos].iter().any(|&i| &*singles[i].code == full)
                || g.iter().any(|&i| {
                    singles[i].code.len() > full.len() && singles[i].code.starts_with(full)
                })
            {
                continue;
            }

            let is_jm = |i: usize| {
                &*singles[i].code == full
                    || singles[i].code.len() < full.len() && full.starts_with(&*singles[i].code)
            };

            let n = g.iter().filter(|&&i| i != full_i && is_jm(i)).count();

            for &i in g {
                let code = &*singles[i].code;
                if i != full_i
                    && is_jm(i)
                    && (n > 1
                        || cnt[code] > 1
                        || (1..code.len()).any(|k| !cnt.contains_key(&code[..k])))
                {
                    bad[i] = true;
                }
            }
        }
    }

    order
        .into_iter()
        .filter_map(|i| bad[i].then_some(&singles[i]))
        .collect()
}
