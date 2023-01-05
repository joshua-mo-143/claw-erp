use crossterm::{cursor, execute, terminal};
use std::io::stdout;
use std::{io, io::Write};

pub fn read_input() -> String {
    let mut input = String::new();

    let stdin = io::stdin();

    stdin
        .read_line(&mut input)
        .expect("Had a problem reading your input :(");

    input
}

pub fn parse_input(string: &str, input_type: InputType) -> String {
    println!("{}", string);

    let stdout = stdout();

    let input = read_input();

    delete_lines(&stdout, 2);
    match input_type {
        InputType::String => {
            println!("{}: {}", string, input);
            input
        }
        InputType::Num => match input.parse::<i64>() {
            Ok(num) => {
                print!("{} {}", string, num);
                num.to_string()
            }
            Err(err) => {
                println!("Error: {} - setting amount to 0.", err);
                "0".to_string()
            }
        },
        InputType::Price => match input.parse::<f64>() {
            Ok(num) => {
                println!("{}: £{}", string, num);
                format!("£{}", num.to_string())
            }
            Err(err) => {
                println!("Error: {} - setting price at £0.00.", err);
                "£0.00".to_string()
            }
        },
    }
}

pub fn delete_lines(mut stdout: &io::Stdout, lines: u16) {
    execute!(stdout, cursor::MoveToPreviousLine(lines))
        .expect("Had an issue moving to the lines that need to be deleted");
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown))
        .expect("Had an issue clearing the terminal lines");
    stdout.flush().expect("Had an issue flushing stdout :(");
}

pub fn invalid_input() {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveToPreviousLine(1))
        .expect("Had an issue moving to the lines that need to be deleted");
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown))
        .expect("Had an issue clearing the terminal lines");
    println!("Invalid command - try using `help` to see valid commands.")
}

pub enum InputType {
    String,
    Num,
    Price,
}
