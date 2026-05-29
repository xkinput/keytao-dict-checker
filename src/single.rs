use rustc_hash::FxHashMap;

pub(crate) struct Single {
    stems: FxHashMap<char, Vec<String>>,
}

impl Single {
    pub(crate) fn load(path: &str) -> crate::DynResult<Self> {
        let s = std::fs::read_to_string(path)?;
        let mut lines = s.lines().enumerate();
        crate::dict::Dict::verify_and_rem_header(&mut lines)?;

        let mut stems: FxHashMap<_, Vec<_>> =
            FxHashMap::with_capacity_and_hasher(8192, Default::default());
        for (i, l) in lines {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let Ok(entry) = crate::entry::Entry::new(i + 1, l) else {
                continue;
            };
            let Some(single) = entry.single() else {
                continue;
            };
            let code = entry.code();
            let mut ci = code.char_indices();
            if ci.nth(2).is_none() {
                continue;
            }
            let end = ci.next().map_or(code.len(), |(i, _)| i);
            let stem = code[..end].to_string();

            stems.entry(single).or_default().push(stem);
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
