mod checks;
mod cli;
mod entry;
mod inputs;
mod keytao;
mod output;
mod reader;

use checks::*;
use cli::*;
use entry::*;
use inputs::*;
use output::*;
use std::{env::args, error::Error, fmt::Display, io, io::Write, path::Path, process::ExitCode};

const TITLE: &str = "「RIME 键道」（KeyTao）码表检查器";
const VER: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "Garth TB | 天卜 <g-art-h@outlook.com>";
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

pub(crate) type DynRes<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> ExitCode {
    println!("{TITLE} v{VER}");
    println!("作者：{AUTHOR}");
    println!("仓库：{REPO}");
    println!("{}", "=".repeat(32));

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
        Args::SingleOnly(path) => run_single_only(&path)?,
        Args::PhraseOnly(path) => run_phrase_only(&path)?,
        Args::Both {
            single: s_path,
            phrase: p_path,
        } => run_both(&s_path, &p_path)?,
    }

    Ok(println!("程序结束，已退出！"))
}

fn run_single_only(path: &Path) -> DynRes {
    print("载入单字码表...")?;
    let singles: SingleDict = load_dict(path)?;
    println!("完成！共 {} 个词条。", singles.len());

    let mut report = Vec::with_capacity(1024);

    check_s_format(&singles, &mut report)?;
    check_s_xm_consistency(&singles, &mut report)?;
    check_s_fj_absence(&singles, &mut report)?;
    check_s_jm_absence(&singles, &mut report)?;
    check_s_jm_anomaly(&singles, &mut report)?;
    // TODO

    output_report(&report, path, "单字")
}

fn run_phrase_only(path: &Path) -> DynRes {
    print("载入词组码表...")?;
    let phrases: PhraseDict = load_dict(path)?;
    println!("完成！共 {} 个词条。", phrases.len());

    let mut report = Vec::with_capacity(1024);

    check_p_format(&phrases, &mut report)?;
    // TODO

    output_report(&report, path, "词组")
}

fn run_both(s_path: &Path, p_path: &Path) -> DynRes {
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
    check_s_xm_consistency(&singles, &mut s_report)?;
    check_s_fj_absence(&singles, &mut s_report)?;
    check_s_jm_absence(&singles, &mut s_report)?;
    check_s_jm_anomaly(&singles, &mut s_report)?;
    check_p_format(&phrases, &mut p_report)?;
    // TODO

    output_report(&s_report, s_path, "单字")?;
    output_report(&p_report, p_path, "词组")
}

fn check_s_format(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字编码形式异常... ")?;
    let res = s_format::check(singles);
    report_items(report, "单字编码形式异常", "条", &res)
}

fn check_s_xm_consistency(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字形码段不自洽... ")?;
    let res = s_xm_consistency::check(singles);
    report_items(report, "单字形码段不自洽", "条", &res)
}

fn check_s_fj_absence(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字飞键伴生缺失... ")?;
    let res = s_fj_absence::check(singles);
    report_items(report, "单字飞键伴生缺失", "条", &res)
}

fn check_s_jm_absence(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字简码缺失... ")?;
    let res = s_jm_absence::check(singles);
    report_items(report, "单字简码缺失", "条", &res)
}

fn check_s_jm_anomaly(singles: &[Single], report: &mut Vec<u8>) -> DynRes {
    print("检查单字简码不当... ")?;
    let res = s_jm_anomaly::check(singles);
    report_items(report, "单字简码不当", "条", &res)
}

fn check_p_format(phrases: &[Phrase], report: &mut Vec<u8>) -> DynRes {
    print("检查词组编码形式异常...")?;
    let res = p_format::check(phrases);
    report_items(report, "词组编码形式异常", "条", &res)
}

fn report_items<T: Display>(report: &mut Vec<u8>, title: &str, unit: &str, items: &[T]) -> DynRes {
    match items.len() {
        0 => Ok(println!("没有！")),
        n => {
            let eq = "=".repeat((32 - 2 * title.chars().count()) / 2);
            writeln!(report, "{eq}{title}{eq}")?;
            for e in items {
                writeln!(report, "{e}")?;
            }
            Ok(println!("共 {n} {unit}！"))
        }
    }
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

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}
