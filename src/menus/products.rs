use crossterm::event::{Event, read};

use sqlx::{SqliteConnection, Connection};
use sqlx::Row;
use std::error::Error;
use crate::utils::{get_input, delete_lines, get_input_price};
use std::io::{stdout};

pub async fn products_menu(conn: &SqliteConnection) -> Result<(), Box<dyn Error>> {
    println!("-PRODUCTS MENU-");
    println!("Press `h` for help.");

    loop {
        match read()? {
            Event::Key(event) => {
                match event.code { 
                    crossterm::event::KeyCode::Char('h') => println!("Hello world"),
                    crossterm::event::KeyCode::Char('c') => create_product().await?,
                    crossterm::event::KeyCode::Char('v') => view_products().await?,
                    _ => continue
            }
            },
            _ => continue
        };
    }
}

async fn create_product() -> Result<(), Box<dyn Error>> {
    println!("-NEW PRODUCT-");

    let stdout = stdout();

    println!("Product name: ");
    let item_name = get_input();

    delete_lines(&stdout, 2);
    print!("Product name: {}", item_name);

    println!("Product price: ");
    let item_price = get_input_price();

    delete_lines(&stdout, 2);
    print!("Product price: {}\r\n", item_price);

    let mut conn = SqliteConnection::connect("Cats.db").await?;

     sqlx::query("INSERT INTO product (name, price) VALUES ($1, $2)")
        .bind(item_name)
        .bind(item_price)
        .execute(&mut conn)
        .await?;

        println!("meme");
        Ok(())
}

async fn view_products() -> Result<(), Box<dyn Error>> {

    let mut conn = SqliteConnection::connect("Cats.db").await?;

    let meme = sqlx::query("SELECT * FROM product")
        .fetch_all(&mut conn).await?;

    println!("Loading...");

    for row in meme.into_iter() {
        let meme = Product {
            id: row.get("id"),
            name: row.get("name"),
            price: row.get("price")
        };
        println!("{}", meme);
    }

    Ok(())
}

#[derive(Debug)]
struct Product {
    id: Option<u32>,
    name: String,
    price: String,
}