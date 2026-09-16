use crate::{entry::Single, keytao::*};

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut entries = Vec::from_iter(singles.iter());
    entries.sort_unstable_by_key(|e| (e.text, &e.code));

    let mut res = vec![];
    let mut runs = vec![];
    let mut marks = vec![];

    for g in entries.chunk_by(|a, b| a.text == b.text) {
        runs.clear();
        runs.extend(g.chunk_by(|a, b| a.code == b.code));
        marks.clear();
        marks.resize(runs.len(), false);

        for &run in &runs {
            let code = &*run[0].code;
            let mut hits = [0; MAX_CODE_N];
            let mut hn = 0;
            let mut cnt = 0;

            for n in 1..code.len() {
                if let Ok(ix) = runs.binary_search_by(|r| r[0].code[..].cmp(&code[..n])) {
                    cnt += runs[ix].len();
                    hits[hn] = ix;
                    hn += 1;
                }
            }

            if cnt > 1 {
                for &ix in &hits[..hn] {
                    marks[ix] = true;
                }
            }
        }

        for (&m, &run) in marks.iter().zip(&runs) {
            if m {
                res.extend_from_slice(run);
            }
        }
    }

    res
}
