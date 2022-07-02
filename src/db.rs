use serde_json::Result;
use std::fs::File;

use crate::model::{BookEntry, EntryType};

pub fn create_test_bookings() -> Result<()> {
    let exp = BookEntry::new("eins", EntryType::Income, 5, 13.22);
    let exp2 = BookEntry::new("zwei", EntryType::Income, 3, 983.22);
    let exp3 = BookEntry::new("drei", EntryType::Income, 3, 1.22);
    let v = vec![exp, exp2, exp3];

    let j = serde_json::to_writer_pretty(&File::create("data.json").unwrap(), &v)?;
    // println!("{}", j);

    // let p: Expense = serde_json::from_str(data)?;

    //     // Do things just like with any other Rust data structure.
    //     println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

pub fn get_bookings() -> Vec<BookEntry> {
    let b: Vec<BookEntry> = serde_json::from_reader(&File::open("data.json").unwrap()).unwrap();
    println!("{:?}", b);
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
