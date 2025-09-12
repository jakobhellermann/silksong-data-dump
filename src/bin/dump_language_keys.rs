use std::path::Path;

use anyhow::{Context, Result};
use silksong_data_dump::lang;

fn main() -> Result<()> {
    let env = silksong_data_dump::detect_game()?.context("Couldn't find silksong game files")?;

    let out = Path::new("out/languages");
    let _ = std::fs::remove_dir_all(out);

    let lang = lang::get_language_keys(&env)?;
    for (lang, sheets) in lang {
        for (sheet, keys) in sheets {
            let dir = out.join(&lang);
            std::fs::create_dir_all(&dir)?;
            std::fs::write(
                dir.join(sheet).with_extension("json"),
                serde_json::to_string_pretty(&keys)?,
            )?;
        }
    }

    Ok(())
}
