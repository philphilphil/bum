use anyhow::Result;
use std::{
    fs::{self, File},
    path::Path,
};

use crate::model::{BudgetBooking, Category, RecurringBooking, Setting};

// TODO: Add a default path and option to set a path to db files via cli arg
const DB_BASEPATH: &str = "db/";
const DB_FILE_CATEGORY: &str = "data_categories.json";
const DB_FILE_SETTINGS: &str = "settings.json";
// TODO: Split into active and archive bookings
const DB_FILE_BOOKINGS: &str = "data_bookings.json";
const DB_FILE_BOOKINGS_ARCHIVE: &str = "data_bookings_archive.json";
const DB_FILE_RECURRING: &str = "data_recurring.json";

pub fn get_expenses() -> Result<Vec<BudgetBooking>> {
    let b: Vec<BudgetBooking> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS))?)?;
    Ok(b)
}

pub fn get_expenses_archive() -> Result<Vec<BudgetBooking>> {
    let b: Vec<BudgetBooking> = serde_json::from_reader(&File::open(
        Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS_ARCHIVE),
    )?)?;
    Ok(b)
}

pub fn add_expense(booking: BudgetBooking) -> Result<()> {
    let book_path = Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS);
    let mut b: Vec<BudgetBooking> = serde_json::from_reader(&File::open(&book_path)?)?;
    b.push(booking);
    serde_json::to_writer_pretty(&File::create(&book_path)?, &b)?;
    Ok(())
}

pub(crate) fn get_settings() -> Result<Vec<Setting>> {
    let c: Vec<Setting> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_SETTINGS))?)?;
    Ok(c)
}

pub(crate) fn get_setting_currency_symbol() -> Result<String> {
    let s: Vec<Setting> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_SETTINGS))?)?;
    let symbol: String = s
        .iter()
        .filter(|s| s.key == "Currency_Symbol")
        .map(|s| s.value.clone())
        .collect::<String>();
    Ok(symbol)
}

// pub fn add_setting(setting: Setting) -> Result<()> {
//     let set_path = Path::new(DB_BASEPATH).join(DB_FILE_SETTINGS);
//     let mut settings: Vec<Setting> = serde_json::from_reader(&File::open(&set_path)?)?;
//     settings.push(setting);
//     serde_json::to_writer_pretty(&File::create(&set_path)?, &settings)?;
//     Ok(())
// }

pub(crate) fn get_categories() -> Result<Vec<Category>> {
    let c: Vec<Category> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_CATEGORY))?)?;
    Ok(c)
}

pub fn add_category(cat: Category) -> Result<()> {
    let cat_path = Path::new(DB_BASEPATH).join(DB_FILE_CATEGORY);
    let mut categories: Vec<Category> = serde_json::from_reader(&File::open(&cat_path)?)?;
    categories.push(cat);
    serde_json::to_writer_pretty(&File::create(&cat_path)?, &categories)?;
    Ok(())
}

pub(crate) fn get_recurring() -> Result<Vec<RecurringBooking>> {
    let r: Vec<RecurringBooking> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_RECURRING))?)?;
    Ok(r)
}

pub fn add_recurring(rec: RecurringBooking) -> Result<()> {
    let rec_path = Path::new(DB_BASEPATH).join(DB_FILE_RECURRING);
    let mut recurrings: Vec<RecurringBooking> = serde_json::from_reader(&File::open(&rec_path)?)?;
    recurrings.push(rec);
    serde_json::to_writer_pretty(&File::create(&rec_path)?, &recurrings)?;
    Ok(())
}

pub fn ensure_db_files_exist() -> Result<()> {
    let cat_path = Path::new(DB_BASEPATH).join(DB_FILE_CATEGORY);
    let book_path = Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS);
    let rec_path = Path::new(DB_BASEPATH).join(DB_FILE_RECURRING);
    let set_path = Path::new(DB_BASEPATH).join(DB_FILE_SETTINGS);
    if !cat_path.exists() {
        fs::write(cat_path, "[]")?;
    }
    if !book_path.exists() {
        fs::write(book_path, "[]")?;
    }
    if !rec_path.exists() {
        fs::write(rec_path, "[]")?;
    }
    if !set_path.exists() {
        fs::write(set_path, "[]")?;
    }
    Ok(())
}
