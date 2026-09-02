use crate::entry::{Entry, ParseText};
use saphyr::{LoadableYamlNode, Yaml};
use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};

/// 读取词库 `path` 并对其中的每个词条执行 `f`，返回词条总数
pub(crate) fn visit_dict<T, F>(path: &Path, mut f: F) -> crate::DynRes<usize>
where
    T: ParseText,
    F: FnMut(Entry<T>),
{
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines().enumerate();

    let mut header = String::with_capacity(256);
    for (_, line) in lines.by_ref() {
        let line = line?;
        if line.trim_end() == "..." {
            break;
        }
        header.push_str(&line);
        header.push('\n');
    }
    validate_header(&header)?;

    let mut cnt = 0;
    for (i, line) in lines {
        let line = line?;
        if let Some(entry) = Entry::<T>::parse(i + 1, &line)? {
            f(entry);
            cnt += 1;
        }
    }

    Ok(cnt)
}

fn validate_header(s: &str) -> crate::DynRes<()> {
    let docs = Yaml::load_from_str(s).map_err(|e| format!("YAML头解析失败: \n{e}"))?;
    let doc = docs.first().ok_or_else(|| "YAML头为空")?;

    if doc
        .as_mapping_get("name")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err("YAML头缺失'name'字段".into());
    }

    if doc
        .as_mapping_get("version")
        .and_then(|v| v.as_str())
        .is_none()
    {
        return Err("YAML头缺失'version'字段".into());
    }

    if let Some(cols) = doc.as_mapping_get("columns") {
        let seq = cols.as_vec().ok_or_else(|| "'columns'字段不是列表")?;
        let expected = ["text", "code", "weight"];
        if seq.len() != expected.len() {
            return Err("'columns'字段不是[text, code, weight]".into());
        }
        for (i, col) in seq.iter().enumerate() {
            let s = col
                .as_str()
                .ok_or_else(|| "'columns'列表的元素不是字符串")?;
            if s != expected[i] {
                return Err("'columns'字段不是[text, code, weight]".into());
            }
        }
    }

    Ok(())
}
