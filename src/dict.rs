use crate::trie::Trie;
use rustc_hash::FxHashMap;
use std::{fs, rc::Rc};

struct Entry {
    line_idx: usize,
    text: Rc<str>,
    code: Rc<str>,
    weight: f64,
}

pub(crate) struct Entries {
    pool: Vec<Entry>,
    words: FxHashMap<Rc<str>, Vec<usize>>,
    codes: Trie<usize>,
}

impl Entries {
    pub(crate) fn read(dict_path: &str) -> crate::DynResult<Self> {
        let s = fs::read_to_string(dict_path)?;
        let mut lines = s.lines().enumerate();

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
        if let Some(cols) = header.columns
            && cols != expected
        {
            return Err(format!("不支持此词库列配置：应为{expected:?}，实为{cols:?}").into());
        }

        let mut pool = Vec::with_capacity(65536);
        for (i, l) in lines {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let mut parts = l.splitn(4, '\t');
            let Some(text) = parts.next() else {
                continue;
            };
            let Some(code) = parts.next() else {
                continue;
            };
            pool.push(Entry {
                line_idx: i,
                text: text.into(),
                code: code.into(),
                weight: parts.next().map_or(0.0, |s| {
                    let (n, d) = s.strip_suffix('%').map_or((s, 1.0), |n| (n, 100.0));
                    n.parse::<f64>().map_or(0.0, |v| v / d)
                }),
            });
        }
        let cnt = pool.len();
        if cnt == 0 {
            return Err("词库为空".into());
        }

        let mut words: FxHashMap<_, Vec<_>> =
            FxHashMap::with_capacity_and_hasher(cnt, Default::default());
        let mut codes = Trie::with_capacity(4 * cnt);
        for (i, e) in pool.iter().enumerate() {
            words.entry(e.text.clone()).or_default().push(i);
            codes.insert(&e.code, i);
        }

        Ok(Self { pool, words, codes })
    }
}
