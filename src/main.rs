use std::{env::args, error::Error, io, io::Write, process::exit};

mod cli;
mod entry;
mod inputs;
mod keytao;
mod output;
mod reader;
mod redundant;
mod vacant;

pub(crate) type DynRes<T> = Result<T, Box<dyn Error>>;

fn main() {
    println!("「RIME 键道」词库检查器 v{}", env!("CARGO_PKG_VERSION"));
    println!("作者：Garth TB | 天卜 <g-art-h@outlook.com>");
    println!("仓库：{}", env!("CARGO_PKG_REPOSITORY"));

    if let Err(e) = run() {
        eprintln!("错误：{e}");
        let mut src = e.source();
        while let Some(s) = src {
            eprintln!("    > {s}");
            src = s.source();
        }
        exit(1);
    }
}

fn run() -> DynRes<()> {
    print("解析参数...")?;
    let args = cli::Args::parse(args().skip(1))?;
    println!("完成！");

    print("载入词库...")?;
    let (phrases, cnt) = inputs::load_phrase_dict(&args.phrase)?;
    println!("完成！共{cnt}个词条。");

    let mut singles = None;
    if let Some(path) = args.single {
        print("载入单字码表...")?;
        let (map, cnt) = inputs::load_single_dict(&path)?;
        println!("完成！共{cnt}个词条，{}个单字。", map.len());
        singles = Some(map);
    }

    let mut report = Vec::with_capacity(1024);

    // TODO: 检查飞键
    // TODO: 检查错码

    if (args.checks & cli::R) != 0 {
        print("检查冗余...")?;
        let r = redundant::find_redundant(&phrases);
        match r.len() {
            0 => println!("完成！没有冗余。"),
            cnt => {
                writeln!(report, "------冗余------")?;
                for e in r {
                    writeln!(report, "{e}")?;
                }
                println!("完成！共{cnt}条冗余。");
            }
        }
    }

    if (args.checks & cli::V) != 0 {
        print("检查空码...")?;
        let v = vacant::find_vacant_codes(&phrases);
        match v.len() {
            0 => println!("完成！没有空码。"),
            cnt => {
                let mut sorted: Vec<_> = v.into_iter().collect();
                sorted.sort_unstable();
                writeln!(report, "------空码------")?;
                for code in sorted {
                    writeln!(report, "{code}")?;
                }
                println!("完成！共{cnt}个空码。");
            }
        }
    }

    if report.is_empty() {
        println!("报告为空，未生成文件。");
    } else {
        print("输出报告...")?;
        let path = output::write_report(&args.phrase, &report)?;
        println!("完成！已写入：{}", path.display());
    }

    Ok(println!("程序结束。已退出。"))
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}
