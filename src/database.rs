use crate::utils::{parse_input, InputType};

use cli_table::{print_stdout, Cell, Style, Table};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::{Column, ConnectOptions, Connection, Row};
use std::collections::BTreeMap;
use std::error::Error;
use std::str::FromStr;

pub async fn create_database() -> Result<String, Box<dyn Error>> {
    let mut conn = SqliteConnectOptions::from_str("sqlite://test.db?mode=rwc")?
        .connect()
        .await?;

    sqlx::query(
        r#"CREATE TABLE product (
        id INTEGER PRIMARY KEY,
        Name TEXT NOT NULL,
        Price TEXT NOT NULL,
        Category TEXT
    );
    "#,
    )
    .execute(&mut conn)
    .await?;

    println!("Database created.");
    Ok("OK".to_string())
}

pub async fn retrieve_data(data: &str) -> Result<Data, sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

    let query = sqlx::query(data).fetch_all(&mut conn).await?;

    let mut items = Vec::new();
    let mut headers: BTreeMap<i32, String> = BTreeMap::new();

    for row in query.iter() {
        let mut prod: BTreeMap<i32, String> = BTreeMap::new();

        let id: i32 = row.get("id");
        let id_string = id.to_string();

        prod.insert(0, id_string);

        if id == 1 {

            for (counter, hehe) in row.columns().iter().enumerate() {
                headers.insert(counter as i32, hehe.name().to_string());
   
            }
        }

        for i in 1..row.len() {
            let key = i;
            let value = row.get(i);

            prod.insert(key.try_into().unwrap(), value);
        }
        items.push(prod);
    }
    let returned_data = Data {
        records: items,
        headers
    };
    Ok(returned_data)
}

pub fn view_one_record(data: Data, id: i32) {
    let mut record: Vec<String> = Vec::new();

    for i in data.records {
        if i.get(&0) == Some(&id.to_string()) {
            for e in i.values() {
                record.push(e.to_owned());
            }
        }
    }

    let table_display = vec![record]
        .table()
        .title(data.headers.iter().map(|e| e.1.cell().bold(true)));

    print_stdout(table_display).unwrap();
}

pub fn view_all_records(data: Data) {
    let mut records: Vec<Vec<String>> = Vec::new();

    for i in data.records {
        let mut record: Vec<String> = Vec::new();

        for e in i.values() {
            record.push(e.to_owned());
        }

        records.push(record);
    }

    let table_display = records
        .table()
        .title(data.headers.iter().map(|e| e.1.cell().bold(true)));

    print_stdout(table_display).unwrap();
}

pub async fn create_record(meme: RecordType) -> Result<(), Box<dyn Error>> {
    match meme {
        RecordType::Product => {
            println!("-NEW PRODUCT-");

            let item_name = parse_input("Product name: ", InputType::String);

            let item_price = parse_input("Product price: ", InputType::Price);

            let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

            sqlx::query("INSERT INTO product (name, price) VALUES ($1, $2)")
                .bind(item_name)
                .bind(item_price)
                .execute(&mut conn)
                .await?;

            println!("Added successfully.");
            Ok(())
        }
        RecordType::Customer => todo!(),
        RecordType::SalesOrder => todo!(),
        RecordType::Invoice => todo!(),
    }
}

#[derive(Clone)]
pub struct Data {
    records: Vec<BTreeMap<i32, String>>,
    headers: BTreeMap<i32, String>,
}

pub enum RecordType {
    Product,
    Customer,
    SalesOrder,
    Invoice,
}
