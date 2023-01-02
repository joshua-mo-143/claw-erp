use std::io::{stdout, Write, Stdout};
use crossterm::{execute, terminal, cursor};
use std::io;
use std::error::Error;
use sqlx::{SqliteConnection, Connection};

pub async fn create_database() -> Result<String, Box<dyn Error>> {
    let conn = SqliteConnection::connect("Cats.db").await?;

    println!("Database created.");
    Ok("OK".to_string())
}

pub fn clear_screen() {
    let mut stdout = stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).expect("Something went wrong trying to clear the terminal!");
    execute!(stdout, cursor::MoveTo(1,1)).expect("Something went wrong trying to move the cursor back to the top of the terminal");
    println!("Screen cleared.");

    stdout.flush().expect("Something went wrong trying to flush the input :(");
}

pub fn get_input() -> String {
    let mut command = String::new();

        let stdin = io::stdin();
        
        stdin.read_line(&mut command).expect("Had a problem reading your input :(");

        command
}

pub fn get_input_price() -> String {

    let mut price = String::new();

    let stdin = io::stdin();
    
    stdin.read_line(&mut price).expect("Had a problem reading your input :(");

    let price: f64 = match price.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Expected a number :("),
    };

    format!("£{:.2}", price)
}

pub fn delete_lines(mut stdout: &Stdout, lines: u16) {
    execute!(stdout, cursor::MoveToPreviousLine(lines)).expect("Had an issue moving to the lines that need to be deleted");
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Had an issue clearing the current line");

    stdout.flush().expect("Had an issue flushing stdout :(");
}