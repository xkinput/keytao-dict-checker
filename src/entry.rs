use crate::DynResult;
use std::{fmt::Display, rc::Rc};

pub(crate) struct Entry<T> {
    pub(crate) line_num: usize,
    pub(crate) text: T,
    pub(crate) code: String,
    raw: String,
}

impl<T: Display> Entry<T> {
    pub(crate) fn to_str(&self) -> String {
        format!("第{}行：'{}'", self.line_num, self.raw)
    }
}

pub(crate) trait TextParser: Sized {
    fn parse(num: usize, trimmed_text: &str) -> DynResult<Self>;
}

impl<T: TextParser> Entry<T> {
    pub(crate) fn new(line_num: usize, line: &str) -> DynResult<Option<Self>> {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            return Ok(None);
        }
        let mut parts = l.splitn(4, '\t');
        Ok(Some(Self {
            line_num,
            text: match parts.next().unwrap().trim() {
                "" => return Err(format!("第{line_num}行词条缺失文本").into()),
                trimmed => T::parse(line_num, trimmed)?,
            },
            code: match parts.next().map(|s| s.trim()) {
                Some("") | None => return Err(format!("第{line_num}行词条缺失编码").into()),
                Some(s) => s.into(),
            },
            raw: l.into(),
        }))
    }
}

pub(crate) type Single = Entry<char>;

impl TextParser for char {
    fn parse(line_num: usize, trimmed_text: &str) -> DynResult<Self> {
        let mut chars = trimmed_text.chars();
        let c = chars.next().unwrap(); // 一定非空
        if chars.next().is_some() {
            return Err(format!("第{line_num}行词条的文本不是单字").into());
        }
        Ok(c)
    }
}

pub(crate) type Phrase = Entry<Rc<str>>;

impl TextParser for Rc<str> {
    fn parse(line_num: usize, trimmed_text: &str) -> DynResult<Self> {
        if trimmed_text.chars().count() < 2 {
            return Err(format!("第{line_num}行词条的文本不是词组").into());
        }
        Ok(trimmed_text.into())
    }
}
