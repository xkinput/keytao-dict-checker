use crate::{Phrase, StemMap, keytao::*};
use rayon::prelude::*;

pub(crate) fn check<'a>(phrases: &'a [Phrase], stems: &StemMap) -> Vec<&'a Phrase> {
    let mut entries = Vec::from_iter(phrases);
    entries.par_sort_unstable_by_key(|e| (&e.text, &e.code));

    let mut result = vec![];
    let mut chains = vec![];
    let mut marks = vec![];

    for grp in entries.chunk_by(|a, b| a.text == b.text) {
        check_text(grp, stems, &mut chains, &mut marks, &mut result);
    }

    result
}

fn check_text<'a>(
    grp: &[&'a Phrase],
    stems: &StemMap,
    chains: &mut Vec<(u32, usize, usize)>,
    marks: &mut Vec<bool>,
    result: &mut Vec<&'a Phrase>,
) {
    let text = &*grp[0].text;
    let Some((chars, n)) = phrase_chars(text) else {
        return;
    };
    let min = phrase_min_code_len(text);

    chains.clear();
    let mut i = 0;
    while i < grp.len() {
        let Some(k) = prefix_key(&grp[i].code, min) else {
            i += 1;
            continue;
        };
        let start = i;
        i += 1;
        while i < grp.len() && prefix_key(&grp[i].code, min) == Some(k) {
            i += 1;
        }
        chains.push((k, start, i));
    }

    let chains = &chains[..];
    marks.clear();

    for e in grp {
        let code = e.code.as_bytes();
        let Some(prefix) = code.get(..min) else {
            continue;
        };
        let at = |i| code.get(i).copied();

        let mut base = [0; 4];
        base[..min].copy_from_slice(prefix);

        let mut alt = [(0usize, 0usize, [0u8; 2]); 4];
        let mut m = 0;

        if n == 2 {
            for (p, seg) in [(0usize, [code[0], code[1]]), (2, [code[2], code[3]])] {
                if let Some(t) = full_alt_target(seg) {
                    alt[m] = (p, 2, t);
                    m += 1;
                }
            }
        } else {
            for i in 0..n {
                let third = match (n, i) {
                    (3, 0) => at(3),
                    (3, 1) => at(4),
                    (3, 2) => at(5),
                    (4, 0) => at(4),
                    (4, 1) => at(5),
                    _ => None,
                };

                if let Some(t) = slot_alt(stems, chars[i], code[i], third) {
                    alt[m] = (i, 1, [t, 0]);
                    m += 1;
                }
            }
        }

        if m == 0 {
            continue;
        }

        let base_key = pack_prefix(prefix);
        let mut keys = [0; 16];
        keys[0] = base_key;
        let mut cnt = 1;

        for mask in 1..1u32 << m {
            keys[cnt] = alt_key(base, min, &alt[..m], mask);
            cnt += 1;
        }

        if keys[1..cnt]
            .iter()
            .any(|&k| chain_range(chains, k).is_none())
        {
            if marks.is_empty() {
                marks.resize(grp.len(), false);
            }

            for &k in &keys[..cnt] {
                if let Some((s, end)) = chain_range(chains, k) {
                    marks[s..end].fill(true);
                }
            }
        }
    }

    if !marks.is_empty() {
        for (i, &marked) in marks.iter().enumerate() {
            if marked {
                result.push(grp[i]);
            }
        }
    }
}

fn phrase_chars(text: &str) -> Option<([char; 4], usize)> {
    let mut cs = text.chars();
    let (Some(c0), Some(c1)) = (cs.next(), cs.next()) else {
        return None;
    };
    let mut chars = [c0, c1, c0, c0];

    let n = match cs.next() {
        None => 2,
        Some(c2) => {
            chars[2] = c2;
            match cs.next() {
                None => 3,
                Some(c3) => {
                    chars[3] = cs.last().unwrap_or(c3);
                    4
                }
            }
        }
    };

    Some((chars, n))
}

fn slot_alt(stems: &StemMap, c: char, first: u8, third: Option<u8>) -> Option<u8> {
    let Some(stems) = stems.get(&c) else {
        return None;
    };

    let mut need = None;
    let mut saw_no = false;

    for &s in stems {
        if s[0] != first || third.is_some_and(|t| s[2] != t) {
            continue;
        }

        match seg_first_alt([s[0], s[1]]) {
            Some(t) => {
                if saw_no {
                    return None;
                }

                match need {
                    None => need = Some(t),
                    Some(x) if x == t => {}
                    _ => return None,
                }
            }
            None => {
                if need.is_some() {
                    return None;
                }
                saw_no = true;
            }
        }
    }

    need
}

fn seg_first_alt(seg: [u8; 2]) -> Option<u8> {
    full_alt_target(seg).map(|t| t[0]).filter(|&t| t != seg[0])
}

fn full_alt_target(seg: [u8; 2]) -> Option<[u8; 2]> {
    Some(match seg {
        [b'f', b'e'] => [b'q', b'e'],
        [b'q', b'e'] => [b'f', b'e'],
        [b'f', b'z'] => [b'q', b'z'],
        [b'q', b'z'] => [b'f', b'z'],
        [b'j', b'e'] => [b'w', b'e'],
        [b'w', b'e'] => [b'j', b'e'],
        [b'j', b'z'] => [b'w', b'z'],
        [b'w', b'z'] => [b'j', b'z'],
        [b'e', b'm'] => [b'e', b'x'],
        [b'e', b'x'] => [b'e', b'm'],
        [b'f', b'm'] => [b'f', b'x'],
        [b'f', b'x'] => [b'f', b'm'],
        [b'g', b'm'] => [b'g', b'x'],
        [b'g', b'x'] => [b'g', b'm'],
        [b'h', b'm'] => [b'h', b'x'],
        [b'h', b'x'] => [b'h', b'm'],
        [b'k', b'm'] => [b'k', b'x'],
        [b'k', b'x'] => [b'k', b'm'],
        [b'w', b'm'] => [b'w', b'x'],
        [b'w', b'x'] => [b'w', b'm'],
        [b'f', b'h'] => [b'q', b'h'],
        _ => return None,
    })
}

fn alt_key(mut base: [u8; 4], len: usize, alt: &[(usize, usize, [u8; 2])], mask: u32) -> u32 {
    for (i, &(p, l, t)) in alt.iter().enumerate() {
        if mask & 1u32 << i != 0 {
            base[p..p + l].copy_from_slice(&t[..l]);
        }
    }

    pack_prefix(&base[..len])
}

fn chain_range(chains: &[(u32, usize, usize)], k: u32) -> Option<(usize, usize)> {
    let Ok(i) = chains.binary_search_by_key(&k, |c| c.0) else {
        return None;
    };
    Some((chains[i].1, chains[i].2))
}

fn prefix_key(code: &str, len: usize) -> Option<u32> {
    code.as_bytes().get(..len).map(pack_prefix)
}

fn pack_prefix(b: &[u8]) -> u32 {
    b.iter().fold(0, |k, &c| k << 8 | u32::from(c))
}
