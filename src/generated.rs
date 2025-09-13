#![allow(dead_code, unused_imports, non_snake_case, nonstandard_style)]
use rabex_env::rabex::objects::{PPtr, TypedPPtr};
use rabex_env::unity::types::*;

#[derive(Debug, serde::Deserialize)]
pub struct CollectableItemRelicType {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub useResponses: Vec<UseResponse>,
    pub useResponseTextOverride: LocalisedString,
    pub preventUseChaining: u8,
    pub alwaysPlayInstantUse: u8,
    pub customInventoryDisplay: TypedPPtr<CustomInventoryItemCollectableDisplay>,
    pub extraDescriptionSection: PPtr, /* GameObject */
    pub resetIsSeen: u8,
    pub isVisibleWithBareInventory: u8,
    pub isHidden: u8,
    pub hideInShopCounters: u8,
    pub useQuestForCap: TypedPPtr<Quest>,
    pub customMaxAmount: i32,
    pub storyEvent: i32,
    pub typeName: LocalisedString,
    pub typeDescription: LocalisedString,
    pub appendDescription: LocalisedString,
    pub relicPlayType: i32,
    pub rewardAmount: i32,
    pub relics: Vec<TypedPPtr<CollectableRelic>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UseResponse {
    pub UseType: i32,
    pub Amount: i32,
    pub AmountRange: MinMaxInt,
    pub Description: LocalisedString,
}

#[derive(Debug, serde::Deserialize)]
pub struct LocalisedString {
    pub Sheet: String,
    pub Key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CustomInventoryItemCollectableDisplay {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub jitterMagnitudeMultiplier: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Quest {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: LocalisedString,
    pub location: LocalisedString,
    pub targetCount: i32,
    pub targetCounter: TypedPPtr<QuestTargetCounter>,
    pub targets: Vec<QuestTarget>,
    pub consumeTargetIfApplicable: u8,
    pub getTargetCondition: PlayerDataTest,
    pub canTurnInAtBoard: u8,
    pub giveNameOverride: LocalisedString,
    pub invItemAppendDesc: LocalisedString,
    // pub customPickupDisplay: UIMsgDisplay,
    pub rewardItem: TypedPPtr<SavedItem>,
    pub rewardCount: i32,
    pub rewardCountAct3: i32,
    pub awardAchievementOnComplete: String,
    pub inventoryDescription: LocalisedString,
    pub descAppendItemList: u8,
    pub descAppendBehaviour: i32,
    pub descAppendFormat: i32,
    pub inventoryCompletableDescription: LocalisedString,
    pub inventoryCompletedDescription: LocalisedString,
    pub descCounterType: i32,
    pub listCounterType: i32,
    pub hideMax: u8,
    pub hideCountersWhenCompletable: u8,
    pub overrideParagraphSpacing: OverrideFloat,
    pub overrideParagraphSpacingShort: OverrideFloat,
    pub hideDescCounterForLangs: Vec<i32>,
    pub wallDescription: LocalisedString,
    pub playerDataTest: PlayerDataTest,
    pub persistentBoolTests: Vec<PersistentBoolTest>,
    pub requiredCompleteQuests: Vec<TypedPPtr<FullQuestBase>>,
    pub requiredUnlockedTools: Vec<TypedPPtr<ToolItem>>,
    pub requiredCompleteTotalGroups: Vec<TypedPPtr<QuestCompleteTotalGroup>>,
    pub previousQuestStep: TypedPPtr<FullQuestBase>,
    pub markCompleted: Vec<TypedPPtr<FullQuestBase>>,
    pub cancelIfIncomplete: Vec<TypedPPtr<FullQuestBase>>,
    pub hideIfComplete: Vec<TypedPPtr<FullQuestBase>>,
    pub questType: TypedPPtr<QuestType>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CollectableRelic {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub description: LocalisedString,
    pub mixerOverride: PPtr, /* AudioMixerGroup */
    pub playEventRegister: String,
    pub eventConditionItem: TypedPPtr<SavedItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MinMaxInt {
    pub Start: i32,
    pub End: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestTargetCounter {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestTarget {
    pub Counter: TypedPPtr<QuestTargetCounter>,
    pub Count: i32,
    pub AltTest: PlayerDataTest,
    pub ItemName: LocalisedString,
    pub HideInCount: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerDataTest {
    pub TestGroups: Vec<TestGroup>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SavedItem {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: Option<LocalisedString>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OverrideFloat {
    pub IsEnabled: u8,
    pub Value: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct PersistentBoolTest {
    pub ID: String,
    pub SceneName: String,
    pub ExpectedValue: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct FullQuestBase {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: LocalisedString,
    pub location: LocalisedString,
    pub targetCount: i32,
    pub targetCounter: TypedPPtr<QuestTargetCounter>,
    pub targets: Vec<QuestTarget>,
    pub consumeTargetIfApplicable: u8,
    pub getTargetCondition: PlayerDataTest,
    pub canTurnInAtBoard: u8,
    pub giveNameOverride: LocalisedString,
    pub invItemAppendDesc: LocalisedString,
    // pub customPickupDisplay: UIMsgDisplay,
    pub rewardItem: TypedPPtr<SavedItem>,
    pub rewardCount: i32,
    pub rewardCountAct3: i32,
    pub awardAchievementOnComplete: String,
    pub inventoryDescription: LocalisedString,
    pub descAppendItemList: u8,
    pub descAppendBehaviour: i32,
    pub descAppendFormat: i32,
    pub inventoryCompletableDescription: LocalisedString,
    pub inventoryCompletedDescription: LocalisedString,
    pub descCounterType: i32,
    pub listCounterType: i32,
    pub hideMax: u8,
    pub hideCountersWhenCompletable: u8,
    pub overrideParagraphSpacing: OverrideFloat,
    pub overrideParagraphSpacingShort: OverrideFloat,
    pub hideDescCounterForLangs: Vec<i32>,
    pub wallDescription: LocalisedString,
    pub playerDataTest: PlayerDataTest,
    pub persistentBoolTests: Vec<PersistentBoolTest>,
    pub requiredCompleteQuests: Vec<TypedPPtr<FullQuestBase>>,
    pub requiredUnlockedTools: Vec<TypedPPtr<ToolItem>>,
    pub requiredCompleteTotalGroups: Vec<TypedPPtr<QuestCompleteTotalGroup>>,
    pub previousQuestStep: TypedPPtr<FullQuestBase>,
    pub markCompleted: Vec<TypedPPtr<FullQuestBase>>,
    pub cancelIfIncomplete: Vec<TypedPPtr<FullQuestBase>>,
    pub hideIfComplete: Vec<TypedPPtr<FullQuestBase>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ToolItem {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub isCounted: u8,
    pub countKey: TypedPPtr<SavedItem>,
    pub getReplaces: TypedPPtr<ToolItem>,
    pub r#type: i32,
    pub alternateUnlockedTest: PlayerDataTest,
    pub preventTutorialMsg: u8,
    pub baseStorageAmount: i32,
    pub unlockStartAmount: i32,
    pub preventStorageIncrease: u8,
    pub replenishResource: i32,
    pub replenishUsage: i32,
    pub replenishUsageMultiplier: f32,
    pub isCustomUsage: u8,
    pub togglePromptText: LocalisedString,
    pub damageFlags: i32,
    pub poisonDamageTicks: i32,
    pub poisonHueShift: f32,
    pub zapDamageTicks: i32,
    pub hasCustomAction: u8,
    pub customButtonCombo: Display,
    pub showPromptHold: u8,
    pub refillMsg: LocalisedString,
    pub extraDescriptionSection: PPtr, /* GameObject */
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestCompleteTotalGroup {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub quests: Vec<CompleteQuest>,
    pub target: f32,
    pub additionalTest: PlayerDataTest,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestType {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: LocalisedString,
    pub isDonateType: u8,
    pub removeQuestFromListOnComplete: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestGroup {
    pub Tests: Vec<Test>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Display {
    pub ActionButton: i32,
    pub DirectionModifier: i32,
    pub PromptText: LocalisedString,
    pub ShowHold: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct CompleteQuest {
    pub Quest: TypedPPtr<FullQuestBase>,
    pub Value: f32,
    pub IsRequired: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct Test {
    pub Type: i32,
    pub FieldName: String,
    pub BoolValue: u8,
    pub NumType: i32,
    pub IntValue: i32,
    pub FloatValue: f32,
    pub StringValue: String,
    pub StringType: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnemyJournalRecord {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: LocalisedString,
    pub description: LocalisedString,
    pub notes: LocalisedString,
    pub killsRequired: i32,
    pub isAlwaysUnlocked: u8,
    pub recordType: i32,
    pub isRequiredForCompletion: u8,
    pub requiredType: i32,
    pub completeOthers: Vec<TypedPPtr<EnemyJournalRecord>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct IntReference {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub value: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct DamageTag {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub damageAmount: i32,
    pub specialDamageType: i32,
    pub nailElement: i32,
    pub isToolDamage: u8,
    pub startDelay: f32,
    pub delayPerHit: f32,
    pub totalHitLimit: i32,
    pub damageCooldownTimer: TypedPPtr<TimerGroup>,
    pub doFlash: u8,
    pub flashConfig: FlashConfig,
}

#[derive(Debug, serde::Deserialize)]
pub struct TimerGroup {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub delay: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct FlashConfig {
    pub Amount: f32,
    pub TimeUp: f32,
    pub StayTime: f32,
    pub TimeDown: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ShopItem {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub displayName: LocalisedString,
    pub description: LocalisedString,
    pub descriptionMultiple: LocalisedString,
    pub purchaseType: i32,
    pub typeFlags: i32,
    pub currencyType: i32,
    pub costReference: TypedPPtr<CostReference>,
    pub cost: i32,
    pub requiredItem: TypedPPtr<CollectableItem>,
    pub requiredItemAmount: i32,
    pub requiredTools: i32,
    pub requiredToolsAmount: i32,
    pub requiredToolsDescription: LocalisedStringPlural,
    pub upgradeFromItem: TypedPPtr<CollectableItem>,
    pub extraAppearConditions: PlayerDataTest,
    pub questsAppearConditions: Vec<QuestTest>,
    pub playerDataBoolName: String,
    pub savedItem: TypedPPtr<SavedItem>,
    pub playerDataIntName: String,
    pub subItems: Vec<SubItem>,
    pub subItemSelectPrompt: LocalisedString,
    pub onPurchase: UnityEvent,
    pub spawnOnPurchaseConditionals: Vec<ConditionalSpawn>,
    pub setExtraPlayerDataBools: Vec<String>,
    pub setExtraPlayerDataInts: Vec<PlayerDataIntOperation>,
    pub eventAfterPurchased: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CostReference {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub value: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct CollectableItem {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub useResponses: Vec<UseResponse>,
    pub useResponseTextOverride: LocalisedString,
    pub preventUseChaining: u8,
    pub alwaysPlayInstantUse: u8,
    pub customInventoryDisplay: TypedPPtr<CustomInventoryItemCollectableDisplay>,
    pub extraDescriptionSection: PPtr, /* GameObject */
    pub resetIsSeen: u8,
    pub isVisibleWithBareInventory: u8,
    pub isHidden: u8,
    pub hideInShopCounters: u8,
    pub useQuestForCap: TypedPPtr<Quest>,
    pub customMaxAmount: i32,
    pub storyEvent: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct LocalisedStringPlural {
    pub Plural: LocalisedString,
    pub Single: LocalisedString,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuestTest {
    pub Quest: TypedPPtr<FullQuestBase>,
    pub CheckAvailable: u8,
    pub IsAvailable: u8,
    pub CheckAccepted: u8,
    pub IsAccepted: u8,
    pub CheckCompletedAmount: u8,
    pub CompletedAmount: i32,
    pub CheckCompletable: u8,
    pub IsCompletable: u8,
    pub CheckCompleted: u8,
    pub IsCompleted: u8,
    pub CheckWasEverCompleted: u8,
    pub WasEverCompleted: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct SubItem {
    pub Value: i32,
    pub Condition: PlayerDataTest,
}

#[derive(Debug, serde::Deserialize)]
pub struct UnityEvent {
    pub m_PersistentCalls: PersistentCallGroup,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConditionalSpawn {
    pub Condition: PlayerDataTest,
    pub GameObjectsToSpawn: Vec<PPtr /* GameObject */>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PlayerDataIntOperation {
    pub variableName: String,
    pub operation: i32,
    pub number: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct PersistentCallGroup {
    pub m_Calls: Vec<PersistentCall>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PersistentCall {
    pub m_Target: PPtr, /* Object */
    pub m_TargetAssemblyTypeName: String,
    pub m_MethodName: String,
    pub m_Mode: i32,
    pub m_Arguments: ArgumentCache,
    pub m_CallState: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ArgumentCache {
    pub m_ObjectArgument: PPtr, /* Object */
    pub m_ObjectArgumentAssemblyTypeName: String,
    pub m_IntArgument: i32,
    pub m_FloatArgument: f32,
    pub m_StringArgument: String,
    pub m_BoolArgument: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct ToolItemBasic {
    pub m_GameObject: TypedPPtr<GameObject>,
    pub m_Enabled: u8,
    pub m_Script: TypedPPtr<MonoScript>,
    pub m_Name: String,
    pub isCounted: u8,
    pub countKey: TypedPPtr<SavedItem>,
    pub getReplaces: TypedPPtr<ToolItem>,
    pub r#type: i32,
    pub alternateUnlockedTest: PlayerDataTest,
    pub preventTutorialMsg: u8,
    pub baseStorageAmount: i32,
    pub unlockStartAmount: i32,
    pub preventStorageIncrease: u8,
    pub replenishResource: i32,
    pub replenishUsage: i32,
    pub replenishUsageMultiplier: f32,
    pub isCustomUsage: u8,
    pub togglePromptText: LocalisedString,
    pub damageFlags: i32,
    pub poisonDamageTicks: i32,
    pub poisonHueShift: f32,
    pub zapDamageTicks: i32,
    pub hasCustomAction: u8,
    pub customButtonCombo: Display,
    pub showPromptHold: u8,
    pub refillMsg: LocalisedString,
    pub extraDescriptionSection: PPtr, /* GameObject */
    pub displayName: LocalisedString,
    pub description: LocalisedString,
    pub popupNameOverride: LocalisedString,
    pub usageOptions: UsageOptions,
}

#[derive(Debug, serde::Deserialize)]
pub struct UsageOptions {
    pub UseAltForQuickSling: u8,
    pub ThrowCooldown: f32,
    pub ThrowAnim: i32,
    pub ThrowVelocity: Vector2,
    pub ThrowVelocityAlt: Vector2,
    pub ThrowOffset: Vector2,
    pub ThrowOffsetAlt: Vector2,
    pub ScaleToHero: u8,
    pub FlipScale: u8,
    pub SetDamageDirection: u8,
    pub FsmEventName: String,
    pub IsNonBlockingEvent: u8,
    pub SilkRequired: i32,
    pub MaxActive: i32,
    pub MaxActiveAlt: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
