#![allow(non_snake_case, dead_code)]

use std::fmt::Display;

use rabex_env::rabex::objects::{PPtr, TypedPPtr};
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

use crate::lang::LocalizedString;

#[derive(Debug, Serialize, Deserialize)]
pub struct IntReference {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum PurchaseType {
    Purchase,
    Craft,
    Repair,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyJournalRecord {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub displayName: LocalizedString,
    pub killsRequired: i32,
    // isAlwaysUnlocked: u8, always 0
    // isRequiredForCompletion: u8, always 0
    pub recordType: RecordTypes,
    pub requiredType: RequiredTypes,
    // completeOthers
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum RecordTypes {
    Enemy,
    Other,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum RequiredTypes {
    NotRequired,
    Required,
    RequiredSteelSoul,
}

#[derive(Debug, Deserialize)]
pub struct DamageTag {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub damageAmount: i32,
    pub damageCooldownTimer: TypedPPtr<TimerGroup>,
    pub deathBurstEffects: Vec<serde_json::Value>,
    pub delayPerHit: f32,
    pub isToolDamage: u8,
    pub nailElement: NailElement,
    pub specialDamageType: SpecialDamageType,
    pub startDelay: f32,
    pub totalHitLimit: i32,
}
#[derive(Debug, Deserialize)]
pub struct TimerGroup {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub delay: f32,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum NailElement {
    None,
    Fire,
    Poison,
}
#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum SpecialDamageType {
    None,
    Frost,
    Lightning,
}

#[derive(Debug, Deserialize)]
pub struct ShopItem {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub displayName: LocalizedString,
    pub costReference: TypedPPtr<IntReference>,
    pub cost: i32,
    pub requiredItem: TypedPPtr<CollectibleItem>,
    pub requiredItemAmount: i32,
    pub extraAppearConditions: PlayerDataTest,
    pub purchaseType: PurchaseType,
    pub questsAppearConditions: Vec<QuestTest>,
    // subItems: Vec<()>,
    // spawnOnPurchaseConditionals
    pub setExtraPlayerDataBools: Vec<String>,
    pub setExtraPlayerDataInts: Vec<()>,
}

type CollectibleItem = Named;

#[derive(Debug, Deserialize)]
pub struct PlayerDataTest {
    pub TestGroups: Vec<TestGroup>,
}
impl Display for PlayerDataTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.TestGroups.len() {
            0 => Ok(()),
            1 => Display::fmt(&self.TestGroups[0], f),
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
pub struct TestGroup {
    pub Tests: Vec<Test>,
}
impl Display for TestGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.Tests.len() {
            0 => Ok(()),
            1 => Display::fmt(&self.Tests[0], f),
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
pub enum TestType {
    Bool,
    Int,
    Float,
    Enum,
    String,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub BoolValue: u8,
    pub FieldName: String,
    pub FloatValue: f32,
    pub IntValue: i32,
    pub NumType: TestNumType,
    pub StringType: i32,
    pub StringValue: String,
    pub Type: TestType,
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
pub enum TestNumType {
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
}

#[derive(Debug, Deserialize)]
pub struct QuestTest {
    pub CheckAccepted: u8,
    pub CheckAvailable: u8,
    pub CheckCompletable: u8,
    pub CheckCompleted: u8,
    pub CheckCompletedAmount: u8,
    pub CheckWasEverCompleted: u8,
    pub CompletedAmount: i32,
    pub IsAccepted: u8,
    pub IsAvailable: u8,
    pub IsCompletable: u8,
    pub IsCompleted: u8,
    pub Quest: TypedPPtr<Quest>,
    pub WasEverCompleted: u8,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectableItemRelicType {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub typeName: LocalizedString,
    pub typeDescription: LocalizedString,
    pub rewardAmount: i32,
}

#[derive(Debug, Deserialize)]
pub struct Quest {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub giveNameOverride: LocalizedString,
    pub invItemAppendDesc: LocalizedString,
    pub canTurnInAtBoard: u8,
    pub getTargetCondition: PlayerDataTest,
    pub persistentBoolTests: Vec<serde_json::Value>,
    pub playerDataTest: PlayerDataTest,
    pub questType: PPtr,
    // require stuff
    pub rewardCount: i32,
    pub rewardCountAct3: i32,
    pub rewardItem: TypedPPtr<Named>,
    // targetCount: i32, always 0
    // targets: Vec<QuestTarget>,
    pub targets: Vec<serde_json::Value>,
}
#[derive(Debug, Deserialize)]
pub struct QuestTarget {
    pub Count: i32,
    pub AltTest: PlayerDataTest,
    pub ItemName: LocalizedString,
}

#[derive(Debug, Deserialize)]
pub struct ToolItem {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub damageFlags: ToolDamageFlags,
    pub poisonDamageTicks: i32,
    pub countKey: TypedPPtr<SavedItem>,
    // zapDamageTicks: i32, always 0
    pub replenishResource: ReplenishResources,
    // replenishUsage: i32, always 0
    // replenishUsageMultiplier: f32, always 1.0
    pub usageOptions: ToolUsageOptions,
}
#[derive(Debug, Deserialize)]
pub struct ToolUsageOptions {
    pub SilkRequired: i32,
    // ThrowCooldown: f32, always 0.4
    pub UseAltForQuickSling: u8,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ToolDamageFlags {
    None = 0,
    Shredding = 1,
    Spearing = 2,
    Searing = 4,
}

#[derive(Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ReplenishResources {
    None = -1,
    Money,
    Shard,
}

#[derive(Debug, Deserialize)]
pub struct CollectableRelic {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub description: LocalizedString,
    pub eventConditionItem: TypedPPtr<SavedItem>,
}
#[derive(Debug, Deserialize)]
pub struct SavedItem {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
    pub displayName: LocalizedString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Named {
    #[serde(rename(deserialize = "m_Name"))]
    pub name: String,
}

impl From<Named> for String {
    fn from(value: Named) -> Self {
        value.name
    }
}
