use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DaveBudget {
	pub income: f64,
	pub expenses: HashMap<String, f64>,
}

impl DaveBudget {
	pub fn new() -> DaveBudget {
		DaveBudget {
			income: 0.0,
			expenses: HashMap::new(),
		}
	}

	pub fn add_income(&mut self, amount: f64) {
		self.income += amount;
	}

	pub fn add_expense(&mut self, expense_name: String, amount: f64) {
		self.expenses.insert(expense_name, amount);
	}

	/*fn remove_expense(&mut self, expense_name: String, amount: f64) {
		self.expenses.remove(&expense_name, amount);
	}*/

	pub fn get_balance(&self) -> f64 {
		self.income - self.expenses.values().sum::<f64>()
	}
}
