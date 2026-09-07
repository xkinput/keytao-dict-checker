use crate::{entry::Phrase, keytao::*};
use ahash::AHashSet;

pub(crate) fn check(phrases: &[Phrase]) -> Vec<&str> {
    let mut used = AHashSet::with_capacity(phrases.len());
    used.extend(
        phrases
            .iter()
            .filter(|p| p.code.len() < MAX_CODE_LEN)
            .map(|p| p.code.as_str()),
    );

    let mut vacant = AHashSet::new();
    for p in phrases {
        vacant.extend(shorter(&p.code).filter(|&c| !used.contains(c)));
    }

    let mut vec = Vec::from_iter(vacant);
    vec.sort_unstable();
    vec
}
