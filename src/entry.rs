use std::rc::Rc;

pub(crate) struct Entry {
    text: Rc<str>,
    code: String,
    weight: Option<String>,
}

impl Entry {
    pub(crate) fn text(&self) -> Rc<str> {
        self.text.clone()
    }

    pub(crate) fn code(&self) -> &str {
        &self.code
    }

    pub(crate) fn new(num: usize, line: &str) -> crate::DynResult<Self> {
        let mut parts = line.splitn(4, '\t');
        let text = match parts.next().unwrap().trim() {
            "" => return Err(format!("第{num}行词条缺失文本").into()),
            trimmed => trimmed.into(),
        };
        let code = parts
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("第{num}行词条缺失编码"))?
            .into();
        let weight = parts
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.into());
        if parts.next().is_some() {
            return Err(format!("第{num}行词条有多余内容").into());
        }

        Ok(Self { text, code, weight })
    }
}
