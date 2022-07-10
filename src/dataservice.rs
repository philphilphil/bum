use crate::{
    db,
    model::{EntryType, RecurringEntry, RecurringType},
};
use anyhow::Result;

pub struct CategorySum {
    pub name: String,
    pub amount: f32,
}

pub fn calculate_categorie_sums() -> Result<Vec<CategorySum>> {
    let items = db::get_recurring()?;
    let mut result = vec![];
    let mut categories: Vec<String> = items.iter().map(|c| c.category_token.to_string()).collect();
    categories.sort();
    categories.dedup();

    for cat in categories {
        let rec: Vec<&RecurringEntry> = items.iter().filter(|c| c.category_token == cat).collect();

        let monthly_sum: f32 = rec
            .iter()
            .filter(|e| e.rate_type == RecurringType::Monthly)
            .map(|c| c.amount)
            .sum();

        let yearly_sum: f32 = rec
            .iter()
            .filter(|e| e.rate_type == RecurringType::Yearly)
            .map(|c| c.amount)
            .sum();

        let mut sum = monthly_sum + (yearly_sum / 12.0);

        if rec.first().unwrap().kind == EntryType::Expense {
            sum *= -1.0;
        }

        result.push(CategorySum {
            name: cat,
            amount: sum,
        });
    }
    Ok(result)
}

pub fn get_recurring(kind: EntryType) -> Result<Vec<RecurringEntry>> {
    let recurring = db::get_recurring()?
        .into_iter()
        .filter(|c| c.kind == kind)
        .collect();
    Ok(recurring)
}
