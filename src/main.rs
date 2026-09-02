use std::{env::args, error::Error, io, io::Write, path::Path, process::exit};

mod cli;
mod entry;
mod inputs;
mod reader;
mod vacant;

pub(crate) type DynRes<T> = Result<T, Box<dyn Error>>;

fn main() {
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
    println!("「RIME 键道」词库检查器 v{}", env!("CARGO_PKG_VERSION"));
    println!("作者：Garth TB | 天卜 <g-art-h@outlook.com>");
    println!("仓库：{}", env!("CARGO_PKG_REPOSITORY"));

    print("解析参数...")?;
    let args = cli::Args::parse(args().skip(1))?;
    println!("完成！");

    print("载入词库...")?;
    let (phrases, cnt) = inputs::load_phrase_dict(Path::new(&args.phrase))?;
    println!("完成！共{cnt}个词条。");

    let mut singles = None;
    if let Some(single) = args.single {
        print("载入单字码表...")?;
        let (map, cnt) = inputs::load_single_dict(Path::new(&single))?;
        println!("完成！共{cnt}个词条，{}个单字。", map.len());
        singles = Some(map);
    }

    todo!("调度")
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}
