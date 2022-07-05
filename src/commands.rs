use anyhow::*;

use crate::{
    db,
    model::{BookEntry, Category, EntryType},
};

pub fn handle_command(cmd: &str) -> Result<()> {
    let action: Vec<&str> = cmd.split(' ').collect();

    match action[0] {
        "adde" => {
            let exp = BookEntry::new(
                action[1],
                EntryType::Expense,
                5,
                action[2].parse::<f32>().unwrap(),
            );
            db::add_booking(exp)?;
        }
        "addc" => {
            let c = Category::new(action[1], action[2]);
            db::add_category(c)?;
        }

        _ => return Err(anyhow!("Invalid command.")),
    }

    Ok(())
}
