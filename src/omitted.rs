use crate::{entry::Phrase, inputs::SingleDict, keytao::*};
use rayon::prelude::*;

type FirstInfo = ahash::AHashMap<char, [u8; 26]>;

pub(crate) fn check<'a>(
    phrases: &'a [Phrase],
    singles: &SingleDict,
) -> (Vec<&'a str>, Vec<&'a str>) {
    let first = build_first_info(singles);

    let mut pairs: Vec<_> = phrases
        .par_iter()
        .filter_map(|p| {
            let key = ym_key(&p.code)?;
            let len = if key >> 24 == 0 { 3 } else { 4 };
            let ok = len == phrase_ym_len(&p.text);
            ok.then(|| (p.text.as_str(), key))
        })
        .collect();
    pairs.par_sort_unstable();
    pairs.dedup();

    let mut certain = vec![];
    let mut possible = vec![];
    for group in pairs.chunk_by(|a, b| a.0 == b.0) {
        let text = group[0].0;
        match classify(text, group, &first) {
            Some(true) => certain.push(text),
            Some(false) => possible.push(text),
            None => {}
        }
    }
    (certain, possible)
}

fn build_first_info(singles: &SingleDict) -> FirstInfo {
    let mut info = ahash::AHashMap::with_capacity(singles.len());

    for (&c, stems) in singles {
        let mut trigger = [false; 26];
        let mut non = [false; 26];
        let mut target = [0u8; 26];

        for &s in stems {
            let seg = stem_ym(s);
            let i = seg[0].wrapping_sub(b'a') as usize;
            if i < 26 {
                if let Some(t) = first_alt_target(seg) {
                    trigger[i] = true;
                    target[i] = t - b'a';
                } else {
                    non[i] = true;
                }
            }
        }

        let mut states = [0u8; 26];
        for i in 0..26 {
            if trigger[i] {
                states[i] = 1 + u8::from(non[i]) << 5 | target[i];
            }
        }

        info.insert(c, states);
    }

    info
}

fn classify(text: &str, group: &[(&str, u32)], first: &FirstInfo) -> Option<bool> {
    let mut cs = text.chars();
    let c0 = cs.next()?;
    let c1 = cs.next()?;
    let mut chars = [c0, c1, c0, c0];

    let n = match cs.next() {
        None => 2,
        Some(c2) => {
            chars[2] = c2;
            if cs.next().is_none() {
                3
            } else {
                chars[3] = text.chars().next_back()?;
                4
            }
        }
    };

    let mut possible = false;

    for &(_, key) in group {
        let b = key.to_le_bytes();
        let mut alt = [(0usize, 0usize, [0u8; 2], false); 4];
        let mut m = 0;

        if n == 2 {
            for (p, seg) in [(0usize, [b[0], b[1]]), (2usize, [b[2], b[3]])] {
                if let Some(t) = full_alt_target(seg) {
                    alt[m] = (p, 2, t, true);
                    m += 1;
                }
            }
        } else {
            for i in 0..n {
                if let Some((t, d)) = first_state(first, chars[i], b[i]) {
                    alt[m] = (i, 1, [t, 0], d);
                    m += 1;
                }
            }
        }

        for mask in 1..1u32 << m {
            let mut nb = b;
            let mut def = true;

            for (i, &(p, l, t, d)) in alt[..m].iter().enumerate() {
                if mask & 1u32 << i != 0 {
                    nb[p..p + l].copy_from_slice(&t[..l]);
                    def = def && d;
                }
            }

            let cand = u32::from_le_bytes(nb);
            if group.binary_search_by_key(&cand, |&(_, k)| k).is_err() {
                if def {
                    return Some(true);
                }
                possible = true;
            }
        }
    }

    possible.then_some(false)
}

fn first_state(info: &FirstInfo, c: char, x: u8) -> Option<(u8, bool)> {
    let st = *info.get(&c)?.get(x.wrapping_sub(b'a') as usize)?;
    match st >> 5 {
        0 => None,
        kind => Some((b'a' + (st & 31), kind == 1)),
    }
}
