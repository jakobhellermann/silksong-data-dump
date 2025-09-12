#![allow(non_snake_case, dead_code)]
use std::fmt::{Debug, Display, Write};
use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};
use rabex::objects::{PPtr, TypedPPtr};
use rabex_env::handle::{ScriptFilter, ScriptFilterContains, SerializedFileHandle};
use rabex_env::{Environment, rabex};
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use silksong_data_dump::lang::{self, LocalizedString};

fn main() -> Result<()> {
    let env = silksong_data_dump::detect_game()?.context("Couldn't find silksong game files")?;

    let out = Path::new("out");
    std::fs::create_dir_all(out)?;

    let languages = lang::get_language_keys(&env)?;
    let lang = &languages["en"];

    let data_assets = Path::new("dataassets_assets_assets/dataassets");

    /*let (x, x_data) = env.load_addressables_bundle_content(
        data_assets.join("collectables/collectableitems.bundle"),
    )?;
    let x = SerializedFileHandle::new(&env, &x, &x_data);
    for mb in x.objects_of::<MonoBehaviour>()? {
        let script = mb.mono_script().context("1")?.unwrap();
        dbg!(script.full_name());
    }*/

    dump_csv::<IntReference>(&env, out, &data_assets, "costs", &"CostReference")?;
    dump_csv::<IntReference>(&env, out, &data_assets, "damages", &"DamageReference")?;
    dump_csv_with::<CollectableItemRelicType, _>(
        &env,
        out,
        &data_assets,
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
        &data_assets,
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
        &data_assets,
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
        &data_assets,
        "questsystem/quests",
        &"Quest",
        |file, item| {
            let name = item
                .invItemAppendDesc
                .try_get(lang)
                .map(ToOwned::to_owned)
                .unwrap_or(item.name);
            let reward = file
                .deref_optional(item.rewardItem)?
                .map(|x| x.read())
                .transpose()?
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
                            .deref_optional(counter)
                            .unwrap()
                            .map(|x| x.read())
                            .transpose()
                            .unwrap()
                            .map(|x| x.name)
                            .unwrap_or_default();
                        let x = QuestTarget::deserialize(x).unwrap();
                        if x.AltTest.TestGroups.is_empty() {
                            format!("{} {} ", x.Count, item)
                        } else {
                            format!("{} ", x.AltTest.to_string())
                        }
                    })
                    .collect(),
            })
        },
    )?;
    dump_csv_with::<DamageTag, _>(
        &env,
        out,
        &data_assets,
        "damagetags",
        &"DamageTag",
        |file, item| {
            let damage_cooldown = file
                .deref_optional(item.damageCooldownTimer)?
                .map(|x| x.read())
                .transpose()?;
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
        &data_assets,
        "shopitems",
        &"ShopItem",
        |file, item| {
            let cost_ref = file
                .deref_optional(item.costReference)?
                .map(|x| x.read())
                .transpose()?;
            let required_item = file
                .deref_optional(item.requiredItem)?
                .map(|x| x.read())
                .transpose()?;
            let conditions = if item.extraAppearConditions.TestGroups.len() > 0 {
                Some(item.extraAppearConditions.to_string())
            } else {
                None
            };

            let quest_requirement = if item.questsAppearConditions.len() > 0 {
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
    let file = SerializedFileHandle::new(&env, &file.0, &file.1);
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
struct IntReference {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    value: i32,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum PurchaseType {
    Purchase,
    Craft,
    Repair,
}

#[derive(Debug, Serialize, Deserialize)]
struct EnemyJournalRecord {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    displayName: LocalizedString,
    killsRequired: i32,
    // isAlwaysUnlocked: u8, always 0
    // isRequiredForCompletion: u8, always 0
    recordType: RecordTypes,
    requiredType: RequiredTypes,
    // completeOthers
}

#[derive(Debug, Serialize, Deserialize)]
struct EnemyJournalRecordData {
    name: String,
    killsRequired: i32,
    recordType: RecordTypes,
    requiredType: RequiredTypes,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum RecordTypes {
    Enemy,
    Other,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum RequiredTypes {
    NotRequired,
    Required,
    RequiredSteelSoul,
}

#[derive(Debug, Deserialize)]
struct DamageTag {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    damageAmount: i32,
    damageCooldownTimer: TypedPPtr<TimerGroup>,
    deathBurstEffects: Vec<serde_json::Value>,
    delayPerHit: f32,
    isToolDamage: u8,
    nailElement: NailElement,
    specialDamageType: SpecialDamageType,
    startDelay: f32,
    totalHitLimit: i32,
}
#[derive(Debug, Deserialize)]
struct TimerGroup {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    delay: f32,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum NailElement {
    None,
    Fire,
    Poison,
}
#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum SpecialDamageType {
    None,
    Frost,
    Lightning,
}

#[derive(Debug, Deserialize)]
struct ShopItem {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    displayName: LocalizedString,
    costReference: TypedPPtr<IntReference>,
    cost: i32,
    requiredItem: TypedPPtr<CollectibleItem>,
    requiredItemAmount: i32,
    extraAppearConditions: PlayerDataTest,
    purchaseType: PurchaseType,
    questsAppearConditions: Vec<QuestTest>,
    // subItems: Vec<()>,
    // spawnOnPurchaseConditionals
    setExtraPlayerDataBools: Vec<String>,
    setExtraPlayerDataInts: Vec<()>,
}

type CollectibleItem = Named;

#[derive(Debug, Deserialize)]
struct PlayerDataTest {
    TestGroups: Vec<TestGroup>,
}
impl Display for PlayerDataTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.TestGroups.len() {
            0 => Ok(()),
            1 => return Display::fmt(&self.TestGroups[0], f),
            _ => {
                let mut first = true;
                for group in &self.TestGroups {
                    if !first {
                        write!(f, " OR ")?;
                    }
                    write!(f, "{}", group)?;
                    first = false;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct TestGroup {
    Tests: Vec<Test>,
}
impl Display for TestGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.Tests.len() {
            0 => Ok(()),
            1 => return Display::fmt(&self.Tests[0], f),
            _ => {
                write!(f, "(")?;
                let mut first = true;
                for test in &self.Tests {
                    if !first {
                        write!(f, " AND ")?;
                    }
                    write!(f, "{}", test)?;
                    first = false;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
enum TestType {
    Bool,
    Int,
    Float,
    Enum,
    String,
}

#[derive(Debug, Deserialize)]
struct Test {
    BoolValue: u8,
    FieldName: String,
    FloatValue: f32,
    IntValue: i32,
    NumType: TestNumType,
    StringType: i32,
    StringValue: String,
    Type: TestType,
}
impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match &self.NumType {
            TestNumType::Equal => "=",
            TestNumType::NotEqual => "!=",
            TestNumType::LessThan => "<",
            TestNumType::MoreThan => ">",
        };
        match self.Type {
            TestType::Bool => write!(f, "{} = {}", self.FieldName, self.BoolValue != 0),
            TestType::Int => write!(f, "{} {op} {}", self.FieldName, self.IntValue),
            TestType::Float => todo!(),
            TestType::Enum => write!(f, "{} {op} {}", self.FieldName, self.IntValue),
            TestType::String => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum TestNumType {
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
}

#[derive(Debug, Deserialize)]
struct QuestTest {
    CheckAccepted: u8,
    CheckAvailable: u8,
    CheckCompletable: u8,
    CheckCompleted: u8,
    CheckCompletedAmount: u8,
    CheckWasEverCompleted: u8,
    CompletedAmount: i32,
    IsAccepted: u8,
    IsAvailable: u8,
    IsCompletable: u8,
    IsCompleted: u8,
    Quest: TypedPPtr<Quest>,
    WasEverCompleted: u8,
}
#[derive(Debug, Serialize, Deserialize)]
struct CollectableItemRelicType {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    typeName: LocalizedString,
    typeDescription: LocalizedString,
    rewardAmount: i32,
}

#[derive(Debug, Serialize)]
struct CollectableItemData {
    name: String,
    displayName: String,
    rewardAmount: i32,
}

#[derive(Debug, Deserialize)]
struct Quest {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    giveNameOverride: LocalizedString,
    invItemAppendDesc: LocalizedString,
    canTurnInAtBoard: u8,
    getTargetCondition: PlayerDataTest,
    persistentBoolTests: Vec<serde_json::Value>,
    playerDataTest: PlayerDataTest,
    questType: PPtr,
    // require stuff
    rewardCount: i32,
    rewardCountAct3: i32,
    rewardItem: TypedPPtr<Named>,
    // targetCount: i32, always 0
    // targets: Vec<QuestTarget>,
    targets: Vec<serde_json::Value>,
}
#[derive(Debug, Deserialize)]
struct QuestTarget {
    Count: i32,
    AltTest: PlayerDataTest,
    ItemName: LocalizedString,
}

#[derive(Debug, Deserialize)]
struct ToolItem {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    damageFlags: ToolDamageFlags,
    poisonDamageTicks: i32,
    countKey: TypedPPtr<SavedItem>,
    // zapDamageTicks: i32, always 0
    replenishResource: ReplenishResources,
    // replenishUsage: i32, always 0
    // replenishUsageMultiplier: f32, always 1.0
    usageOptions: ToolUsageOptions,
}
#[derive(Debug, Deserialize)]
struct ToolUsageOptions {
    SilkRequired: i32,
    // ThrowCooldown: f32, always 0.4
    UseAltForQuickSling: u8,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum ToolDamageFlags {
    None = 0,
    Shredding = 1,
    Spearing = 2,
    Searing = 4,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
enum ReplenishResources {
    None = -1,
    Money,
    Shard,
}

#[derive(Debug, Deserialize)]
struct CollectableRelic {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    description: LocalizedString,
    eventConditionItem: TypedPPtr<SavedItem>,
}
#[derive(Debug, Deserialize)]
struct SavedItem {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
    displayName: LocalizedString,
}

// csv types

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

#[derive(Debug, Serialize, Deserialize)]
struct Named {
    #[serde(rename(deserialize = "m_Name"))]
    name: String,
}

impl From<Named> for String {
    fn from(value: Named) -> Self {
        value.name
    }
}
