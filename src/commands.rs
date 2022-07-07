use anyhow::*;

use crate::{
    db,
    model::{BookEntry, Category, EntryType, RecurringEntry, RecurringType},
};

// TODO:
// - move each command into its own method
// - add regex for validation

pub fn handle_command(cmd: &str) -> Result<()> {
    let action: Vec<&str> = cmd.split(' ').collect();
    let mut rec_type = RecurringType::Monthly;
    if action.len() > 4 && action[4] == "yearly" {
        rec_type = RecurringType::Yearly;
    }

    match action[0] {
        "ae" => {
            let exp = BookEntry::new(
                action[1],
                EntryType::Expense,
                action[2],
                action[3].parse::<f32>().unwrap(),
            );
            db::add_booking(exp)?;
        }
        "ari" => {
            let rec = RecurringEntry::new(
                action[1],
                EntryType::Income,
                action[2],
                action[3].parse::<f32>().unwrap(),
                rec_type,
            );
            db::add_recurring(rec)?;
        }

        "are" => {
            let rec = RecurringEntry::new(
                action[1],
                EntryType::Expense,
                action[2],
                action[3].parse::<f32>().unwrap(),
                rec_type,
            );
            db::add_recurring(rec)?;
        }
        "ac" => {
            let c = Category::new(action[1], action[2]);
            db::add_category(c)?;
        }

        _ => return Err(anyhow!("Invalid command.")),
    }

    Ok(())
}
