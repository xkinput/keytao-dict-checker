use crate::entry::{Phrase, Single};
use rustc_hash::FxHashMap;

type StemMap = FxHashMap<char, Vec<String>>;

pub(crate) struct SingleDict {
    stems: StemMap,
}

impl SingleDict {
    pub(crate) fn load(path: &str) -> crate::DynResult<Self> {
        let s = std::fs::read_to_string(path)?;
        let mut lines = s.lines().enumerate();
        crate::phrase_dict::PhraseDict::verify_and_rem_header(&mut lines)?;

        let mut stems: StemMap = FxHashMap::with_capacity_and_hasher(8192, Default::default());
        for (i, l) in lines {
            if let Some(single) = Single::new(i + 1, l)? {
                let code = single.code;
                let mut ci = code.char_indices();
                if ci.nth(2).is_none() {
                    continue;
                }
                let end = ci.next().map_or(code.len(), |(i, _)| i);
                let stem = code[..end].to_string();
                stems.entry(single.text).or_default().push(stem);
            }
        }
        if stems.is_empty() {
            return Err("单字码表为空".into());
        }

        for v in stems.values_mut() {
            v.sort_unstable();
            v.dedup();
        }

        Ok(Self { stems })
    }

    pub(crate) fn is_valid_encoding(&self, phrase: &Phrase) -> bool {
        let mut text: Vec<_> = phrase.text.chars().collect();
        if text.len() < 2 {
            return false;
        }
        if text.len() > 4 {
            text[3] = text.pop().unwrap();
            text.truncate(4);
        }
        todo!()
    }
}
