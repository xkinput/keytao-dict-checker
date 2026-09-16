use crate::{entry::Single, keytao::*};

const PAIRS: [(u8, u8, u8, u8); 10] = [
    (b'f', b'e', b'q', b'e'),
    (b'f', b'z', b'q', b'z'),
    (b'j', b'e', b'w', b'e'),
    (b'j', b'z', b'w', b'z'),
    (b'e', b'm', b'e', b'x'),
    (b'f', b'm', b'f', b'x'),
    (b'g', b'm', b'g', b'x'),
    (b'h', b'm', b'h', b'x'),
    (b'k', b'm', b'k', b'x'),
    (b'w', b'm', b'w', b'x'),
];

fn ix(c: u8) -> usize {
    usize::from(c - b'a')
}

pub(crate) fn check(singles: &[Single]) -> Vec<&Single> {
    let mut evid = [[0; CODE_ELEMS]; CODE_ELEMS];
    let mut one = [0; CODE_ELEMS];
    let mut req = [[0; CODE_ELEMS]; CODE_ELEMS];

    for (i, (a, b, c, d)) in PAIRS.into_iter().enumerate() {
        let m1 = 1u32 << i * 2;
        let m2 = m1 << 1;
        let (a, b, c, d) = (ix(a), ix(b), ix(c), ix(d));
        evid[a][b] = m1;
        evid[c][d] = m2;
        one[a] |= m1;
        one[c] |= m2;
        req[a][b] = m2;
        req[c][d] = m1;
    }

    let m = 1u32 << PAIRS.len() * 2;
    let (q, h, f) = (ix(b'q'), ix(b'h'), ix(b'f'));
    evid[q][h] = m;
    one[q] |= m;
    req[f][h] = m;

    let mut masks = ahash::AHashMap::<_, u32>::with_capacity(singles.len() / 2);
    for e in singles {
        let m = match e.code.as_bytes() {
            [a] => one[ix(*a)],
            [a, b, ..] => evid[ix(*a)][ix(*b)],
            _ => 0,
        };
        *masks.entry(e.text).or_default() |= m;
    }

    let mut res: Vec<_> = singles
        .iter()
        .filter(|e| match e.code.as_bytes() {
            [a, b, ..] => {
                let r = req[ix(*a)][ix(*b)];
                r != 0 && masks[&e.text] & r == 0
            }
            _ => false,
        })
        .collect();
    res.sort_unstable_by_key(|e| (e.text, &e.code));
    res
}
