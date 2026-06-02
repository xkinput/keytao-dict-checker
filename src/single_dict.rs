use crate::{DynResult, dict::DictLoader, entry};
use rustc_hash::FxHashMap;

pub(crate) struct SingleDict {
    stems: FxHashMap<char, Vec<[char; 3]>>,
}

impl DictLoader for SingleDict {
    fn parse_lines<'a>(lines: &mut impl Iterator<Item = (usize, &'a str)>) -> DynResult<Self> {
        let mut stems = FxHashMap::with_capacity_and_hasher(8192, Default::default());
        for (i, l) in lines {
            if let Some(single) = entry::Single::new(i + 1, l)? {
                let mut chars = single.code.chars();
                if let Some(a) = chars.next()
                    && let Some(b) = chars.next()
                    && let Some(c) = chars.next()
                {
                    stems
                        .entry(single.text)
                        .or_insert_with(Vec::new)
                        .push([a, b, c]);
                }
            }
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

impl SingleDict {
    pub(crate) fn is_err_encoding(&self, phrase: &entry::Phrase) -> bool {
        let mut text_chars = phrase.text.chars();
        let mut valid_text: Vec<_> = (&mut text_chars).take(4).collect();
        if valid_text.len() < 2 {
            return true;
        }
        if let Some(c) = text_chars.next_back() {
            valid_text[3] = c;
        }

        let code: Vec<_> = phrase.code.chars().collect();
        if code.len() < 3 || (valid_text.len() != 3 && code.len() < 4) || 6 < code.len() {
            return true;
        }

        let Some(codes0) = self.stems.get(&valid_text[0]) else {
            return true;
        };
        let Some(codes1) = self.stems.get(&valid_text[1]) else {
            return true;
        };
        match valid_text.get(2..) {
            Some([]) => !codes0.iter().any(|a| {
                codes1.iter().any(|b| {
                    code.starts_with(&[a[0], a[1], b[0], b[1]])
                        && code.get(4).is_none_or(|&x| x == a[2])
                        && code.get(5).is_none_or(|&x| x == b[2])
                })
            }),
            Some([c2]) if let Some(codes2) = self.stems.get(c2) => !codes0.iter().any(|a| {
                codes1.iter().any(|b| {
                    codes2.iter().any(|c| {
                        code.starts_with(&[a[0], b[0], c[0]])
                            && code.get(3).is_none_or(|&x| x == a[2])
                            && code.get(4).is_none_or(|&x| x == b[2])
                            && code.get(5).is_none_or(|&x| x == c[2])
                    })
                })
            }),
            Some([c2, c3])
                if let Some(codes2) = self.stems.get(c2)
                    && let Some(codes3) = self.stems.get(c3) =>
            {
                !codes0.iter().any(|a| {
                    codes1.iter().any(|b| {
                        codes2.iter().any(|c| {
                            codes3.iter().any(|d| {
                                code.starts_with(&[a[0], b[0], c[0], d[0]])
                                    && code.get(4).is_none_or(|&x| x == a[2])
                                    && code.get(5).is_none_or(|&x| x == b[2])
                            })
                        })
                    })
                })
            }
            _ => true,
        }
    }
}
