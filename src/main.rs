use cli::*;
use inputs::*;
use output::*;
use std::{env::args, error::Error, io, io::Write, path::Path, process::ExitCode};

mod cli;
mod entry;
mod inputs;
mod keytao;
mod output;
mod reader;

mod checks {
    mod p_companion;
    mod p_consistency;
    mod p_redundancy;
    mod s_companion;
    mod s_omission;
    mod s_redundancy;
    mod vacancy;
}

const TITLE: &str = "「RIME 键道」（KeyTao）码表检查器";
const VER: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Garth TB | 天卜 <g-art-h@outlook.com>";
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

pub(crate) type DynRes<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    println!("{TITLE} v{VER}");
    println!("作者：{AUTHOR}");
    println!("仓库：{REPO}");
    println!("================");

    if let Err(e) = run() {
        eprintln!("错误：{e}");
        let mut src = e.source();
        while let Some(s) = src {
            eprintln!("    > {s}");
            src = s.source();
        }
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> DynRes {
    print("解析参数...")?;
    let args = Args::parse(args().skip(1))?;
    println!("完成！");

    match args {
        Args::SingleOnly(path) => check_single_only(&path)?,
        Args::PhraseOnly(path) => check_phrase_only(&path)?,
        Args::Both {
            single: s_path,
            phrase: p_path,
        } => check_both(&s_path, &p_path)?,
    }

    Ok(println!("程序结束，已退出！"))
}

fn check_single_only(path: &Path) -> DynRes {
    print("载入单字码表...")?;
    let singles: SingleDict = load_dict(path)?;
    println!("完成！共 {} 个词条", singles.len());

    let mut report = Vec::with_capacity(1024);

    // TODO

    write_report_and_show(&report, path, "单字")
}

fn check_phrase_only(path: &Path) -> DynRes {
    print("载入词组码表...")?;
    let phrases: PhraseDict = load_dict(path)?;
    println!("完成！共 {} 个词条", phrases.len());

    let mut report = Vec::with_capacity(1024);

    // TODO

    write_report_and_show(&report, path, "词组")
}

fn check_both(s_path: &Path, p_path: &Path) -> DynRes {
    print("载入单字码表...")?;
    let singles: SingleDict = load_dict(s_path)?;
    println!("完成！共 {} 个词条", singles.len());

    print("载入词组码表...")?;
    let phrases: PhraseDict = load_dict(p_path)?;
    println!("完成！共 {} 个词条", phrases.len());

    let mut s_report = Vec::with_capacity(1024);
    let mut p_report = Vec::with_capacity(1024);

    // TODO

    write_report_and_show(&s_report, s_path, "单字")?;
    write_report_and_show(&p_report, p_path, "词组")
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}

fn write_report_and_show(report: &[u8], path: &Path, kind: &str) -> DynRes {
    if report.is_empty() {
        Ok(println!("{kind}报告为空，码表没问题！"))
    } else {
        print(&format!("输出{kind}报告..."))?;
        let file_name = write_report(path, &report)?;
        Ok(println!("已写入：{}", file_name.display()))
    }
}
