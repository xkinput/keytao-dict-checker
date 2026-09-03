use ahash::AHashSet;
use rayon::prelude::*;

pub(crate) fn find_vacant_codes(phrases: &[crate::entry::Phrase]) -> AHashSet<&str> {
    let mut used = AHashSet::with_capacity(phrases.len());
    used.extend(phrases.iter().map(|e| e.code.as_str()));
    phrases
        .par_iter()
        .fold(AHashSet::new, |mut set, e| {
            let len = e.code.len();
            let xm_len = crate::keytao::xm_len(&e.code);
            if 0 < xm_len && xm_len < len {
                for i in (len - xm_len)..len {
                    let head = &e.code[..i];
                    if !used.contains(head) && !set.contains(head) {
                        set.insert(head);
                    }
                }
            }
            set
        })
        .reduce(AHashSet::new, |mut a, mut b| {
            if a.len() < b.len() {
                std::mem::swap(&mut a, &mut b);
            }
            a.extend(b);
            a
        })
}
