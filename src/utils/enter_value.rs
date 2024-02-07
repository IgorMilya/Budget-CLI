use std::io;
use std::io::Write;

pub fn enter_value(comment: &str, error: &str) -> String {
    print!("{}", comment);
    io::stdout().flush().unwrap();
    let mut written_name = String::new();
    io::stdin().read_line(&mut written_name).expect(error);
    written_name
}