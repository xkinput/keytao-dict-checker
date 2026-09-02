use std::path::PathBuf;

/// 飞键（Alternate）
pub(crate) const A: u8 = 1 << 0;
/// 错码（Incorrect）
pub(crate) const I: u8 = 1 << 1;
/// 冗余（Redundant）
pub(crate) const R: u8 = 1 << 2;
/// 空码（Vacant）
pub(crate) const V: u8 = 1 << 3;

/// 已检验的控制台参数
pub(crate) struct Args {
    /// 词库路径
    pub(crate) phrase: PathBuf,
    /// 单字码表路径
    pub(crate) single: Option<PathBuf>,
    /// 检查项
    pub(crate) checks: u8,
}

impl Args {
    /// 解析并检验控制台参数；典型入参 `std::env::args().skip(1)`
    pub(crate) fn parse(args: impl Iterator<Item = String>) -> crate::DynRes<Self> {
        let mut phrase = None;
        let mut single = None;
        let mut checks = 0;

        for arg in args {
            if let Some(flag) = arg.strip_prefix('-') {
                match flag {
                    "a" => checks |= A,
                    "i" => checks |= I,
                    "r" => checks |= R,
                    "v" => checks |= V,
                    _ => return Err(format!("检查项'-{flag}'无效").into()),
                }
            } else if phrase.is_none() {
                phrase = Some(PathBuf::from(&arg));
            } else if single.is_none() {
                single = Some(PathBuf::from(&arg));
            } else {
                return Err(format!("参数'{arg}'无效").into());
            }
        }

        let Some(phrase) = phrase else {
            return Err("词库路径缺失".into());
        };
        if checks == 0 {
            checks = A | I | R | V;
        }
        if (checks & I) != 0 && single.is_none() {
            return Err("检查项'-i'依赖单字码表路径，但后者缺失".into());
        }

        Ok(Self {
            phrase,
            single,
            checks,
        })
    }
}
