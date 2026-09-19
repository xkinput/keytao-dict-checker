use crate::{keytao::*, reader::*, *};

/// 单字码表
pub(crate) type SingleDict = Vec<Single>;

/// 词组码表
pub(crate) type PhraseDict = Vec<Phrase>;

pub(crate) fn load_dict<T: EntryText>(path: &Path) -> DynRes<Vec<Entry<T>>> {
    let mut dict = Vec::with_capacity(4096);
    for_each_entry(path, |e| dict.push(e))?;
    if dict.is_empty() {
        return Err("码表为空".into());
    }
    Ok(dict)
}

/// 构词码表
pub(crate) type StemMap = ahash::AHashMap<char, Vec<Stem>>;

pub(crate) fn get_stems(singles: &SingleDict) -> DynRes<StemMap> {
    let mut stems = StemMap::with_capacity(4096);
    for e in singles {
        if let Some(stem) = get_stem(&e.code) {
            let stems = stems.entry(e.text).or_default();
            if !stems.contains(&stem) {
                stems.push(stem);
            }
        }
    }
    if stems.is_empty() {
        return Err("无有效构词码".into());
    }
    Ok(stems)
}
