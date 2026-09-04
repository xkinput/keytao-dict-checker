use crate::{entry::Phrase, inputs::SingleDict};
use rayon::prelude::*;

pub(crate) fn find_incorrect<'a>(phrases: &'a [Phrase], singles: &SingleDict) -> Vec<&'a Phrase> {
    phrases
        .par_iter()
        .filter(|e| !is_correct(e, singles))
        .collect()
}

fn is_correct(e: &Phrase, singles: &SingleDict) -> bool {
    let mut code = [None; 6];
    let mut len = 0;
    for c in e.code.chars() {
        if len == 6 {
            return false;
        }
        code[len] = Some(c);
        len += 1;
    }

    let mut cs = e.text.chars();
    match (cs.next(), cs.next(), cs.next(), cs.next()) {
        (Some(c0), Some(c1), c2, c3) => match (c2, c3) {
            (None, _) => {
                matches!(len, 4..=6)
                    && slot_ok(singles, c0, [code[0], code[1], code[4]])
                    && slot_ok(singles, c1, [code[2], code[3], code[5]])
            }
            (Some(c2), None) => {
                matches!(len, 3..=6)
                    && slot_ok(singles, c0, [code[0], None, code[3]])
                    && slot_ok(singles, c1, [code[1], None, code[4]])
                    && slot_ok(singles, c2, [code[2], None, code[5]])
            }
            (Some(c2), Some(_)) => match e.text.chars().next_back() {
                Some(c3) => {
                    matches!(len, 4..=6)
                        && slot_ok(singles, c0, [code[0], None, code[4]])
                        && slot_ok(singles, c1, [code[1], None, code[5]])
                        && slot_ok(singles, c2, [code[2], None, None])
                        && slot_ok(singles, c3, [code[3], None, None])
                }
                None => false,
            },
        },
        _ => false,
    }
}

fn slot_ok(singles: &SingleDict, c: char, req: [Option<char>; 3]) -> bool {
    singles.get(&c).is_some_and(|stems| {
        stems.iter().any(|&[x, y, z]| {
            req[0].is_none_or(|r| r == x)
                && req[1].is_none_or(|r| r == y)
                && req[2].is_none_or(|r| r == z)
        })
    })
}
