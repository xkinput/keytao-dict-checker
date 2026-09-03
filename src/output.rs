use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn write_report(i_path: &Path, report: &[u8]) -> crate::DynRes<PathBuf> {
    let stem = i_path.file_stem().ok_or("无法获取词库文件名")?;
    let make_o_path = |tail: &str| {
        let mut name = stem.to_os_string();
        name.push(tail);
        i_path.with_file_name(name)
    };

    let mut o_path = make_o_path("_report.txt");
    let mut n = 2u32;
    while fs::exists(&o_path)? {
        o_path = make_o_path(&format!("_report_{n}.txt"));
        n += 1;
    }

    fs::write(&o_path, report)?;
    Ok(o_path)
}
