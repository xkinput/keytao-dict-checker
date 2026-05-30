use crate::{entry::Phrase, trie::Trie};
use rustc_hash::FxHashMap;

type TextMap = FxHashMap<std::rc::Rc<str>, Vec<usize>>;

pub(crate) struct PhraseDict {
    entries: Vec<Phrase>,
    texts: TextMap,
    codes: Trie<usize>,
}

impl PhraseDict {
    pub(crate) fn load(path: &str) -> crate::DynResult<Self> {
        let s = std::fs::read_to_string(path)?;
        let mut lines = s.lines().enumerate();
        Self::verify_and_rem_header(&mut lines)?;

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

    pub(crate) fn verify_and_rem_header(
        lines: &mut std::iter::Enumerate<core::str::Lines>,
    ) -> crate::DynResult<()> {
        // 丢弃文件头前的内容
        while lines.next().is_some_and(|(_, l)| l != "---") {}

        let mut yaml = String::with_capacity(1024);
        loop {
            match lines.next() {
                None => return Err("词库文件头缺失或未闭合".into()),
                Some((_, "...")) => break,
                Some((_, l)) if yaml.len() < 65536 => yaml.push_str(l),
                _ => return Err("词库文件头过长，疑似未闭合".into()),
            }
            yaml.push('\n');
        }

        #[derive(serde::Deserialize)]
        struct Header {
            columns: Option<Vec<String>>,
        }
        let expected = ["text", "code", "weight"];
        let header: Header = yaml_serde::from_str(&yaml)?;
        match header.columns {
            Some(cols) if cols != expected => {
                Err(format!("词库列配置无效：应为{expected:?}，实为{cols:?}").into())
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn for_each_vacant_code(&self, f: impl FnMut(&str)) {
        self.codes.for_each_vacant(f)
    }
}
