use std::fmt::Display;

use crate::enums::{TestNumType, TestType};
use crate::generated::*;
use crate::lang::Language;

impl LocalisedString {
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
impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self.NumType.try_into().unwrap() {
            TestNumType::Equal => "=",
            TestNumType::NotEqual => "!=",
            TestNumType::LessThan => "<",
            TestNumType::MoreThan => ">",
        };
        match self.Type.try_into().unwrap() {
            TestType::Bool => write!(f, "{} = {}", self.FieldName, self.BoolValue != 0),
            TestType::Int => write!(f, "{} {op} {}", self.FieldName, self.IntValue),
            TestType::Float => todo!(),
            TestType::Enum => write!(f, "{} {op} {}", self.FieldName, self.IntValue),
            TestType::String => todo!(),
        }
    }
}
