use crate::{keytao::*, reader::*, *};
use std::path::Path;

/// 单字码表
pub(crate) type SingleDict = Vec<Single>;
/// 词组码表
pub(crate) type PhraseDict = Vec<Phrase>;

/// 读取并返回码表
pub(crate) fn load_dict<T: EntryText>(path: &Path) -> DynRes<Vec<Entry<T>>> {
    let mut dict = Vec::with_capacity(4096);
    for_each_entry(path, |e| dict.push(e))?;
    Ok(dict)
}

/// 构词码表
pub(crate) type StemMap = ahash::AHashMap<char, Vec<Stem>>;

/// 读取并返回构词码表
pub(crate) fn load_stems(path: &Path) -> DynRes<StemMap> {
    let mut map = StemMap::with_capacity(4096);
    for_each_entry(path, |e| {
        if let Some(stem) = stem(&e.code) {
            let stems = map.entry(e.text).or_default();
            if !stems.contains(&stem) {
                stems.push(stem);
            }
        }
    })?;
    Ok(map)
}
