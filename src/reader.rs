use crate::{DynRes, entry::Entry, entry::EntryText};
use saphyr::{LoadableYamlNode, Yaml};
use std::{fs::File, io::BufRead, io::BufReader, path::Path};

pub(crate) fn for_each_entry<T: EntryText>(path: &Path, mut f: impl FnMut(Entry<T>)) -> DynRes {
    let mut reader = BufReader::new(File::open(path)?);
    let mut line = String::with_capacity(256);
    let mut line_num = 0;
    let mut header = String::with_capacity(256);

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        line_num += 1;
        let line = line.trim_end();
        header.push_str(line);
        header.push('\n');
        if line == "..." {
            break;
        }
    }

    validate_header(&header)?;

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        line_num += 1;
        match Entry::parse(line_num, line.trim_end()) {
            Ok(Some(entry)) => f(entry),
            Err(err) => return Err(format!("第 {line_num} 行：{err}").into()),
            _ => (),
        }
    }

    Ok(())
}

fn validate_header(s: &str) -> DynRes {
    let docs = Yaml::load_from_str(s).map_err(|e| format!("YAML 头解析失败：{e}"))?;
    let doc = docs.first().ok_or("YAML 头为空。")?;

    doc.as_mapping_get("name")
        .and_then(Yaml::as_str)
        .ok_or("YAML 头中的 name 缺失或不是字符串。")?;
    doc.as_mapping_get("version")
        .and_then(Yaml::as_str)
        .ok_or("YAML 头中的 version 缺失或不是字符串。")?;

    if let Some(cols) = doc.as_mapping_get("columns") {
        let seq = cols.as_vec().ok_or("YAML 头中的 columns 不是列表。")?;
        let expected = ["text", "code", "weight"];
        if seq.len() != expected.len()
            || seq
                .iter()
                .zip(expected)
                .any(|(col, name)| col.as_str() != Some(name))
        {
            return Err("YAML 头中的 columns 不是 [text, code, weight]".into());
        }
    }

    Ok(())
}
