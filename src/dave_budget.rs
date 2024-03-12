use std::collections::HashMap;

struct DaveBudget {
	income: f64,
	expenses: HashMap<String, f64>,
}

impl DaveBudget {
	fn new() -> DaveBudget {
		DaveBudget {
			income: 0.0,
			expenses: HashMap::new(),
		}
	}

	fn add_income(&mut self, amount: f64) {
		self.income += amount;
	}

	fn add_expense(&mut self, expense_name: String, amount: f64) {
		self.expenses.insert(expense_name, amount);
	}

	fn get_balance(&self) -> f64 {
		self.income - self.expenses.values().sum::<f64>()
	}
}

pub fn budget_main() {
	let mut budget = DaveBudget::new();
	budget.add_income(1000.0);
	budget.add_expense(String::from("Rent"), 500.0);
	budget.add_expense(String::from("Groceries"), 200.0);
	budget.add_expense(String::from("Transportation"), 150.0);
	println!("Balance: ${}", budget.get_balance());
}
