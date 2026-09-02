use crate::{DynRes, entry::Phrase, reader::visit_dict};
use std::path::Path;

/// 词库
pub(crate) type PhraseDict = Vec<Phrase>;

/// 读取并返回词库和词条总数
pub(crate) fn load_phrase_dict(path: &Path) -> DynRes<(PhraseDict, usize)> {
    let mut phrases = PhraseDict::with_capacity(65536);
    let cnt = visit_dict::<String, _>(path, |e| phrases.push(e))?;
    Ok((phrases, cnt))
}

/// 单字码表
pub(crate) type SingleDict = ahash::AHashMap<char, Vec<[char; 3]>>;

/// 读取并返回单字码表和词条总数
pub(crate) fn load_single_dict(path: &Path) -> DynRes<(SingleDict, usize)> {
    let mut singles = SingleDict::with_capacity(4096);

    let cnt = visit_dict::<char, _>(path, |e| {
        let mut chars = e.code.chars();
        let stem = match (chars.next(), chars.next(), chars.next()) {
            (Some(a), Some(b), Some(c)) => [a, b, c],
            _ => return,
        };
        let codes = singles.entry(e.text).or_default();
        if !codes.contains(&stem) {
            codes.push(stem);
        }
    })?;

    Ok((singles, cnt))
}
