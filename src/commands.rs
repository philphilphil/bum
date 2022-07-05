use anyhow::*;

use crate::{
    db,
    model::{BookEntry, EntryType},
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
        _ => return Err(anyhow!("Invalid command.")),
    }

    Ok(())
}
