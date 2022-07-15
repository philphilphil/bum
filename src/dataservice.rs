use std::collections::HashMap;

use crate::{
    db,
    model::{BookingType, BudgetBooking, Category, RecurringBooking, RecurringType},
};
use anyhow::Result;

pub struct CategorySum {
    pub name: String,
    pub amount: f32,
}

#[derive(Default)]
pub struct DataService {
    pub total_income: f32,
    pub total_reccuring_expenses: f32,
    pub total_budget_spent: f32,
    pub total_budget_left: f32,
    recurring_bookings: Vec<RecurringBooking>,
    budget_bookings: Vec<BudgetBooking>,
    budget_bookings_archive: Vec<BudgetBooking>,
    categories: Vec<Category>,
}

impl DataService {
    pub fn new() -> Self {
        let mut data_service = DataService::default();
        data_service.load_data().expect("Issue loading data");
        data_service.calculate().expect("Issue calculating");
        data_service
    }

    pub fn load_data(&mut self) -> Result<()> {
        self.recurring_bookings = db::get_recurring()?;
        self.budget_bookings = db::get_expenses()?;
        self.budget_bookings_archive = db::get_expenses_archive()?;
        self.categories = db::get_categories()?;
        Ok(())
    }

    pub fn calculate_reccuring_categorie_sums(&self) -> Result<Vec<CategorySum>> {
        let mut result = vec![];
        let mut categories: Vec<String> = self
            .recurring_bookings
            .iter()
            .map(|c| c.category_token.to_string())
            .collect();
        categories.sort();
        categories.dedup();

        for cat in categories {
            let rec: Vec<&RecurringBooking> = self
                .recurring_bookings
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

            if rec.first().unwrap().kind == BookingType::Expense {
                sum *= -1.0;
            }

            result.push(CategorySum {
                name: cat,
                amount: sum,
            });
        }

        let expenses = CategorySum {
            name: "Budget Expenses".to_string(),
            amount: self.total_budget_spent * -1.0,
        };
        result.push(expenses);

        Ok(result)
    }

    pub fn get_recurring(&self, kind: BookingType) -> Result<Vec<&RecurringBooking>> {
        let recurring = self
            .recurring_bookings
            .iter()
            .filter(|c| c.kind == kind)
            .collect();
        Ok(recurring)
    }

    pub fn get_all_bookings(&self) -> Result<&Vec<BudgetBooking>> {
        Ok(&self.budget_bookings)
    }

    pub fn get_bookings(&self, kind: BookingType) -> Result<Vec<&BudgetBooking>> {
        let bookings = self
            .budget_bookings
            .iter()
            .filter(|i| i.kind == kind)
            .collect();
        Ok(bookings)
    }

    pub fn get_bookings_archive(&self) -> Result<&Vec<BudgetBooking>> {
        Ok(&self.budget_bookings_archive)
    }

    pub fn get_categorie_map(&self) -> Result<HashMap<String, String>> {
        let mut categorie_map = HashMap::new();
        for c in &self.categories {
            categorie_map.insert(c.token.clone(), c.name.clone());
        }
        Ok(categorie_map)
    }

    pub fn calculate(&mut self) -> Result<()> {
        let recurring_expense_bookings = self.get_recurring(BookingType::Expense)?;
        let budget_bookings = self.get_bookings(BookingType::Expense)?;
        let budget_bookins_income = self.get_bookings(BookingType::Income)?;
        let (monthly_bookings, yearly_bookings): (Vec<&RecurringBooking>, Vec<&RecurringBooking>) =
            recurring_expense_bookings
                .into_iter()
                .partition(|r| r.rate_type == RecurringType::Monthly);
        let income_bookings = self.get_recurring(BookingType::Income)?;

        let income = income_bookings.iter().map(|i| i.amount).sum();
        let monthly: f32 = monthly_bookings.iter().map(|i| i.amount).sum();
        let yearly: f32 = yearly_bookings.iter().map(|i| i.amount).sum::<f32>() / 12.0;
        let budget_spent: f32 = budget_bookings.iter().map(|b| b.amount).sum();
        let budget_income: f32 = budget_bookins_income.iter().map(|b| b.amount).sum();

        self.total_income = income;
        self.total_reccuring_expenses = yearly + monthly;
        self.total_budget_left = income - (monthly + yearly + budget_spent - budget_income);
        self.total_budget_spent = budget_spent - budget_income;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::BookingType::*;
    use crate::model::RecurringType::*;

    #[test]
    fn test_simple_calculation() {
        let mut ds = DataService::default();
        let budget_bookings = vec![
            BudgetBooking::new("T", Expense, "tt", 2.00),
            BudgetBooking::new("T", Expense, "tt", 1.00),
            BudgetBooking::new("T", Expense, "tt", 1.00),
            BudgetBooking::new("T", Expense, "tt", 3.00),
            BudgetBooking::new("T", Income, "tt", 3.00),
        ];
        let recurring_bookings = vec![
            RecurringBooking::new("Ti", Expense, "tt", 1.00, Monthly),
            RecurringBooking::new("Ti", Expense, "tt", 12.00, Yearly),
            RecurringBooking::new("Ti", Income, "tt", 10.00, Monthly),
        ];

        ds.budget_bookings = budget_bookings;
        ds.recurring_bookings = recurring_bookings;
        ds.calculate().unwrap();

        assert_eq!(ds.total_budget_left, 4.00);
        assert_eq!(ds.total_income, 10.00);
        assert_eq!(ds.total_reccuring_expenses, 2.00);
        assert_eq!(ds.total_budget_spent, 4.00);
    }

    #[test]
    fn test_complex_calculation() {
        let mut ds = DataService::default();
        let budget_bookings = vec![
            BudgetBooking::new("E1", Expense, "tt", 3.00),
            BudgetBooking::new("E2", Expense, "tt", 33.12),
            BudgetBooking::new("E3", Expense, "tt", 13.49),
            BudgetBooking::new("E4", Expense, "tt", 32.00),
            BudgetBooking::new("E5", Expense, "tt", 22.22),
            BudgetBooking::new("E6", Expense, "tt", 750.00),
            BudgetBooking::new("E7", Expense, "tt", 123.01),
            BudgetBooking::new("I1", Income, "tt", 10.00),
            BudgetBooking::new("I2", Income, "tt", 33.33),
            BudgetBooking::new("I3", Income, "tt", 5.49),
        ];
        let recurring_bookings = vec![
            RecurringBooking::new("Income", Income, "tt", 4000.00, Monthly),
            RecurringBooking::new("M1", Expense, "tt", 1.00, Monthly),
            RecurringBooking::new("M2", Expense, "tt", 2.00, Monthly),
            RecurringBooking::new("M3", Expense, "tt", 22.50, Monthly),
            RecurringBooking::new("M4", Expense, "tt", 11.00, Monthly),
            RecurringBooking::new("M5", Expense, "tt", 21.85, Monthly),
            RecurringBooking::new("M6", Expense, "tt", 7.01, Monthly),
            RecurringBooking::new("M7", Expense, "tt", 41.00, Monthly),
            RecurringBooking::new("M8", Expense, "tt", 600.00, Monthly),
            RecurringBooking::new("M9", Expense, "tt", 1001.11, Monthly),
            RecurringBooking::new("Y1", Expense, "tt", 12.00, Yearly),
            RecurringBooking::new("Y2", Expense, "tt", 389.00, Yearly),
            RecurringBooking::new("Y3", Expense, "tt", 72.22, Yearly),
        ];

        ds.budget_bookings = budget_bookings;
        ds.recurring_bookings = recurring_bookings;
        ds.calculate().unwrap();

        assert_eq!(ds.total_budget_left, 1325.075);
        assert_eq!(ds.total_income, 4000.00);
        assert_eq!(ds.total_reccuring_expenses, 1746.905);
        assert_eq!(ds.total_budget_spent, 928.02);
    }

    #[test]
    fn test_categories() {
        let mut ds = DataService::default();
        let categories = vec![
            Category::new("Cat A", "a"),
            Category::new("b", "b"),
            Category::new("c", "c"),
            Category::new("Cat D", "dd"),
            Category::new("e", "e"),
            Category::new("f", "ff"),
        ];
        ds.categories = categories;
        let map = ds.get_categorie_map().unwrap();

        assert_eq!(map.get("a").unwrap(), "Cat A");
        assert_eq!(map.get("dd").unwrap(), "Cat D");
        assert_eq!(map.get("ff").unwrap(), "f");
        assert_eq!(map.get("g"), None);
    }
}
