use anyhow::Result;
use rabex_env::Environment;
use rabex_env::game_files::GameFiles;
use rabex_env::rabex::tpk::TpkTypeTreeBlob;
use rabex_env::rabex::typetree::typetree_cache::sync::TypeTreeCache;

pub fn detect_game() -> Result<Option<Environment>> {
    find_steam_game("Hollow Knight: Silksong")
}
pub fn find_steam_game(name: &str) -> Result<Option<Environment>> {
    let name_filter = name.to_lowercase();

    let steam = steamlocate::SteamDir::locate()?;
    for lib in steam.libraries()? {
        let lib = lib?;
        for app in lib.apps() {
            let app = app?;
            let path = lib.resolve_app_dir(&app);

            let Ok(game_files) = GameFiles::probe(path) else {
                continue;
            };

            if app
                .name
                .as_deref()
                .unwrap_or_default()
                .to_lowercase()
                .contains(&name_filter)
            {
                let tpk = TypeTreeCache::new(TpkTypeTreeBlob::embedded());
                let env = Environment::new(game_files, tpk);
                return Ok(Some(env));
            }
        }
    }
    Ok(None)
}
