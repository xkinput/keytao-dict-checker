use crate::{DynResult, dict::DictLoader, entry::Phrase, single_dict::SingleDict, trie::Trie};
use rustc_hash::FxHashMap;

type TextMap = FxHashMap<std::rc::Rc<str>, Vec<usize>>;

pub(crate) struct PhraseDict {
    pool: Vec<Phrase>,
    texts: TextMap,
    codes: Trie<usize>,
}

impl DictLoader for PhraseDict {
    fn parse_lines<'a>(lines: &mut impl Iterator<Item = (usize, &'a str)>) -> DynResult<Self> {
        let mut pool = Vec::with_capacity(65536);
        for (i, l) in lines {
            if let Some(phrase) = Phrase::new(i + 1, l)? {
                pool.push(phrase);
            }
        }
        let cnt = pool.len();
        if cnt == 0 {
            return Err("词库为空".into());
        }

        let mut texts: TextMap = FxHashMap::with_capacity_and_hasher(cnt, Default::default());
        let mut codes = Trie::with_capacity(4 * cnt);
        for (i, e) in pool.iter().enumerate() {
            texts.entry(e.text.clone()).or_default().push(i);
            codes.insert(&e.code, i);
        }

        Ok(Self { pool, texts, codes })
    }
}

impl PhraseDict {
    pub(crate) fn vacant_codes(&self) -> Vec<String> {
        self.codes.vacant_codes()
    }

    pub(crate) fn err_encodings(&self, single: &SingleDict) -> impl Iterator<Item = &Phrase> {
        self.pool
            .iter()
            .filter(|phrase| single.is_err_encoding(phrase))
    }
}
