use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Debug, Deserialize, PartialEq)]
pub enum EntryType {
    #[default]
    Income,
    Expense,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookEntry {
    pub name: String,
    pub kind: EntryType,
    pub category_token: String,
    pub amount: f32,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

impl BookEntry {
    pub fn new(name: &str, kind: EntryType, category_token: &str, amount: f32) -> Self {
        Self {
            name: name.to_string(),
            kind,
            category_token: category_token.to_string(),
            amount,
            date: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Default, Serialize, Debug, Deserialize, PartialEq)]
pub enum RecurringType {
    #[default]
    Monthly,
    Yearly,
}

#[derive(Serialize, Deserialize)]
pub struct RecurringEntry {
    pub name: String,
    pub kind: EntryType,
    pub category_token: String,
    pub amount: f32,
    pub rate_type: RecurringType,
    // #[serde(serialize_with = "to_ts")]
    // pub next_payment_date: DateTime<Utc>,
    // #[serde(serialize_with = "to_ts")]
    // pub cancelation_period: DateTime<Utc>,
}

impl RecurringEntry {
    pub fn new(
        name: &str,
        kind: EntryType,
        category_token: &str,
        amount: f32,
        rate_type: RecurringType,
    ) -> Self {
        Self {
            name: name.to_string(),
            kind,
            category_token: category_token.to_string(),
            amount,
            rate_type,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub token: String,
    pub name: String,
}

impl Category {
    pub fn new(token: &str, name: &str) -> Self {
        Category {
            token: token.to_string(),
            name: name.to_string(),
        }
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct RecurringIncome {
//     pub name: String,
//     pub amount: f32,
//     pub category_id: u8,
// }
