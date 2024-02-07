use std::io;
use std::io::Write;
use crate::utils::enter_value;

pub fn enter_name(loop_name: &str, loop_error: &str) -> String {
    let mut written_name = enter_value("Enter the name of the cost: ", "Failed to read input");

    while written_name.trim().to_string().is_empty() {
        print!("{}", loop_name);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut written_name).expect(loop_error);
    }

    written_name.trim().to_string()
}