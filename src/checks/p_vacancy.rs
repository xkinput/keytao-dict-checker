use crate::{Phrase, keytao::*};

const BITS: usize = 27usize.pow(MAX_CODE_N as u32 - 1);
const WORDS: usize = (BITS + 63) / 64;

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&str> {
    let mut used = vec![0; WORDS];

    for e in phrases {
        if e.code.len() < MAX_CODE_N {
            insert(&mut used, e.code.bytes().fold(0, append_key));
        }
    }

    let mut seen = vec![0; WORDS];
    let mut result = vec![];

    for e in phrases {
        let n = e.code.len();
        let min = phrase_min_code_len(&e.text);
        if n <= min {
            continue;
        }

        let mut k = 0;
        for (i, c) in e.code.bytes().enumerate().take(n - 1) {
            k = append_key(k, c);
            if i + 1 >= min && used[k >> 6] & (1u64 << (k & 63)) == 0 && insert(&mut seen, k) {
                result.push(&e.code[..i + 1]);
            }
        }
    }

    result.sort_unstable();
    result
}

fn append_key(key: usize, c: u8) -> usize {
    key * 27 + usize::from(c - b'a' + 1)
}

fn insert(bits: &mut [u64], key: usize) -> bool {
    let mask = 1u64 << (key & 63);
    let word = &mut bits[key >> 6];
    let new = *word & mask == 0;
    *word |= mask;
    new
}
