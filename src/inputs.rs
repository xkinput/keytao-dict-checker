use crate::{entry::*, keytao::*, reader::*, *};
use std::path::Path;

/// 码表
pub(crate) type Dict<T> = Vec<Entry<T>>;

/// 读取并返回码表
pub(crate) fn load_dict<T: EntryText>(path: &Path) -> DynRes<Dict<T>> {
    let mut dict = Dict::with_capacity(4096);
    for_each_entry(path, |e| dict.push(e))?;
    Ok(dict)
}

/// 构词码表
pub(crate) type StemMap = ahash::AHashMap<char, Vec<Stem>>;

/// 读取并返回构词码表和词条总数
pub(crate) fn load_stems(path: &Path) -> DynRes<(StemMap, usize)> {
    let mut map = StemMap::with_capacity(4096);
    let mut n = 0;

    for_each_entry(path, |s| {
        n += 1;
        if let Some(stem) = stem(&s.code) {
            let stems = map.entry(s.text).or_default();
            if !stems.contains(&stem) {
                stems.push(stem);
            }
        }
    })?;

    Ok((map, n))
}
