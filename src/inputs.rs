use crate::{DynRes, entry::Phrase, keytao::stem, reader::visit_dict};
use std::path::Path;

/// 词库
pub(crate) type PhraseDict = Vec<Phrase>;

/// 读取并返回词库
pub(crate) fn load_phrase_dict(path: &Path) -> DynRes<PhraseDict> {
    let mut phrases = PhraseDict::with_capacity(65536);
    visit_dict(path, |e| phrases.push(e))?;
    Ok(phrases)
}

/// 单字码表
pub(crate) type SingleDict = ahash::AHashMap<char, Vec<u32>>;

/// 读取并返回单字码表和词条总数
pub(crate) fn load_single_dict(path: &Path) -> DynRes<(SingleDict, usize)> {
    let mut singles = SingleDict::with_capacity(4096);
    let n = visit_dict(path, |e| {
        if let Some(stem) = stem(&e.code) {
            let stems = singles.entry(e.text).or_default();
            if !stems.contains(&stem) {
                stems.push(stem);
            }
        }
    })?;
    Ok((singles, n))
}
