use crate::{DynResult, dict::DictLoader, entry::Phrase, single_dict::SingleDict, trie::Trie};
use rustc_hash::FxHashMap;

pub(crate) struct PhraseDict {
    pool: Vec<Phrase>,
    texts: FxHashMap<std::rc::Rc<str>, Vec<usize>>,
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

        let mut texts = FxHashMap::with_capacity_and_hasher(cnt, Default::default());
        let mut codes = Trie::with_capacity(4 * cnt);
        for (i, e) in pool.iter().enumerate() {
            texts.entry(e.text.clone()).or_insert_with(Vec::new).push(i);
            codes.insert(&e.code, i);
        }

        Ok(Self { pool, texts, codes })
    }
}

impl PhraseDict {
    pub(crate) fn redundancies(&self) -> Vec<&Phrase> {
        let mut result = vec![];
        for (text, indexes) in self.texts.iter().filter(|(_, v)| v.len() > 1) {
            let stem_len = if text.chars().count() == 3 { 3 } else { 4 };
            let mut groups = FxHashMap::with_capacity_and_hasher(indexes.len(), Default::default());
            for phrase in indexes.iter().map(|&i| &self.pool[i]) {
                let code = &phrase.code;
                let i = code
                    .char_indices()
                    .nth(stem_len)
                    .map_or(code.len(), |(i, _)| i);
                groups
                    .entry(&code[..i])
                    .or_insert_with(Vec::new)
                    .push(phrase);
            }
            for (_, phrases) in groups.iter().filter(|(_, v)| v.len() > 1) {
                result.extend(phrases);
            }
        }
        result
    }

    pub(crate) fn vacant_codes(&self) -> Vec<String> {
        self.codes.vacant_codes()
    }

    pub(crate) fn err_encodings(&self, single: &SingleDict) -> impl Iterator<Item = &Phrase> {
        self.pool
            .iter()
            .filter(|phrase| single.is_err_encoding(phrase))
    }
}
