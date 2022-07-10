use std::collections::HashMap;

use crate::{
    db,
    model::{EntryType, RecurringEntry, RecurringType},
};
use anyhow::Result;

pub struct CategorySum {
    pub name: String,
    pub amount: f32,
}

// FIXME: refactor the data service so it only gets the data from db once

pub fn calculate_categorie_sums() -> Result<Vec<CategorySum>> {
    let rec = db::get_recurring()?;
    let exp = db::get_expenses()?;

    let mut result = vec![];
    let mut categories: Vec<String> = rec.iter().map(|c| c.category_token.to_string()).collect();
    categories.sort();
    categories.dedup();

    for cat in categories {
        let rec: Vec<&RecurringEntry> = rec.iter().filter(|c| c.category_token == cat).collect();

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

    let expepnse = CategorySum {
        name: "Period Expenses".to_string(),
        amount: exp.iter().map(|e| e.amount).sum::<f32>() * -1.0,
    };
    result.push(expepnse);

    Ok(result)
}

pub fn get_recurring(kind: EntryType) -> Result<Vec<RecurringEntry>> {
    let recurring = db::get_recurring()?
        .into_iter()
        .filter(|c| c.kind == kind)
        .collect();
    Ok(recurring)
}

pub fn get_categorie_map() -> Result<HashMap<String, String>> {
    let mut categorie_map = HashMap::new();
    let categories = db::get_categories()?;
    for c in categories {
        categorie_map.insert(c.token.clone(), c.name.clone());
    }
    Ok(categorie_map)
}

pub struct SumOverview {
    pub income: f32,
    pub expenses: f32,
    pub budget_spent: f32,
    pub budget_left: f32,
}
pub fn calc_overview() -> Result<SumOverview> {
    let recurring_expense_entries = get_recurring(EntryType::Expense)?;
    let (monthly_entries, yearly_entries): (Vec<RecurringEntry>, Vec<RecurringEntry>) =
        recurring_expense_entries
            .into_iter()
            .partition(|r| r.rate_type == RecurringType::Monthly);
    let income_entries = get_recurring(EntryType::Income)?;
    let budget_spent_entries = db::get_expenses()?;

    let income = income_entries.iter().map(|i| i.amount).sum();
    let monthly: f32 = monthly_entries.iter().map(|i| i.amount).sum();
    let yearly: f32 = yearly_entries.iter().map(|i| i.amount).sum::<f32>() / 12.0;
    let budget_spent = budget_spent_entries.iter().map(|i| i.amount).sum();

    Ok(SumOverview {
        income,
        expenses: yearly + monthly,
        budget_spent,
        budget_left: income - (monthly + yearly + budget_spent),
    })
}
