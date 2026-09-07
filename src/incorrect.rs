use crate::{entry::Phrase, inputs::SingleDict, keytao::MAX_CODE_LEN};
use rayon::prelude::*;

pub(crate) fn check<'a>(phrases: &'a [Phrase], singles: &SingleDict) -> Vec<&'a Phrase> {
    let mut vec: Vec<_> = phrases
        .par_iter()
        .filter(|&p| !is_correct(p, singles))
        .collect();
    vec.par_sort_unstable_by_key(|&p| &p.text);
    vec
}

fn is_correct(p: &Phrase, singles: &SingleDict) -> bool {
    let b = p.code.as_bytes();

    if b.len() > MAX_CODE_LEN || !b.iter().all(u8::is_ascii_lowercase) {
        return false;
    }

    let len = b.len();
    let at = |i| b.get(i).copied();
    let mut cs = p.text.chars();

    let (Some(c0), Some(c1)) = (cs.next(), cs.next()) else {
        return false;
    };

    match (cs.next(), cs.next()) {
        (None, _) => {
            len >= 4
                && slot_ok(singles, c0, [at(0), at(1), at(4)])
                && slot_ok(singles, c1, [at(2), at(3), at(5)])
        }
        (Some(c2), None) => {
            len >= 3
                && slot_ok(singles, c0, [at(0), None, at(3)])
                && slot_ok(singles, c1, [at(1), None, at(4)])
                && slot_ok(singles, c2, [at(2), None, at(5)])
        }
        (Some(c2), _) => match p.text.chars().next_back() {
            Some(c3) => {
                len >= 4
                    && slot_ok(singles, c0, [at(0), None, at(4)])
                    && slot_ok(singles, c1, [at(1), None, at(5)])
                    && slot_ok(singles, c2, [at(2), None, None])
                    && slot_ok(singles, c3, [at(3), None, None])
            }
            _ => false,
        },
    }
}

fn slot_ok(singles: &SingleDict, c: char, req: [Option<u8>; 3]) -> bool {
    let Some(stems) = singles.get(&c) else {
        return false;
    };

    let mut mask = 0;
    let mut val = 0;

    for (i, b) in req.into_iter().enumerate() {
        if let Some(b) = b {
            let shift = i * 8;
            mask |= 0xffu32 << shift;
            val |= (b as u32) << shift;
        }
    }

    stems.iter().any(|&s| s & mask == val)
}
