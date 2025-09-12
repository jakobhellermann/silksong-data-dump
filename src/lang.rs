use anyhow::Result;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use indexmap::IndexMap;
use rabex_env::Environment;
use rabex_env::rabex::objects::ClassId;
use rabex_env::unity::types::{ResourceManager, TextAsset};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "String")]
pub struct LocalizedString {
    pub Sheet: String,
    pub Key: String,
}
impl From<LocalizedString> for String {
    fn from(value: LocalizedString) -> Self {
        format!("{}-{}", value.Sheet, value.Key)
    }
}

impl LocalizedString {
    pub fn get<'a>(&self, lang: &'a Language) -> &'a str {
        assert!(!self.Sheet.is_empty());
        assert!(!self.Key.is_empty());
        &lang[&self.Sheet.to_ascii_lowercase()][&self.Key]
    }
    pub fn try_get<'a>(&self, lang: &'a Language) -> Option<&'a str> {
        if self.Sheet.is_empty() && self.Key.is_empty() {
            return None;
        }
        Some(&lang[&self.Sheet.to_ascii_lowercase()][&self.Key])
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LocalizedStringPlural {
    pub Plural: LocalizedString,
    pub Single: LocalizedString,
}

pub type Languages = IndexMap<String, Language>;
pub type Language = IndexMap<String, IndexMap<String, String>>;

pub fn get_language_keys(env: &Environment) -> Result<Languages> {
    let ggm = env.load_cached("globalgamemanagers")?;
    let resource_manager = ggm.find_object_of::<ResourceManager>()?.unwrap();

    let mut languages = Languages::new();
    for (name, resource) in resource_manager.m_Container {
        let Some(name) = name.strip_prefix("languages/") else {
            continue;
        };
        let data = ggm.deref(resource.typed::<TextAsset>())?;
        if data.class_id() != ClassId::TextAsset {
            continue;
        }

        let key = b"UKu52ePUBwetZ9wNX88o54dnfKRu0T1l";
        let (lang, sheet) = name.split_once('_').unwrap();
        let data = data.read()?;
        let data = BASE64_STANDARD.decode(data.m_Script)?;
        let data = decrypt(key, &data)?;

        let language_keys = quick_xml::de::from_str::<LanguageAsset>(&data)?;
        let language_keys = language_keys
            .entries
            .into_iter()
            .map(|entry| (entry.name, entry.value))
            .collect();

        languages
            .entry(lang.to_owned())
            .or_default()
            .insert(sheet.to_owned(), language_keys);
    }

    Ok(languages)
}

fn decrypt(key: &[u8], data: &[u8]) -> Result<String> {
    use aes::cipher::block_padding::Pkcs7;
    use aes::cipher::{BlockDecryptMut, KeyInit};

    let mut out = vec![0; data.len()];
    let pt = ecb::Decryptor::<aes::Aes256>::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(data, &mut out)?;
    let data = std::str::from_utf8(pt)?;

    Ok(data.to_owned())
}

#[derive(Debug, Deserialize)]
struct LanguageAsset {
    #[serde(rename = "entry")]
    pub entries: Vec<LanguageEntry>,
}

#[derive(Debug, Deserialize)]
struct LanguageEntry {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text", default)]
    pub value: String,
}
