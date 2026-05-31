use crate::{phrase_dict::PhraseDict, single_dict::SingleDict};

#[derive(serde::Deserialize)]
struct RawConfig {
    phrase_dict: String,
    single_dict: Option<String>,
    vacant: Option<bool>,
    encoding: Option<bool>,
}

pub(crate) struct Config {
    pub(crate) phrase_dict: PhraseDict,
    pub(crate) single_dict: Option<SingleDict>,
    pub(crate) vacant: Option<bool>,
    pub(crate) encoding: Option<bool>,
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
            vacant: raw.vacant,
            encoding: raw.encoding,
        })
    }
}
