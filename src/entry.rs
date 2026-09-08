use std::fmt;

pub(crate) trait EntryText: Sized {
    const KIND: &str;
    fn parse(s: &str) -> Option<Self>;
}

/// 码表中的词条
pub(crate) struct Entry<T: EntryText> {
    /// 从 1 开始的行号
    line_num: usize,
    /// 修剪过的原始行
    line: String,
    /// 文本
    pub(crate) text: T,
    /// 编码
    pub(crate) code: String,
}

/// 单字词条
pub(crate) type Single = Entry<char>;

/// 词组词条
pub(crate) type Phrase = Entry<String>;

impl EntryText for char {
    const KIND: &str = "单字";
    fn parse(s: &str) -> Option<Self> {
        s.parse().ok()
    }
}

impl EntryText for String {
    const KIND: &str = "词组";
    fn parse(s: &str) -> Option<Self> {
        s.chars().nth(1).is_some().then(|| s.into())
    }
}

impl<T: EntryText> Entry<T> {
    /// 将修剪过的码表行解析为词条
    pub(crate) fn parse(line_num: usize, line: &str) -> crate::DynRes<Option<Self>> {
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let mut parts = line.splitn(3, '\t');

        let raw_text = parts
            .next()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| "缺失文本".to_string())?;
        let parsed_text = T::parse(raw_text).ok_or_else(|| format!("文本不是{}", T::KIND))?;

        let code = parts
            .next()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| "缺失编码".to_string())?;

        Ok(Some(Self {
            line_num,
            line: line.into(),
            text: parsed_text,
            code: code.into(),
        }))
    }
}

impl<T: EntryText> fmt::Display for Entry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "第{}行：'{}'", self.line_num, self.line)
    }
}
