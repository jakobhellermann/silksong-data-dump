#![allow(non_snake_case)]
use std::fmt::{Debug, Write};
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};
use rabex::objects::TypedPPtr;
use rabex_env::handle::{ScriptFilter, ScriptFilterContains, SerializedFileHandle};
use rabex_env::{Environment, rabex};
use serde::{Deserialize, Serialize};

use silksong_data_dump::lang;
use silksong_data_dump::types::*;

fn main() -> Result<()> {
    let env = silksong_data_dump::detect_game()?.context("Couldn't find silksong game files")?;

    let out = Path::new("out");
    std::fs::create_dir_all(out)?;

    let languages = lang::get_language_keys(&env)?;
    let lang = &languages["en"];

    let data_assets = Path::new("dataassets_assets_assets/dataassets");

    dump_csv::<IntReference>(&env, out, data_assets, "costs", &"CostReference")?;
    dump_csv::<IntReference>(&env, out, data_assets, "damages", &"DamageReference")?;
    dump_csv_with::<CollectableItemRelicType, _>(
        &env,
        out,
        data_assets,
        "collectables/collectableitems",
        &ScriptFilterContains("CollectableItemRelicType"),
        |_, item| {
            Ok(CollectableItemData {
                name: item.name,
                displayName: item.typeName.get(lang).to_owned(),
                rewardAmount: item.rewardAmount,
            })
        },
    )?;
    dump_csv_with::<EnemyJournalRecord, _>(
        &env,
        out,
        data_assets,
        "enemyjournal/journalrecords",
        &"EnemyJournalRecord",
        |_, item| {
            Ok(EnemyJournalRecordData {
                name: item.displayName.get(lang).to_owned(),
                killsRequired: item.killsRequired,
                recordType: item.recordType,
                requiredType: item.requiredType,
            })
        },
    )?;
    dump_csv_with::<ToolItem, _>(
        &env,
        out,
        data_assets,
        "tools/toolitems",
        &ScriptFilterContains("Tool"),
        |file, item| {
            let display_name = file
                .deref_read_optional(item.countKey)?
                .map(|x| x.displayName.get(lang).to_owned())
                .filter(|name| *name != "Ruined Tool");
            Ok(ToolItemData {
                name: display_name.unwrap_or(item.name),
                damageFlags: item.damageFlags,
                poisonDamageTicks: item.poisonDamageTicks,
                replenishResource: item.replenishResource,
                SilkRequired: item.usageOptions.SilkRequired,
                UseAltForQuickSling: item.usageOptions.UseAltForQuickSling,
            })
        },
    )?;
    dump_csv_with::<Quest, _>(
        &env,
        out,
        data_assets,
        "questsystem/quests",
        &"Quest",
        |file, item| {
            let name = item
                .invItemAppendDesc
                .try_get(lang)
                .map(ToOwned::to_owned)
                .unwrap_or(item.name);
            let reward = file
                .deref_read_optional(item.rewardItem)?
                .map(|x| x.name)
                .unwrap_or_default();

            Ok(QuestData {
                name,
                // getTargetCondition: item.getTargetCondition.to_string(),
                condition: item.playerDataTest.to_string(),
                rewardCount: item.rewardCount,
                rewardItem: reward,
                rewardCountAct3: item.rewardCountAct3,
                // targetCount: item.targetCount,
                requirements: item
                    .targets
                    .iter()
                    // .map(|x| format!("{} {},", x.AltTest.to_string(), x.Count))
                    .map(|x| {
                        let counter = TypedPPtr::<Named>::deserialize(&x["Counter"]).unwrap();
                        let item = file
                            .deref_read_optional(counter)
                            .unwrap()
                            .map(|x| x.name)
                            .unwrap_or_default();
                        let x = QuestTarget::deserialize(x).unwrap();
                        if x.AltTest.TestGroups.is_empty() {
                            format!("{} {} ", x.Count, item)
                        } else {
                            format!("{} ", x.AltTest)
                        }
                    })
                    .collect(),
            })
        },
    )?;
    dump_csv_with::<DamageTag, _>(
        &env,
        out,
        data_assets,
        "damagetags",
        &"DamageTag",
        |file, item| {
            let damage_cooldown = file.deref_read_optional(item.damageCooldownTimer)?;
            Ok(DamageTagData {
                name: item.name,
                damageAmount: item.damageAmount,
                damageCooldownTimer: damage_cooldown.map(|cooldown| format!("{}s", cooldown.delay)),
                delayPerHit: item.delayPerHit,
                isToolDamage: item.isToolDamage,
                nailElement: item.nailElement,
                specialDamageType: item.specialDamageType,
                startDelay: item.startDelay,
                totalHitLimit: item.totalHitLimit,
            })
        },
    )?;
    dump_csv_with::<ShopItem, _>(
        &env,
        out,
        data_assets,
        "shopitems",
        &"ShopItem",
        |file, item| {
            let cost_ref = file.deref_read_optional(item.costReference)?;
            let required_item = file.deref_read_optional(item.requiredItem)?;
            let conditions = if !item.extraAppearConditions.TestGroups.is_empty() {
                Some(item.extraAppearConditions.to_string())
            } else {
                None
            };

            let quest_requirement = if !item.questsAppearConditions.is_empty() {
                let mut quests_str = String::new();
                for quest_test in &item.questsAppearConditions {
                    let quest = file.deref(quest_test.Quest)?.read()?;
                    write!(&mut quests_str, "'{}'", quest.name)?;
                    let m = |b: u8| match b != 0 {
                        true => "",
                        false => "not ",
                    };
                    if quest_test.CheckAvailable != 0 {
                        write!(&mut quests_str, " {}available", m(quest_test.IsAvailable),)?;
                    }
                    if quest_test.CheckAccepted != 0 {
                        write!(&mut quests_str, " {}accepted", m(quest_test.IsAccepted),)?;
                    }
                    if quest_test.CheckCompletedAmount != 0 {
                        write!(
                            &mut quests_str,
                            " completedamount {}",
                            quest_test.CompletedAmount
                        )?;
                    }
                    if quest_test.CheckCompletable != 0 {
                        write!(
                            &mut quests_str,
                            " {}completable",
                            m(quest_test.IsCompletable),
                        )?;
                    }
                    if quest_test.CheckCompleted != 0 {
                        write!(&mut quests_str, " {}completed", m(quest_test.IsCompleted),)?;
                    }
                    if quest_test.CheckWasEverCompleted != 0 {
                        write!(
                            &mut quests_str,
                            " {}completed",
                            m(quest_test.WasEverCompleted),
                        )?;
                    }
                }
                Some(quests_str)
            } else {
                None
            };

            let display_name = item.displayName.get(lang).to_owned();
            let internal_name = item.name;

            Ok(ShopItemData {
                name: display_name,
                internalName: internal_name,
                cost: cost_ref.map(|cost| cost.value).unwrap_or(item.cost),
                item: required_item.map(From::from),
                conditions,
                quest: quest_requirement,
            })
        },
    )?;

    Ok(())
}

fn dump_csv<T: Debug + for<'de> Deserialize<'de> + Serialize>(
    env: &Environment,
    out: &Path,
    data_assets: &Path,
    name: &str,
    script: &dyn ScriptFilter,
) -> Result<()> {
    dump_csv_with::<T, T>(env, out, data_assets, name, script, |_, val| Ok(val))
}

fn dump_csv_with<T, U>(
    env: &Environment,
    out: &Path,
    data_assets: &Path,
    name: &str,
    script: &dyn ScriptFilter,
    mut f: impl FnMut(SerializedFileHandle, T) -> Result<U>,
) -> Result<()>
where
    T: for<'de> Deserialize<'de>,
    U: Debug + Serialize,
{
    let name = Path::new(name);
    let path = data_assets.join(name).with_extension("bundle");
    let file = env.load_addressables_bundle_content(path)?;
    let file = SerializedFileHandle::new(env, &file.0, &file.1);
    let mut writer = csv::Writer::from_writer(File::create(
        out.join(name.file_name().unwrap()).with_extension("csv"),
    )?);
    for value in file.scripts::<T>(script)? {
        let value = f(file.reborrow(), value.read()?)?;
        // println!("{:?}", value);

        writer.serialize(value)?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct EnemyJournalRecordData {
    name: String,
    killsRequired: i32,
    recordType: RecordTypes,
    requiredType: RequiredTypes,
}

#[derive(Debug, Serialize)]
struct CollectableItemData {
    name: String,
    displayName: String,
    rewardAmount: i32,
}
#[derive(Debug, Serialize)]
struct ShopItemData {
    name: String,
    internalName: String,
    cost: i32,
    item: Option<String>,
    conditions: Option<String>,
    quest: Option<String>,
}
#[derive(Debug, Serialize)]
struct DamageTagData {
    name: String,
    damageAmount: i32,
    damageCooldownTimer: Option<String>,
    delayPerHit: f32,
    isToolDamage: u8,
    nailElement: NailElement,
    specialDamageType: SpecialDamageType,
    startDelay: f32,
    totalHitLimit: i32,
}

#[derive(Debug, Serialize)]
struct ToolItemData {
    name: String,
    damageFlags: ToolDamageFlags,
    poisonDamageTicks: i32,
    replenishResource: ReplenishResources,
    SilkRequired: i32,
    UseAltForQuickSling: u8,
}

#[derive(Debug, Serialize)]
struct QuestData {
    name: String,
    // getTargetCondition: String, empty
    // persistentBoolTests: Vec<serde_json::Value>,
    // questType: PPtr,
    // require stuff
    rewardItem: String,
    rewardCount: i32,
    rewardCountAct3: i32,
    // targetCount: i32,
    requirements: String,
    condition: String,
}
