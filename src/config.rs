use crate::{dict::DictLoader, phrase_dict::PhraseDict, single_dict::SingleDict};
use std::path::Path;

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
    pub(crate) report_stem: String,
}

impl Config {
    pub(crate) fn load(config: &str) -> crate::DynResult<Self> {
        let raw: RawConfig = toml::from_str(config)?;
        Ok(Self {
            phrase_dict: PhraseDict::load(&raw.phrase_dict)?,
            single_dict: raw
                .single_dict
                .map(|path| SingleDict::load(&path))
                .transpose()?,
            vacant_codes: raw.vacant_codes.unwrap_or(false),
            err_encodings: raw.err_encodings.unwrap_or(false),
            report_stem: {
                let path = Path::new(&raw.phrase_dict);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let dir = path.parent().unwrap_or(Path::new("."));
                dir.join(format!("{stem}-report")).to_string_lossy().into()
            },
        })
    }
}
