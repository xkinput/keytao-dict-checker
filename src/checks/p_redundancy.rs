use crate::{Phrase, keytao::*};
use rayon::prelude::*;

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&Phrase> {
    let mut entries = Vec::from_iter(phrases);
    entries.par_sort_unstable_by_key(|e| (&e.text, &e.code));
    entries
        .chunk_by(|a, b| a.text == b.text)
        .flat_map(|grp| {
            let n = phrase_min_code_len(&grp[0].text);
            grp.chunk_by(move |a, b| a.code[..n] == b.code[..n])
        })
        .filter(|grp| grp.len() > 1)
        .map(|grp| *grp.iter().max_by_key(|e| e.code.len()).unwrap())
        .collect()
}
