use std::{env::args, error::Error, io, io::Write, process::exit};

mod cli;
mod entry;
mod inputs;
mod keytao;
mod output;
mod reader;

pub(crate) type DynRes<T = ()> = Result<T, Box<dyn Error>>;

fn main() {
    println!("「RIME 键道」词库检查器 v{}", env!("CARGO_PKG_VERSION"));
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

fn run() -> DynRes {
    print("解析参数...")?;
    let args = cli::Args::parse(args().skip(1))?;
    println!("完成！");

    print("载入词库...")?;
    let phrases = inputs::load_phrase_dict(&args.phrase)?;
    println!("完成！共{}个词条。", phrases.len());

    let mut singles = Default::default();
    if let Some(path) = args.single {
        print("载入单字码表...")?;
        let (map, cnt) = inputs::load_single_dict(&path)?;
        println!("完成！共{cnt}个词条，{}个单字。", map.len());
        singles = map;
    }

    let mut report = Vec::with_capacity(1024);

    if (args.checks & cli::I) != 0 {
        todo!("检查错码")
    }

    if (args.checks & cli::O) != 0 {
        todo!("检查遗漏")
    }

    if (args.checks & cli::R) != 0 {
        todo!("检查冗余")
    }

    if (args.checks & cli::V) != 0 {
        todo!("检查空码")
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
