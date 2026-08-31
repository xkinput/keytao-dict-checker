use crate::DynRes;
use std::fmt;

/// 词条文本解析契约
pub(crate) trait ParseText: Sized {
    /// 解析并校验词条文本
    fn parse(line_num: usize, text: &str) -> DynRes<Self>;
}

/// 词库中的词条
pub(crate) struct Entry<T: ParseText> {
    /// 从 1 开始的行号
    line_num: usize,
    /// 原始行（按 RIME 右修剪）
    raw: String,
    /// 文本
    pub(crate) text: T,
    /// 编码
    pub(crate) code: String,
}

/// 单字词条
pub(crate) type Single = Entry<char>;

impl ParseText for char {
    fn parse(line_num: usize, text: &str) -> DynRes<Self> {
        let mut chars = text.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(format!("第{line_num}行词条的文本不是单字").into()),
        }
    }
}

/// 词组词条
pub(crate) type Phrase = Entry<String>;

impl ParseText for String {
    fn parse(line_num: usize, text: &str) -> DynRes<Self> {
        if text.chars().nth(1).is_none() {
            return Err(format!("第{line_num}行词条的文本不是词组").into());
        }
        Ok(text.into())
    }
}

impl<T: ParseText> Entry<T> {
    /// 解析词条
    pub(crate) fn parse(line_num: usize, line: &str) -> DynRes<Option<Self>> {
        let l = line.trim_end();
        if l.is_empty() || l.starts_with('#') {
            return Ok(None);
        }

        let mut parts = l.splitn(3, '\t');
        let text = parts
            .next()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| format!("第{line_num}行词条缺失文本"))?;
        let code = parts
            .next()
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| format!("第{line_num}行词条缺失编码"))?;

        Ok(Some(Self {
            line_num,
            raw: l.into(),
            text: T::parse(line_num, text)?,
            code: code.into(),
        }))
    }
}

impl<T: ParseText> fmt::Display for Entry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "第{}行：'{}'", self.line_num, self.raw)
    }
}
