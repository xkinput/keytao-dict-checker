use std::path::PathBuf;

pub(crate) enum Args {
    SingleOnly(PathBuf),
    PhraseOnly(PathBuf),
    Both { phrase: PathBuf, single: PathBuf },
}

impl Args {
    /// 解析输入参数；典型入参 `std::env::args().skip(1)`
    pub(crate) fn parse(mut args: impl Iterator<Item = String>) -> crate::DynRes<Args> {
        let (Some(a), Some(b), None) = (args.next(), args.next(), args.next()) else {
            return Err("参数无效。用法参见 README。".into());
        };

        Ok(match a.as_str() {
            "-s" => Args::SingleOnly(b.into()),
            "-p" => Args::PhraseOnly(b.into()),
            _ => Args::Both {
                phrase: a.into(),
                single: b.into(),
            },
        })
    }
}
