mod structures;
mod utils;

use structures::budget::Budget;
use structures::text_style::TextStyle;
use std::io;
use termcolor::{Color};

fn main() {
    let mut budget = Budget::new();


    loop {
        println!("\nChoose an option:");
        println!("1. Add a cost entry");
        println!("2. View all entries for a specific cost name");
        println!("3. View total amount for all costs names");
        println!("4. View All Cost Names");
        println!("5. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading");

        let choice: i32 = match choice.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                utils::error("Wrong value, please tyr another");
                continue;
            }
        };

        match choice {
            1 => {
                let name = utils::enter_name("Enter the name of the cost: ", "Failed to read input");

                let amount = utils::enter_value(
                    "Enter the amount: ", "Failed to read input");

                let amount: f32 = match amount.trim().parse() {
                    Ok(x) => x,
                    Err(_) => {
                        utils::error("Invalid input! Please enter a valid amount.");
                        continue;
                    }
                };

                let comments = utils::enter_value(
                    "Enter the comments (optional): ", "Failed to read input");

                let comments = if comments.trim().is_empty() { None } else { Some(comments) };
                budget.add_costs(name, amount, comments);
            }
            2 => {
                let name = utils::enter_name("Enter the name of the cost: ", "Failed to read input");
                budget.get_all_costs_by_name(name);
            }
            3 => {
                let total_costs = budget.get_total_amount_of_all_costs();

                if total_costs.is_empty() {
                    utils::warning("\nNothing in our budget");
                    continue;
                }

                for (name, total) in total_costs {
                    let text_color = TextStyle::default()
                        .color(Color::Cyan)
                        .italic();

                    text_color.set_styles_for_text(&format!("\nName: {},\nTotal: {}\n", name, total));
                }
            }
            4 => {
                let all_names = budget.get_all_costs_names();

                if all_names.is_empty() {
                    utils::warning("\nNothing in our budget");
                    continue;
                }

                for name in all_names {
                    let text_color = TextStyle::default()
                        .color(Color::Green)
                        .bold()
                        .underline();

                    text_color.set_styles_for_text(&format!("{}\n", name));
                }
            }
            5 => {
                println!("\nExiting...");
                break;
            }
            _ => {}
        }
    }
}
