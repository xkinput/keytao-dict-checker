use crate::{DynRes, entry::Entry, entry::ParseText};
use saphyr::{LoadableYamlNode, Yaml};
use std::{fs::File, io::BufRead, io::BufReader, path::Path};

/// 读取词库 `path` 并对其中的每个词条执行 `f`，返回词条总数
pub(crate) fn visit_dict<T: ParseText>(path: &Path, mut f: impl FnMut(Entry<T>)) -> DynRes<usize> {
    let reader = BufReader::new(File::open(path)?);
    let mut lines = reader.lines().enumerate();

    let mut header = String::with_capacity(256);
    for (_, line) in lines.by_ref() {
        let line = line?;
        header += &line;
        header.push('\n');
        if line.trim_end() == "..." {
            break;
        }
    }
    validate_header(&header)?;

    let mut n = 0;
    for (i, line) in lines {
        let line = line?;
        if let Some(e) = Entry::parse(i + 1, line.trim_end())? {
            f(e);
            n += 1;
        }
    }
    if n == 0 {
        return Err("读不到任何词条".into());
    }

    Ok(n)
}

fn validate_header(s: &str) -> DynRes {
    let docs = Yaml::load_from_str(s).map_err(|e| format!("YAML头解析失败: \n{e}"))?;
    let doc = docs.first().ok_or("YAML头为空")?;

    doc.as_mapping_get("name")
        .and_then(Yaml::as_str)
        .ok_or("YAML头缺失'name'字段")?;
    doc.as_mapping_get("version")
        .and_then(Yaml::as_str)
        .ok_or("YAML头缺失'version'字段")?;

    if let Some(cols) = doc.as_mapping_get("columns") {
        let seq = cols.as_vec().ok_or("'columns'字段不是列表")?;
        let expected = ["text", "code", "weight"];
        if seq.len() != expected.len() {
            return Err("'columns'字段不是[text, code, weight]".into());
        }
        for (i, col) in seq.iter().enumerate() {
            if col.as_str().ok_or("'columns'列表的元素不是字符串")? != expected[i] {
                return Err("'columns'字段不是[text, code, weight]".into());
            }
        }
    }

    Ok(())
}
