use std::{env::args, error::Error, process::exit};

mod cli;
mod entry;

pub(crate) type DynRes<T> = Result<T, Box<dyn Error>>;

fn main() {
    if let Err(err) = run() {
        eprintln!("错误：{err}");
        let mut src = err.source();
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

    let args = cli::Args::parse(args().skip(1))?;

    todo!("调度")
}
