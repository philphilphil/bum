use crate::model::{EntryType, RecurringEntry};

pub struct CalcResult {
    pub name: String,
    pub amount: f32,
}

pub fn calculate_total(items: &Vec<RecurringEntry>) -> Vec<CalcResult> {
    let mut result = vec![];
    let mut categories: Vec<String> = items.iter().map(|c| c.category_token.to_string()).collect();
    categories.sort();
    categories.dedup();

    for cat in categories {
        let rec: Vec<&RecurringEntry> = items.iter().filter(|c| c.category_token == cat).collect();

        let mut sum = rec.iter().map(|c| c.amount).sum();

        if rec.first().unwrap().kind == EntryType::Expense {
            sum *= -1.0;
        }

        result.push(CalcResult {
            name: cat,
            amount: sum,
        });
    }
    result
}
