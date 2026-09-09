use std::{ffi::OsString, fs::File, io::ErrorKind, io::Write, path::Path};

/// 将报告写入输入文件同目录，并返回最终文件名
pub(crate) fn write_report(input: &Path, report: &[u8]) -> crate::DynRes<OsString> {
    let dir = input.parent().unwrap_or(Path::new(""));
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("无法从 {} 提取文件主干。", input.display()))?;

    let mut options = File::options();
    options.write(true).create_new(true);

    let mut name = stem.to_os_string();
    name.push("_report.txt");
    let mut n = 2;

    loop {
        let target = dir.join(&name);
        match options.open(&target) {
            Ok(mut file) => {
                file.write_all(report)
                    .map_err(|e| format!("写入检查报告 {} 失败：{e}", target.display()))?;
                return Ok(name);
            }
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                name = stem.to_os_string();
                name.push(format!("_report_{n}.txt"));
                n += 1;
            }
            Err(e) => {
                return Err(format!("创建检查报告 {} 失败：{e}", target.display()).into());
            }
        }
    }
}
