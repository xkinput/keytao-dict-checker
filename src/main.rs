use std::{env, error::Error, fs, io, io::Write};

mod config;
mod dict;
mod entry;
mod phrase_dict;
mod single_dict;
mod trie;

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;

fn main() -> DynResult<()> {
    let mut args = env::args();
    let exe_name = args.next().unwrap();
    let toml = match args.next() {
        Some(s) if args.next().is_none() => fs::read_to_string(s)?,
        _ => return Err(format!("用法：'{exe_name} <config.toml路径>'").into()),
    };
    let config = config::Config::load(&toml)?;
    println!(
        "keytao-dict-checker 0.1.0 (20260606)\n\
        作者：Garth TB | 天卜 <g-art-h@outlook.com>\n\
        仓库：https://github.com/GarthTB/keytao-dict-checker",
    );
    let mut report = Vec::with_capacity(1024);
    if config.redundancies {
        find_redundancies(&config, &mut report)?;
    }
    if config.vacant_codes {
        find_vacant_codes(&config, &mut report)?;
    }
    if config.err_encodings {
        find_err_encodings(&config, &mut report)?;
    }
    if !report.is_empty() {
        output_report(&config, report)?;
    }
    Ok(println!("全部完成，程序结束"))
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}

fn find_redundancies(config: &config::Config, report: &mut Vec<u8>) -> DynResult<()> {
    print("检查冗余...")?;
    let mut redundancies = config.phrase_dict.redundancies();
    if !redundancies.is_empty() {
        writeln!(report, "------冗余------")?;
        redundancies.sort_unstable_by_key(|phrase| phrase.line_num);
        for phrase in redundancies {
            writeln!(report, "{}", phrase.to_str())?;
        }
    }
    Ok(println!("完成"))
}

fn find_vacant_codes(config: &config::Config, report: &mut Vec<u8>) -> DynResult<()> {
    print("检查空码...")?;
    let mut vacant_codes = config.phrase_dict.vacant_codes();
    if !vacant_codes.is_empty() {
        writeln!(report, "------空码------")?;
        vacant_codes.sort_unstable();
        for code in vacant_codes {
            writeln!(report, "{code}")?;
        }
    }
    Ok(println!("完成"))
}

fn find_err_encodings(config: &config::Config, report: &mut Vec<u8>) -> DynResult<()> {
    print("检查错码...")?;
    let single = config.single_dict.as_ref().unwrap();
    let i = report.len();
    writeln!(report, "------错码------")?;
    let j = report.len();
    for phrase in config.phrase_dict.err_encodings(single) {
        writeln!(report, "{}", phrase.to_str())?;
    }
    if report.len() == j {
        report.truncate(i);
    }
    Ok(println!("完成"))
}

fn output_report(config: &config::Config, report: Vec<u8>) -> DynResult<()> {
    print("输出报告...")?;
    let stem = &config.report_stem.to_string_lossy();
    let mut path = format!("{stem}.txt");
    let mut n = 2usize;
    while fs::exists(&path)? {
        path = format!("{stem}-{n}.txt");
        n += 1;
    }
    let mut file = fs::File::create(&path)?;
    file.write_all(&report)?;
    Ok(println!("完成"))
}
