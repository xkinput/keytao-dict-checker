use std::{env, error::Error, fs, io, io::Write};

mod config;
mod dict;
mod entry;
mod phrase_dict;
mod single_dict;
mod trie;

pub(crate) type DynResult<T> = Result<T, Box<dyn Error>>;

fn main() -> DynResult<()> {
    println!(
        "keytao-dict-checker 0.1.0 (20260606)\n\
        作者：Garth TB | 天卜 <g-art-h@outlook.com>\n\
        仓库：https://github.com/GarthTB/keytao-dict-checker",
    );
    let config = match &env::args().skip(1).collect::<Vec<_>>()[..] {
        [arg] => config::Config::load(&fs::read_to_string(arg)?)?,
        [] => return Err("命令行参数错误：缺少配置路径".into()),
        _ => return Err("命令行参数错误：有多余参数".into()),
    };
    let mut report = Vec::with_capacity(1024);
    if config.vacant_codes {
        check_vacant_codes(&config, &mut report)?;
    }
    if config.err_encodings {
        check_err_encodings(&config, &mut report)?;
    }
    if report.len() > 0 {
        output_report(&config, report)?;
    }
    Ok(println!("全部完成，程序结束"))
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}

fn check_vacant_codes(config: &config::Config, report: &mut Vec<u8>) -> DynResult<()> {
    print("检查空码...")?;
    let vacant_codes = config.phrase_dict.vacant_codes();
    if vacant_codes.len() > 0 {
        writeln!(report, "------空码------")?;
        for code in vacant_codes {
            writeln!(report, "{code}")?;
        }
    }
    Ok(println!("完成"))
}

fn check_err_encodings(config: &config::Config, report: &mut Vec<u8>) -> DynResult<()> {
    print("检查错码...")?;
    let Some(single) = &config.single_dict else {
        return Err("单字码表缺失".into());
    };
    let mut buf = Vec::with_capacity(1024);
    for phrase in config.phrase_dict.err_encodings(single) {
        writeln!(buf, "{}", phrase.to_str())?;
    }
    if buf.len() > 0 {
        writeln!(report, "------错码------")?;
        report.extend(buf);
    }
    Ok(println!("完成"))
}

fn output_report(config: &config::Config, report: Vec<u8>) -> DynResult<()> {
    print("生成报告...")?;
    let stem = &config.report_stem;
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
