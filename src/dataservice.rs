use std::collections::HashMap;

use crate::{
    db,
    model::{BookEntry, Category, EntryType, RecurringEntry, RecurringType},
};
use anyhow::Result;

pub struct CategorySum {
    pub name: String,
    pub amount: f32,
}

#[derive(Default)]
pub struct DataService {
    pub total_income: f32,
    pub total_expenses: f32,
    pub total_budget_spent: f32,
    pub total_budget_left: f32,
    recurring_entries: Vec<RecurringEntry>,
    budget_entries: Vec<BookEntry>,
    categories: Vec<Category>,
}
impl DataService {
    pub fn new() -> Self {
        let mut data_service = DataService {
            total_income: 0.0,
            total_expenses: 0.0,
            total_budget_spent: 0.0,
            total_budget_left: 0.0,
            // FIXME: fix error handling
            recurring_entries: db::get_recurring().unwrap(),
            budget_entries: db::get_expenses().unwrap(),
            categories: db::get_categories().unwrap(),
        };

        data_service.calc_overview().unwrap();
        data_service
    }

    pub fn reload(&mut self) {
        self.recurring_entries = db::get_recurring().unwrap();
        self.budget_entries = db::get_expenses().unwrap();
        self.categories = db::get_categories().unwrap();
        self.calc_overview().unwrap();
    }

    pub fn calculate_categorie_sums(&self) -> Result<Vec<CategorySum>> {
        let mut result = vec![];
        let mut categories: Vec<String> = self
            .recurring_entries
            .iter()
            .map(|c| c.category_token.to_string())
            .collect();
        categories.sort();
        categories.dedup();

        for cat in categories {
            let rec: Vec<&RecurringEntry> = self
                .recurring_entries
                .iter()
                .filter(|c| c.category_token == cat)
                .collect();

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

        let expenses = CategorySum {
            name: "Period Expenses".to_string(),
            amount: self.total_expenses * -1.0,
        };
        result.push(expenses);

        Ok(result)
    }

    pub fn get_recurring(&self, kind: EntryType) -> Result<Vec<&RecurringEntry>> {
        let recurring = self
            .recurring_entries
            .iter()
            .filter(|c| c.kind == kind)
            .collect();
        Ok(recurring)
    }

    pub fn get_categorie_map(&self) -> Result<HashMap<String, String>> {
        let mut categorie_map = HashMap::new();
        for c in &self.categories {
            categorie_map.insert(c.token.clone(), c.name.clone());
        }
        Ok(categorie_map)
    }

    fn calc_overview(&mut self) -> Result<()> {
        let recurring_expense_entries = self.get_recurring(EntryType::Expense)?;
        let (monthly_entries, yearly_entries): (Vec<&RecurringEntry>, Vec<&RecurringEntry>) =
            recurring_expense_entries
                .into_iter()
                .partition(|r| r.rate_type == RecurringType::Monthly);
        let income_entries = self.get_recurring(EntryType::Income)?;

        let income = income_entries.iter().map(|i| i.amount).sum();
        let monthly: f32 = monthly_entries.iter().map(|i| i.amount).sum();
        let yearly: f32 = yearly_entries.iter().map(|i| i.amount).sum::<f32>() / 12.0;
        let budget_spent = self.budget_entries.iter().map(|i| i.amount).sum();

        self.total_income = income;
        self.total_expenses = yearly + monthly;
        self.total_budget_left = income - (monthly + yearly + budget_spent);
        self.total_budget_spent = budget_spent;

        Ok(())
    }
}
