use std::collections::HashMap;
use crate::structures::costs::Costs;


pub struct Budget {
    pub budget: HashMap<String, Vec<Costs>>,
}

impl Budget {
    pub fn new() -> Self {
        Self {
            budget: HashMap::new()
        }
    }

    pub fn add_costs(&mut self, name: String, amount: f32, comments: Option<String>) {
        let costs = Costs::new(amount, comments);
        self.budget
            .entry(name)
            .or_insert(Vec::new())
            .push(costs);
    }

    pub fn get_all_costs_by_name(&self, name: String) {
        match self.budget.get(&name) {
            Some(costs) => {
                println!("\nAll entries for {}:", name);
                for Costs { amount, comments } in costs {
                    println!("\nAmount: {}", amount);
                    if let Some(comment) = comments {
                        println!("Comment: {}", comment)
                    }
                }
            }
            None => {
                println!("No entries found for {}", name);
            }
        };
    }

    pub fn get_total_amount_of_all_costs(&self) -> HashMap<&String, f32> {
        self.budget
            .iter().map(|(name, costs)| {
            let total_amount: f32 = costs.iter().map(|cost| cost.amount).sum();
            (name, total_amount)
        })
            .collect::<HashMap<&String, f32>>()
    }
    pub fn get_all_costs_names(&self) -> Vec<&String> {
        self.budget.keys().collect()
    }
}