use std::{error, io, io::Write};

mod config;
mod dict;
mod entry;
mod single;
mod trie;

pub(crate) type DynResult<T> = Result<T, Box<dyn error::Error>>;

fn main() -> DynResult<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        let n = args.len() - 1;
        return Err(format!("命令行参数错误：应为1个'config.toml'路径，实为{n}个").into());
    }
    print(
        "keytao-dict-checker 0.1.0 (20260512)\n\
        作者：Garth TB | 天卜 <g-art-h@outlook.com>\n\
        仓库：https://github.com/GarthTB/cn-input-encode\n\
        加载配置...",
    )?;
    Ok(println!("全部完成，程序结束"))
}

fn print(s: &str) -> io::Result<()> {
    print!("{s}");
    io::stdout().flush()
}
