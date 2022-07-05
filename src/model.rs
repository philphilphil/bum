use chrono::serde::ts_seconds;
use chrono::DateTime;
use chrono::{serde::ts_seconds::serialize as to_ts, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Debug, Deserialize)]
pub enum EntryType {
    #[default]
    Income,
    Expense,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BookEntry {
    pub name: String,
    pub kind: EntryType,
    pub category_id: u8,
    pub amount: f32,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

impl BookEntry {
    pub fn new(name: &str, kind: EntryType, category_id: u8, amount: f32) -> Self {
        Self {
            name: name.to_string(),
            kind,
            category_id,
            amount,
            date: chrono::offset::Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecurringEntry {
    pub name: String,
    pub kind: EntryType,
    pub category_id: u8,
    pub amount: f32,
    #[serde(serialize_with = "to_ts")]
    pub next_payment_date: DateTime<Utc>,
    #[serde(serialize_with = "to_ts")]
    pub cancelation_period: DateTime<Utc>,
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
