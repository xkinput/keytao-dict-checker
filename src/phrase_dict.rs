use crate::{DynResult, dict::DictLoader, entry::Phrase, trie::Trie};
use rustc_hash::FxHashMap;

type TextMap = FxHashMap<std::rc::Rc<str>, Vec<usize>>;

pub(crate) struct PhraseDict {
    entries: Vec<Phrase>,
    texts: TextMap,
    codes: Trie<usize>,
}

impl DictLoader for PhraseDict {
    fn parse_lines<'a>(lines: &mut impl Iterator<Item = (usize, &'a str)>) -> DynResult<Self> {
        let mut entries = Vec::with_capacity(65536);
        for (i, l) in lines {
            if let Some(phrase) = Phrase::new(i + 1, l)? {
                entries.push(phrase);
            }
        }
        let cnt = entries.len();
        if cnt == 0 {
            return Err("词库为空".into());
        }

        let mut texts: TextMap = FxHashMap::with_capacity_and_hasher(cnt, Default::default());
        let mut codes = Trie::with_capacity(4 * cnt);
        for (i, e) in entries.iter().enumerate() {
            texts.entry(e.text.clone()).or_default().push(i);
            codes.insert(&e.code, i);
        }

        Ok(Self {
            entries,
            texts,
            codes,
        })
    }
}

impl PhraseDict {
    pub(crate) fn for_each_vacant_code(&self, f: impl FnMut(&str)) {
        self.codes.for_each_vacant(f)
    }
}
