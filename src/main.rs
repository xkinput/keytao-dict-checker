use checks::*;
use cli::*;
use entry::*;
use inputs::*;
use output::*;
use std::{env::args, error::Error, io, io::Write, path::Path, process::ExitCode};

mod checks;
mod cli;
mod entry;
mod inputs;
mod keytao;
mod output;
mod reader;

const TITLE: &str = "「RIME 键道」（KeyTao）码表检查器";
const VER: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Garth TB | 天卜 <g-art-h@outlook.com>";
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

pub(crate) type DynRes<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    println!("{TITLE} v{VER}");
    println!("作者：{AUTHOR}");
    println!("仓库：{REPO}");
    println!("================================");

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
    println!("完成！共 {} 个词条。", singles.len());

    let mut report = Vec::with_capacity(1024);

    check_s_format(&singles, &mut report)?;

    // TODO

    output_report(&report, path, "单字")
}

fn check_phrase_only(path: &Path) -> DynRes {
    print("载入词组码表...")?;
    let phrases: PhraseDict = load_dict(path)?;
    println!("完成！共 {} 个词条。", phrases.len());

    let mut report = Vec::with_capacity(1024);

    check_p_vacancy(&phrases, &mut report)?;

    // TODO

    output_report(&report, path, "词组")
}

fn check_both(s_path: &Path, p_path: &Path) -> DynRes {
    print("载入单字码表...")?;
    let singles: SingleDict = load_dict(s_path)?;
    let stems = load_stems(s_path)?;
    println!(
        "完成！共 {} 个词条，{} 个单字。",
        singles.len(),
        stems.len()
    );

    print("载入词组码表...")?;
    let phrases: PhraseDict = load_dict(p_path)?;
    println!("完成！共 {} 个词条。", phrases.len());

    let mut s_report = Vec::with_capacity(1024);
    let mut p_report = Vec::with_capacity(1024);

    check_s_format(&singles, &mut s_report)?;
    check_p_vacancy(&phrases, &mut p_report)?;

    // TODO

    output_report(&s_report, s_path, "单字")?;
    output_report(&p_report, p_path, "词组")
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}

fn check_s_format(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字编码形式异常...")?;
    let v = s_format::check(&singles);
    Ok(match v.len() {
        0 => println!("没有！"),
        n => {
            writeln!(report, "========单字编码形式异常========")?;
            for e in v {
                writeln!(report, "{e}")?;
            }
            println!("共 {n} 条！");
        }
    })
}

fn check_p_vacancy(phrases: &[Phrase], report: &mut Vec<u8>) -> DynRes {
    print("检查词组空码...")?;
    let v = p_vacancy::check(&phrases);
    Ok(match v.len() {
        0 => println!("没有！"),
        n => {
            writeln!(report, "==============空码==============")?;
            for code in v {
                writeln!(report, "{code}")?;
            }
            println!("共 {n} 个！");
        }
    })
}

fn output_report(report: &[u8], path: &Path, kind: &str) -> DynRes {
    if report.is_empty() {
        Ok(println!("{kind}报告为空，码表没问题！"))
    } else {
        print(&format!("输出{kind}报告..."))?;
        let file_name = write_report(path, &report)?;
        Ok(println!("已写入：{}", file_name.display()))
    }
}
