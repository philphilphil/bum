use serde_json::Result;
use std::fs::File;

use crate::model::BookEntry;

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

//Todo:
// ensure files are created
// ...
