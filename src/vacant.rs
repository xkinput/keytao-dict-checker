use ahash::AHashSet;
use rayon::prelude::*;

fn find_vacant_codes(phrases: &[crate::entry::Phrase]) -> AHashSet<String> {
    let mut used = AHashSet::with_capacity(phrases.len());
    used.extend(phrases.iter().map(|entry| entry.code.as_str()));

    phrases
        .par_iter()
        .fold(AHashSet::new, |mut result, entry| {
            let code = entry.code.as_str();
            let bytes = code.as_bytes();
            let len = bytes.len();

            let mut tail = 0;
            while tail < len && matches!(bytes[len - 1 - tail], b'a' | b'i' | b'o' | b'u' | b'v') {
                tail += 1;
            }

            if 0 < tail && tail < len {
                let base = len - tail;
                for i in base..len {
                    let prefix = &code[..i];
                    if !used.contains(prefix) && !result.contains(prefix) {
                        result.insert(prefix.to_owned());
                    }
                }
            }

            result
        })
        .reduce(AHashSet::new, |mut a, mut b| {
            if a.len() < b.len() {
                std::mem::swap(&mut a, &mut b);
            }
            a.extend(b);
            a
        })
}
