use anyhow::{Context, Result};
use rabex_env::handle::SerializedFileHandle;

fn main() -> Result<()> {
    let env = silksong_data_dump::detect_game()?.context("Couldn't find silksong game files")?;

    let hero = env.load_addressables_bundle_content("heroloading_assets_all.bundle")?;
    let hero = SerializedFileHandle::new(&env, &hero.0, &hero.1);

    for fsm in hero.scripts::<serde_json::Value>(&"PlayMakerFSM")? {
        let fsm = fsm.read()?;
        // let go = hero.deref(fsm.m_GameObject)?;
        // let path = go.path()?;
        // println!("{}", path);
        dbg!(fsm);
        break;
    }

    Ok(())
}
