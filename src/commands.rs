use anyhow::*;

use crate::{
    db,
    model::{BookingType, BudgetBooking, Category, RecurringBooking, RecurringType},
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
            let exp = BudgetBooking::new(
                action[1],
                BookingType::Expense,
                action[2],
                action[3].parse::<f32>().unwrap(),
            );
            db::add_expense(exp)?;
        }
        "ari" => {
            let rec = RecurringBooking::new(
                action[1],
                BookingType::Income,
                action[2],
                action[3].parse::<f32>().unwrap(),
                rec_type,
            );
            db::add_recurring(rec)?;
        }

        "are" => {
            let rec = RecurringBooking::new(
                action[1],
                BookingType::Expense,
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
