use crate::Single;

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut entries = Vec::from_iter(singles);
    entries.sort_unstable_by_key(|e| (e.text, &e.code));

    let mut result = vec![];
    let mut unique = vec![];

    for t_grp in entries.chunk_by(|a, b| a.text == b.text) {
        unique.clear();

        for c_grp in t_grp.chunk_by(|a, b| a.code == b.code) {
            unique.push((&*c_grp[0].code, c_grp.len(), c_grp[0]));
        }

        for (i, &(code, mut cnt, e)) in unique.iter().enumerate() {
            if unique.get(i + 1).is_some_and(|u| u.0.starts_with(code)) {
                continue;
            }

            for n in 1..code.len() {
                if let Ok(k) = unique.binary_search_by(|u| u.0.cmp(&code[..n])) {
                    cnt += unique[k].1;
                    if cnt > 2 {
                        break;
                    }
                }
            }

            if cnt > 2 {
                result.push(e);
            }
        }
    }

    result
}
