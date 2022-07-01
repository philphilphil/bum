use serde_json::Result;
use std::fs::File;

use crate::model::BookEntry;

pub fn try_db() -> Result<()> {
    let exp = BookEntry {
        name: "brot".to_string(),
        category_id: 1,
        amount: 12.23,
        date: chrono::offset::Utc::now(),
    };
    let exp2 = BookEntry {
        name: "22 brot".to_string(),
        category_id: 1,
        amount: 12.23,
        date: chrono::offset::Utc::now(),
    };

    let exp3 = BookEntry {
        name: "3333 brot".to_string(),
        category_id: 1,
        amount: 12.23,
        date: chrono::offset::Utc::now(),
    };

    let v = vec![exp, exp2, exp3];

    let j = serde_json::to_writer(&File::create("data.json").unwrap(), &v)?;
    // println!("{}", j);

    // let p: Expense = serde_json::from_str(data)?;

    //     // Do things just like with any other Rust data structure.
    //     println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
