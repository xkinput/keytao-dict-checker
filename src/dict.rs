pub(crate) trait DictLoader: Sized {
    fn load(path: &str) -> crate::DynResult<Self> {
        let s = std::fs::read_to_string(path)?;
        let mut lines = s.lines().enumerate();
        verify_and_rem_header(&mut lines)?;
        Self::parse_lines(&mut lines)
    }

    fn parse_lines<'a>(
        lines: &mut impl Iterator<Item = (usize, &'a str)>,
    ) -> crate::DynResult<Self>;
}

#[derive(serde::Deserialize)]
struct Header {
    columns: Option<Vec<String>>,
}

const EXPECTED: [&str; 3] = ["text", "code", "weight"];

fn verify_and_rem_header<'a>(
    lines: &mut impl Iterator<Item = (usize, &'a str)>,
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

    let header: Header = yaml_serde::from_str(&yaml)?;
    match header.columns {
        Some(cols) if cols != EXPECTED => {
            Err(format!("词库列配置无效：应为{EXPECTED:?}，实为{cols:?}").into())
        }
        _ => Ok(()),
    }
}
