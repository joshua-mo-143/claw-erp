use std::error::Error;
use std::io;

mod utils;
use utils::invalid_input;

mod database;
use database::create_database;

mod menus;
use menus::products::products_menu;
use menus::sales::sales_menu;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("----------------");
    println!("Claw ERP v0.1.0");
    if !std::path::Path::new("test.db").exists() {
        println!("You don't have a database in use at the moment!");
        create_database().await?;
    }

    loop {
        let mut command = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut command)
            .expect("Had a problem reading your input :(");

        match command.trim() {
            "help" => {
                print!(
                    r#"You're currently in the main menu!
Commands:
        "about" - Get information about this application.
        "products" - Access the Products menu (handle products, stock levels etc from here).
        "sales" - Access the Sales menu (access sales orders, invoicing etc from here).  
        "#
                )
            }
            "about" => {
                println!("Claw ERP v0.1.0 -- created by Joshua Mo")
            }
            "products" => {
                products_menu().await?;
            }
            "sales" => {
                sales_menu();
            }
            _ => {
                invalid_input();
                continue;
            }
        }
    }
}
