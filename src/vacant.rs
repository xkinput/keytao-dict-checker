use crate::{entry::Phrase, keytao::ym_len};
use rayon::prelude::*;

// 键序即字符串序：6×5bit，0 终止，1..26 为 a..z
const SLOTS: usize = 6;
const BITS: u32 = 5;
const MASK: u32 = (1u32 << BITS) - 1;

pub(crate) fn check(phrases: &[Phrase]) -> Vec<String> {
    let used = used_keys(phrases);
    let cands = cand_keys(phrases);
    diff(cands, used).into_par_iter().map(decode).collect()
}

fn used_keys(phrases: &[Phrase]) -> Vec<u32> {
    let mut keys: Vec<_> = phrases
        .par_iter()
        .filter(|p| matches!(p.code.len(), 3..=5))
        .map(|p| key(&p.code))
        .collect();
    keys.par_sort_unstable();
    keys.dedup();
    keys
}

fn cand_keys(phrases: &[Phrase]) -> Vec<u32> {
    let mut keys = phrases
        .par_iter()
        .fold(Vec::new, |mut acc, p| {
            let code = &p.code;
            let base = ym_len(code);
            let end = code.len().min(SLOTS);
            if base < end {
                let k = key(code);
                for len in base..end {
                    acc.push(prefix(k, len));
                }
            }
            acc
        })
        .reduce(Vec::new, |mut a, b| {
            a.extend(b);
            a
        });
    keys.par_sort_unstable();
    keys.dedup();
    keys
}

fn diff(mut cands: Vec<u32>, used: Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    cands.retain(|&k| {
        while i < used.len() && used[i] < k {
            i += 1;
        }
        i == used.len() || used[i] != k
    });
    cands
}

fn key(code: &str) -> u32 {
    let bytes = &code.as_bytes()[..code.len().min(SLOTS)];
    let mut k = 0u32;
    for &b in bytes {
        k = (k << BITS) | u32::from(b - b'a' + 1);
    }
    k << (BITS * (SLOTS as u32 - bytes.len() as u32))
}

fn prefix(k: u32, len: usize) -> u32 {
    let shift = BITS * (SLOTS as u32 - len as u32);
    (k >> shift) << shift
}

fn decode(k: u32) -> String {
    let mut out = String::with_capacity(SLOTS);
    for i in 0..SLOTS as u32 {
        let v = (k >> (BITS * (SLOTS as u32 - 1 - i))) & MASK;
        if v == 0 {
            break;
        }
        out.push(char::from(b'a' + (v as u8 - 1)));
    }
    out
}
