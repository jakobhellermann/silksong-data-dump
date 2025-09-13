#![allow(non_snake_case, dead_code)]

use num_enum::TryFromPrimitive;
use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum RecordTypes {
    Enemy,
    Other,
}

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum RequiredTypes {
    NotRequired,
    Required,
    RequiredSteelSoul,
}
#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum NailElement {
    None,
    Fire,
    Poison,
}
#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum SpecialDamageType {
    None,
    Frost,
    Lightning,
}

#[derive(TryFromPrimitive, Debug, Deserialize_repr)]
#[repr(i32)]
pub enum TestType {
    Bool,
    Int,
    Float,
    Enum,
    String,
}
#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum TestNumType {
    Equal,
    NotEqual,
    LessThan,
    MoreThan,
}
#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ToolItemType {
    Red,
    Blue,
    Yellow,
    Skill,
}
#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ReplenishUsage {
    Percentage,
    OneForOne,
    Custom,
}

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ToolDamageFlags {
    None = 0,
    Shredding = 1,
    Spearing = 2,
    Searing = 4,
}

#[derive(TryFromPrimitive, Debug, Serialize, Deserialize_repr)]
#[repr(i32)]
pub enum ReplenishResources {
    None = -1,
    Money,
    Shard,
}
