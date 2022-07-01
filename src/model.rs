use chrono::DateTime;
use chrono::{serde::ts_seconds::serialize as to_ts, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum EntryType {
    #[default]
    Income,
    Expense,
}

#[derive(Serialize, Deserialize)]
pub struct BookEntry {
    pub name: String,
    pub kind: EntryType,
    pub category_id: u8,
    pub amount: f32,
    #[serde(serialize_with = "to_ts")]
    pub date: DateTime<Utc>,
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

// #[derive(Serialize, Deserialize)]
// pub struct RecurringIncome {
//     pub name: String,
//     pub amount: f32,
//     pub category_id: u8,
// }
