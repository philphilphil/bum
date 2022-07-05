use serde_json::Result;
use std::{fs::File, path::Path};

use crate::model::{BookEntry, Category};

pub fn get_bookings() -> Vec<BookEntry> {
    let b: Vec<BookEntry> = serde_json::from_reader(&File::open("data.json").unwrap()).unwrap();
    b
}

pub fn add_booking(booking: BookEntry) -> Result<()> {
    let mut b: Vec<BookEntry> = serde_json::from_reader(&File::open("data.json").unwrap()).unwrap();
    b.push(booking);
    serde_json::to_writer_pretty(&File::create("data.json").unwrap(), &b)?;
    Ok(())
}

pub(crate) fn get_categories() -> Vec<Category> {
    let c: Vec<Category> =
        serde_json::from_reader(&File::open("data_categories.json").unwrap()).unwrap();
    c
}

pub fn add_category(cat: Category) -> Result<()> {
    let mut categories: Vec<Category> = Vec::new();

    if Path::new("data_categories.json").exists() {
        categories = serde_json::from_reader(&File::open("data_categories.json").unwrap()).unwrap();
    }

    categories.push(cat);
    serde_json::to_writer_pretty(&File::create("data_categories.json").unwrap(), &categories)?;
    Ok(())
}
//Todo:
// ensure files are created
// ...
