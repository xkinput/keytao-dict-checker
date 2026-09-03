use crate::{entry::Phrase, keytao::ym};
use ahash::AHashMap;
use rayon::prelude::*;

pub(crate) fn find_redundant(phrases: &[Phrase]) -> Vec<&Phrase> {
    let redundant = phrases
        .par_iter()
        .fold(AHashMap::new, |mut map, e| {
            let k = (e.text.as_str(), ym(&e.code));
            map.entry(k).and_modify(|dup| *dup = true).or_insert(false);
            map
        })
        .reduce(AHashMap::new, |mut a, mut b| {
            if a.len() < b.len() {
                std::mem::swap(&mut a, &mut b);
            }
            for (k, v) in b {
                a.entry(k).and_modify(|dup| *dup = true).or_insert(v);
            }
            a
        });
    phrases
        .iter()
        .filter(|e| {
            let k = (e.text.as_str(), ym(&e.code));
            redundant.get(&k).is_some_and(|dup| *dup)
        })
        .collect()
}
