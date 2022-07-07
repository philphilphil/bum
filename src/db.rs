use anyhow::Result;
use std::{
    fs::{self, File},
    path::Path,
};

use crate::model::{BookEntry, Category, RecurringEntry};

// TODO: Add a default path and option to set a path to db files via cli arg
const DB_BASEPATH: &str = "db/";
const DB_FILE_CATEGORY: &str = "data_categories.json";
// TODO: Split into active and archive bookings
const DB_FILE_BOOKINGS: &str = "data_bookings.json";
const DB_FILE_RECURRING: &str = "data_recurring.json";

pub fn get_bookings() -> Result<Vec<BookEntry>> {
    let b: Vec<BookEntry> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS))?)?;
    Ok(b)
}

pub fn add_booking(booking: BookEntry) -> Result<()> {
    let book_path = Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS);
    let mut b: Vec<BookEntry> = serde_json::from_reader(&File::open(&book_path)?)?;
    b.push(booking);
    serde_json::to_writer_pretty(&File::create(&book_path)?, &b)?;
    Ok(())
}

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

pub(crate) fn get_recurring() -> Result<Vec<RecurringEntry>> {
    let r: Vec<RecurringEntry> =
        serde_json::from_reader(&File::open(Path::new(DB_BASEPATH).join(DB_FILE_RECURRING))?)?;
    Ok(r)
}

pub fn add_recurring(rec: RecurringEntry) -> Result<()> {
    let rec_path = Path::new(DB_BASEPATH).join(DB_FILE_RECURRING);
    let mut recurrings: Vec<RecurringEntry> = serde_json::from_reader(&File::open(&rec_path)?)?;
    recurrings.push(rec);
    serde_json::to_writer_pretty(&File::create(&rec_path)?, &recurrings)?;
    Ok(())
}

pub fn ensure_db_files_exist() -> Result<()> {
    let cat_path = Path::new(DB_BASEPATH).join(DB_FILE_CATEGORY);
    let book_path = Path::new(DB_BASEPATH).join(DB_FILE_BOOKINGS);
    let rec_path = Path::new(DB_BASEPATH).join(DB_FILE_RECURRING);
    if !cat_path.exists() {
        fs::write(cat_path, "[]")?;
    }
    if !book_path.exists() {
        fs::write(book_path, "[]")?;
    }
    if !rec_path.exists() {
        fs::write(rec_path, "[]")?;
    }
    Ok(())
}
