use std::collections::HashMap;
use chrono::{DateTime, Local};
use termcolor::Color;
use crate::structures::costs::Costs;
use crate::structures::text_style::TextStyle;
use crate::utils;


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
        let current_time: DateTime<Local> = Local::now();
        let timestamp = current_time.format("%Y-%m-%d %H:%M:%S").to_string();

        let costs = Costs::new(amount, comments, timestamp);
        self.budget
            .entry(name)
            .or_insert(Vec::new())
            .push(costs);

        for (_, vec) in self.budget.iter_mut() {
            vec.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        }
    }

    pub fn get_all_costs_by_name(&self, name: String) {
        match self.budget.get(&name) {
            Some(costs) => {
                let text_color = TextStyle::default()
                    .color(Color::Green)
                    .bold()
                    .underline();

                text_color.set_styles_for_text(&format!("\nAll entries for {}:", name));

                for Costs { amount, comments, timestamp } in costs {
                    self.styles_for_all_costs_by_name(&format!("\nAmount: {}\n", amount));

                    if let Some(comment) = comments {
                        self.styles_for_all_costs_by_name(&format!("Comment: {}\n", comment.trim()));
                    }

                    self.styles_for_all_costs_by_name(&format!("Time: {}\n", timestamp));
                }
            }
            None => {
                utils::warning(&format!("\nNo entries found for {}", name));
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

    fn styles_for_all_costs_by_name(&self, text: &str) {
        let text_color = TextStyle::default()
            .color(Color::Blue)
            .italic();

        text_color.set_styles_for_text(text);
    }
}