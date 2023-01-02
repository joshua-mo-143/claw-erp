use std::io;
use std::error::Error;

use sqlx::{SqliteConnection, Connection};

mod utils;
use utils::{create_database, clear_screen};

mod menus;
use menus::products::products_menu;
use menus::sales::sales_menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Claw ERP v0.1.0");
    println!("If you have any problems, queries or questions, feel free to email me at joshua.mo.876@gmail.com");

    if !std::path::Path::new("Test.db").exists() {
        println!("You don't have a database in use at the moment!");
        create_database().await;

    } else {
        println!("Database found.");
    }

    println!("Database connected.");

    loop {
        let mut command = String::new();

        let stdin = io::stdin();
        
        stdin.read_line(&mut command).expect("Had a problem reading your input :(");
        
        match command.trim() {
            "about" => {
                println!("Claw ERP v0.1.0 -- created by Joshua Mo")
            },
            "p" => {
                products_menu().await?;
            }
            "sales" => {
                sales_menu();
            }
            "clear" => {
                clear_screen();
            },
            _ => continue
        }

    }
}