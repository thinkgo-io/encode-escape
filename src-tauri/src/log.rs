use colored::*;

pub fn log(_message: &str) {
    // println!("{}", message);
    // println!();
}

pub fn log_lines(lines: Vec<&str>) {
    for line in lines {
        log(line);
    }
}

pub fn log_error(context: &str, message: &str) {
    println!("{} {}", "Error:".bright_red(), context.bright_red());
    println!("       {}", message);
    println!();
}
