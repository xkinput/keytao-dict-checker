use crate::{entry::Phrase, keytao::ym};
use rayon::prelude::*;

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&Phrase> {
    let mut vec = Vec::from_iter(phrases);
    vec.par_sort_unstable_by_key(|&p| (&p.text, ym(&p.code), p.code.len()));
    vec.windows(2)
        .filter(|w| w[0].text == w[1].text && ym(&w[0].code) == ym(&w[1].code))
        .map(|w| w[1])
        .collect()
}
