use crate::{dict::DictLoader, phrase_dict::PhraseDict, single_dict::SingleDict};
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize)]
struct RawConfig {
    phrase_dict: String,
    single_dict: Option<String>,
    vacant_codes: Option<bool>,
    err_encodings: Option<bool>,
}

pub(crate) struct Config {
    pub(crate) phrase_dict: PhraseDict,
    pub(crate) single_dict: Option<SingleDict>,
    pub(crate) vacant_codes: bool,
    pub(crate) err_encodings: bool,
    pub(crate) report_stem: PathBuf,
}

impl Config {
    pub(crate) fn load(toml: &str) -> crate::DynResult<Self> {
        let raw: RawConfig = toml::from_str(toml)?;
        let single_dict = raw
            .single_dict
            .map(|path| SingleDict::load(&path))
            .transpose()?;
        let err_encodings = raw.err_encodings.unwrap_or(false);
        if err_encodings && single_dict.is_none() {
            return Err("单字码表缺失".into());
        }
        Ok(Self {
            phrase_dict: PhraseDict::load(&raw.phrase_dict)?,
            single_dict,
            vacant_codes: raw.vacant_codes.unwrap_or(false),
            err_encodings,
            report_stem: {
                let path = Path::new(&raw.phrase_dict);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                path.parent()
                    .unwrap_or(Path::new("."))
                    .join(format!("{stem}-report"))
            },
        })
    }
}
