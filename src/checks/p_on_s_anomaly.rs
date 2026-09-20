use crate::{Phrase, StemMap};
use rayon::prelude::*;

pub(crate) fn check<'a>(phrases: &'a [Phrase], stems: &StemMap) -> Vec<&'a Phrase> {
    let mut result: Vec<_> = phrases
        .par_iter()
        .filter(|e| !is_correct(e, stems))
        .collect();
    result.sort_unstable_by_key(|e| (&e.text, &e.code));
    result
}

fn is_correct(e: &Phrase, stems: &StemMap) -> bool {
    let code = e.code.as_bytes();
    let at = |i| code.get(i).copied();
    let mut cs = e.text.chars();
    let (Some(c0), Some(c1)) = (cs.next(), cs.next()) else {
        return false;
    };

    match (cs.next(), cs.next()) {
        (None, _) => {
            slot_ok(stems, c0, [at(0), at(1), at(4)]) && slot_ok(stems, c1, [at(2), at(3), at(5)])
        }
        (Some(c2), None) => {
            slot_ok(stems, c0, [at(0), None, at(3)])
                && slot_ok(stems, c1, [at(1), None, at(4)])
                && slot_ok(stems, c2, [at(2), None, at(5)])
        }
        (Some(c2), _) => e.text.chars().next_back().is_some_and(|c3| {
            slot_ok(stems, c0, [at(0), None, at(4)])
                && slot_ok(stems, c1, [at(1), None, at(5)])
                && slot_ok(stems, c2, [at(2), None, None])
                && slot_ok(stems, c3, [at(3), None, None])
        }),
    }
}

fn slot_ok(stems: &StemMap, c: char, req: [Option<u8>; 3]) -> bool {
    let Some(stems) = stems.get(&c) else {
        return false;
    };
    stems.iter().any(|s| {
        req[0].map_or(true, |v| s[0] == v)
            && req[1].map_or(true, |v| s[1] == v)
            && req[2].map_or(true, |v| s[2] == v)
    })
}
