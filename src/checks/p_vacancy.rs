use crate::{entry::Phrase, keytao::*};

const N3: usize = CODE_ELEMS * CODE_ELEMS * CODE_ELEMS;
const N4: usize = N3 * CODE_ELEMS;
const N5: usize = N4 * CODE_ELEMS;
const OFFSET: [usize; MAX_CODE_N] = [0, 0, 0, 0, N3, N3 + N4];

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&str> {
    let mut seen = vec![0; (N3 + N4 + N5).div_ceil(64)];

    for e in phrases {
        let n = e.code.len();
        if n < MAX_CODE_N {
            let mut val = 0;
            for b in e.code.bytes() {
                val = val * CODE_ELEMS + (b - b'a') as usize;
            }
            mark(&mut seen, OFFSET[n] + val);
        }
    }

    let mut result = Vec::new();

    for e in phrases {
        let n = e.code.len();
        let min = phrase_min_code_len(&e.text);
        if n > min {
            let mut v = 0;
            for (i, b) in e.code.bytes().take(n - 1).enumerate() {
                v = v * CODE_ELEMS + (b - b'a') as usize;
                let len = i + 1;
                if len >= min && mark(&mut seen, OFFSET[len] + v) {
                    result.push(&e.code[..len]);
                }
            }
        }
    }

    result.sort_unstable();
    result
}

fn mark(blocks: &mut [u64], i: usize) -> bool {
    let block = &mut blocks[i >> 6];
    let mask = 1 << (i & 63);
    let new = *block & mask == 0;
    *block |= mask;
    new
}
