use crate::{DynResult, dict::DictLoader, entry};
use rustc_hash::FxHashMap;

type StemMap = FxHashMap<char, Vec<String>>;

pub(crate) struct SingleDict {
    stems: StemMap,
}

impl DictLoader for SingleDict {
    fn parse_lines<'a>(lines: &mut impl Iterator<Item = (usize, &'a str)>) -> DynResult<Self> {
        let mut stems: StemMap = FxHashMap::with_capacity_and_hasher(8192, Default::default());
        for (i, l) in lines {
            if let Some(single) = entry::Single::new(i + 1, l)? {
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
}

impl SingleDict {
    pub(crate) fn is_valid_encoding(&self, phrase: &entry::Phrase) -> bool {
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
