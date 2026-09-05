use crate::{entry::Phrase, inputs::SingleDict, keytao};
use Status::*;

#[derive(Clone, Copy)]
enum Status {
    Clean,
    Definite,
    Uncertain,
}

type Chain = [u8; 4];
type Ym = [u8; 2];
type Alt = (usize, usize, Ym);

struct Ctx {
    ym_len: usize,
    cnt: usize,
    cs: [char; 4],
}

impl Ctx {
    fn new(text: &str) -> Option<Self> {
        let mut cs = text.chars();
        let c0 = cs.next()?;
        let c1 = cs.next()?;
        Some(match (cs.next(), cs.next()) {
            (None, _) => Self {
                ym_len: 4,
                cnt: 2,
                cs: [c0, c1, c0, c0],
            },
            (Some(c2), None) => Self {
                ym_len: 3,
                cnt: 3,
                cs: [c0, c1, c2, c0],
            },
            (Some(c2), Some(c3)) => Self {
                ym_len: 4,
                cnt: 4,
                cs: [c0, c1, c2, cs.last().unwrap_or(c3)],
            },
        })
    }
}

pub(crate) fn find_alternate<'a>(
    phrases: &'a [Phrase],
    singles: &SingleDict,
) -> (Vec<&'a Phrase>, Vec<&'a Phrase>) {
    let mut idx: Vec<_> = (0..phrases.len()).collect();
    idx.sort_unstable_by_key(|&i| &phrases[i].text);
    let mut statuses = vec![Clean; phrases.len()];
    let mut chains = vec![];

    let mut i = 0;
    while i < idx.len() {
        let text = phrases[idx[i]].text.as_str();
        let mut j = i + 1;
        while j < idx.len() && phrases[idx[j]].text == text {
            j += 1;
        }

        let status = group_status(text, &idx[i..j], phrases, singles, &mut chains);
        for &p in &idx[i..j] {
            statuses[p] = status;
        }

        i = j;
    }

    let (mut definite, mut uncertain) = (vec![], vec![]);
    for (e, &status) in phrases.iter().zip(&statuses) {
        match status {
            Definite => definite.push(e),
            Uncertain => uncertain.push(e),
            Clean => {}
        }
    }

    (definite, uncertain)
}

fn group_status(
    text: &str,
    group: &[usize],
    phrases: &[Phrase],
    singles: &SingleDict,
    chains: &mut Vec<Chain>,
) -> Status {
    let Some(ctx) = Ctx::new(text) else {
        return Uncertain;
    };

    for &ch in &ctx.cs[..ctx.cnt] {
        if !singles.contains_key(&ch) {
            return Uncertain;
        }
    }

    chains.clear();
    let mut uncertain = false;

    for &i in group {
        let e = &phrases[i];
        let ym = keytao::ym(&e.code);

        if ym.len() == ctx.ym_len {
            let mut chain = [0; 4];
            chain[..ctx.ym_len].copy_from_slice(ym.as_bytes());
            chains.push(chain);
        } else {
            uncertain = true;
        }
    }

    chains.sort_unstable();
    chains.dedup();

    let present = chains.as_slice();
    for &chain in present {
        let status = if ctx.cnt == 2 {
            two_char_status(chain, &ctx, singles, present)
        } else {
            multi_char_status(chain, &ctx, singles, present)
        };

        match status {
            Definite => return Definite,
            Uncertain => uncertain = true,
            Clean => {}
        }
    }

    if uncertain { Uncertain } else { Clean }
}

fn two_char_status(chain: Chain, ctx: &Ctx, singles: &SingleDict, present: &[Chain]) -> Status {
    let mut alts = [(0, 0, [0; 2]); 2];
    let mut alt_cnt = 0;
    let mut uncertain = false;

    for i in 0..2 {
        let ym = [chain[2 * i], chain[2 * i + 1]];
        let Some(stems) = singles.get(&ctx.cs[i]) else {
            return Uncertain;
        };

        if !stems
            .iter()
            .any(|&[a, b, _]| a as u8 == ym[0] && b as u8 == ym[1])
        {
            uncertain = true;
            continue;
        }

        if let Some(t) = alternate(ym) {
            alts[alt_cnt] = (2 * i, 2, t);
            alt_cnt += 1;
        }
    }

    if definite_missing(chain, &alts[..alt_cnt], present) {
        Definite
    } else if uncertain {
        Uncertain
    } else {
        Clean
    }
}

fn multi_char_status(chain: Chain, ctx: &Ctx, singles: &SingleDict, present: &[Chain]) -> Status {
    let mut definite_alts = [(0, 0, [0; 2]); 4];
    let mut definite_len = 0;
    let mut uncertain_alts = [(0, 0, [0; 2]); 4];
    let mut uncertain_len = 0;
    let mut uncertain = false;

    for i in 0..ctx.cnt {
        let x = chain[i];
        let Some(stems) = singles.get(&ctx.cs[i]) else {
            return Uncertain;
        };

        let (mut matched, mut alt_cnt, mut target, mut same) = (0, 0, None, true);

        for &[a, b, _] in stems.iter() {
            if a as u8 != x {
                continue;
            }
            matched += 1;
            if let Some(t) = alternate([a as u8, b as u8]) {
                if t[0] != x {
                    alt_cnt += 1;
                    match target {
                        None => target = Some(t[0]),
                        Some(v) if v != t[0] => same = false,
                        _ => {}
                    }
                }
            }
        }

        if matched == 0 {
            uncertain = true;
            continue;
        }
        if alt_cnt == 0 {
            continue;
        }

        match target {
            Some(v) if same => {
                let alt = (i, 1, [v, 0]);
                if alt_cnt == matched {
                    definite_alts[definite_len] = alt;
                    definite_len += 1;
                } else {
                    uncertain_alts[uncertain_len] = alt;
                    uncertain_len += 1;
                }
            }
            _ => uncertain = true,
        }
    }

    if definite_missing(chain, &definite_alts[..definite_len], present) {
        return Definite;
    }

    if uncertain
        || (uncertain_len > 0
            && uncertain_missing(
                chain,
                &definite_alts[..definite_len],
                &uncertain_alts[..uncertain_len],
                present,
            ))
    {
        return Uncertain;
    }

    Clean
}

fn apply_alts(chain: &mut Chain, alts: &[Alt], mask: usize) {
    for (i, &(pos, len, ym)) in alts.iter().enumerate() {
        if (mask >> i) & 1 != 0 {
            chain[pos..pos + len].copy_from_slice(&ym[..len]);
        }
    }
}

fn definite_missing(chain: Chain, alts: &[Alt], present: &[Chain]) -> bool {
    for mask in 1..(1usize << alts.len()) {
        let mut candidate = chain;
        apply_alts(&mut candidate, alts, mask);
        if present.binary_search(&candidate).is_err() {
            return true;
        }
    }
    false
}

fn uncertain_missing(
    chain: Chain,
    definite_alts: &[Alt],
    uncertain_alts: &[Alt],
    present: &[Chain],
) -> bool {
    for definite_mask in 0..(1usize << definite_alts.len()) {
        for uncertain_mask in 1..(1usize << uncertain_alts.len()) {
            let mut candidate = chain;
            apply_alts(&mut candidate, definite_alts, definite_mask);
            apply_alts(&mut candidate, uncertain_alts, uncertain_mask);
            if present.binary_search(&candidate).is_err() {
                return true;
            }
        }
    }
    false
}

fn alternate(ym: Ym) -> Option<Ym> {
    match ym {
        [b'f', b'e'] => Some([b'q', b'e']),
        [b'q', b'e'] => Some([b'f', b'e']),

        [b'f', b'z'] => Some([b'q', b'z']),
        [b'q', b'z'] => Some([b'f', b'z']),

        [b'j', b'e'] => Some([b'w', b'e']),
        [b'w', b'e'] => Some([b'j', b'e']),

        [b'j', b'z'] => Some([b'w', b'z']),
        [b'w', b'z'] => Some([b'j', b'z']),

        [b'e', b'm'] => Some([b'e', b'x']),
        [b'e', b'x'] => Some([b'e', b'm']),

        [b'f', b'm'] => Some([b'f', b'x']),
        [b'f', b'x'] => Some([b'f', b'm']),

        [b'g', b'm'] => Some([b'g', b'x']),
        [b'g', b'x'] => Some([b'g', b'm']),

        [b'h', b'm'] => Some([b'h', b'x']),
        [b'h', b'x'] => Some([b'h', b'm']),

        [b'k', b'm'] => Some([b'k', b'x']),
        [b'k', b'x'] => Some([b'k', b'm']),

        [b'w', b'm'] => Some([b'w', b'x']),
        [b'w', b'x'] => Some([b'w', b'm']),

        [b'f', b'h'] => Some([b'q', b'h']),

        _ => None,
    }
}
