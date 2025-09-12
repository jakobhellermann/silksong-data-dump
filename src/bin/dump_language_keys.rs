use std::path::Path;

use anyhow::{Context, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use indexmap::IndexMap;
use rabex_env::rabex::objects::ClassId;
use rabex_env::unity::types::{ResourceManager, TextAsset};
use serde::Deserialize;

fn main() -> Result<()> {
    let env = silksong_data_dump::detect_game()?.context("Couldn't find silksong game files")?;

    let out = Path::new("out/languages");

    let ggm = env.load_cached("globalgamemanagers")?;
    let resource_manager = ggm.find_object_of::<ResourceManager>()?.unwrap();

    let _ = std::fs::remove_dir_all(&out);
    for (name, resource) in resource_manager.m_Container {
        let Some(name) = name.strip_prefix("languages/") else {
            continue;
        };
        let data = ggm.deref(resource.typed::<TextAsset>())?;
        if data.class_id() != ClassId::TextAsset {
            continue;
        }

        let key = b"";
        let (lang, sheet) = name.split_once('_').unwrap();
        let data = data.read()?;
        let data = BASE64_STANDARD.decode(data.m_Script)?;
        let data = decrypt(key, &data)?;

        let language_keys = quick_xml::de::from_str::<LanguageAsset>(&data)?;
        let language_keys = language_keys
            .entries
            .into_iter()
            .map(|entry| (entry.name, entry.value))
            .collect::<IndexMap<_, _>>();

        let dir = out.join(lang);
        std::fs::create_dir_all(&dir)?;
        std::fs::write(
            dir.join(sheet).with_extension("json"),
            serde_json::to_string_pretty(&language_keys)?,
        )?;
    }

    Ok(())
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
