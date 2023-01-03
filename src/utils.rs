use std::{io, io::Write, error::Error};
use crossterm::{execute, terminal, cursor};
use std::str::{FromStr};
use sqlx::{ConnectOptions, Row, Column, sqlite::{SqliteConnectOptions, SqliteRow}};
use std::io::stdout;

pub async fn create_database() -> Result<String, Box<dyn Error>> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://test.db?mode=rwc")?
        .connect().await?;

    sqlx::query(r#"CREATE TABLE product (
        id INTEGER PRIMARY KEY,
        Name TEXT NOT NULL,
        Price TEXT NOT NULL,
        Category TEXT
    );
    "#
        ).execute(&mut conn).await?;

    println!("Database created.");
    Ok("OK".to_string())
}

pub fn get_input() ->String {
    let mut input = String::new();

        let stdin = io::stdin();
        
        stdin.read_line(&mut input).expect("Had a problem reading your input :(");

        input
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

pub fn delete_lines(mut stdout: &io::Stdout, lines: u16) {
    execute!(stdout, cursor::MoveToPreviousLine(lines)).expect("Had an issue moving to the lines that need to be deleted");
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Had an issue clearing the terminal lines");
    stdout.flush().expect("Had an issue flushing stdout :(");
}

pub fn get_data(data: Vec<SqliteRow>) -> Vec<Vec<String>> {
    let mut product_data = Vec::new();

    for row in data.into_iter() {
        let mut record = Vec::new();

        let id: i32 = row.get("id");
        let id_string = id.to_string();
        record.push(mem);
        for i in 1..row.len() {

            let item = row.get(i);
            record.push(item);
        }

        product_data.push(record);
    }

    product_data
}

pub fn get_one_row(data: SqliteRow) -> Data {
    let mut product_data = Vec::new();

        let mut headers = Vec::new();

        let mut record: Vec<String> = Vec::new();

        let id: i32 = data.get("id");

        let id_string = id.to_string();
        record.push(id_string);
        headers.push("ID".to_string());

        for i in 1..data.len() {
            headers.push(data.column(i).name().to_string());
            let item = data.get(i);
            record.push(item);
        }
        product_data.push(meme);
    
    let returned_data = Data {
        data: product_data,
        headers: headers
    };

    returned_data
}

pub fn invalid_input() {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveToPreviousLine(1)).expect("Had an issue moving to the lines that need to be deleted");
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorDown)).expect("Had an issue clearing the terminal lines");
    println!("Invalid command - try using `help` to see valid commands.")
}

pub struct Data {
    pub data: Vec<Vec<String>>,
    pub headers: Vec<String>
}