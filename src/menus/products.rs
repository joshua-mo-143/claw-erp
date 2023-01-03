use cli_table::{Table, Cell, print_stdout, Style};
use sqlx::{SqliteConnection, Connection};
use std::error::Error;
use crate::utils::{get_input, delete_lines, get_input_price, get_data, get_one_row, invalid_input};
use std::io::{stdout};

pub async fn products_menu() -> Result<(), Box<dyn Error>> {
    println!("-PRODUCTS MENU-");
    println!("Type `help` for a list of commands you can run, or type `home` to go back to the main menu.");

    loop {

        let meme = get_input();

        let mut input = meme.split(' ');
        
        let cmd = input.next().unwrap();

        match cmd.trim() {
            "help" => {
                println!("`create` -- Create a product.");
                println!("`view` -- View all products.");
                println!("`home` -- Go back to the main menu.");
            },
            "new" => create_product().await?,
            "view" => {
                match input.next() {
                    Some(x) => {
                        match x.trim().parse::<i32>() {
                            Ok(num) => {
                                view_one_product(num).await?;
                                continue
                            },
                            Err(err) => {
                                println!("Error: {}", err);
                                continue
                            }
                        }
                    },
                    _ => {
                        view_all_products().await?;
                        continue
                    }
                }

            },
            "home" => {
                 println!("-HOME MENU-");
                 break;
             },
            _ => {invalid_input(); continue}
        }
    }
    Ok(())
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

    let mut conn = SqliteConnection::connect("test.db").await?;

     sqlx::query("INSERT INTO product (name, price) VALUES ($1, $2)")
        .bind(item_name)
        .bind(item_price)
        .execute(&mut conn)
        .await?;

        println!("Added successfully.");
        Ok(())
}

async fn view_all_products() -> Result<(), Box<dyn Error>> {

    let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

    let query = sqlx::query("SELECT * FROM product").fetch_all(&mut conn).await?;

    let table_display = get_data(query).table().title(vec!["ID".cell(), "Item Name".cell(), "Item Price".cell()]);

    println!("{}", table_display.display().unwrap());

    Ok(())
}

async fn view_one_product(id: i32) -> Result<(), Box<dyn Error>> {

    let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

    let query = sqlx::query("SELECT * FROM product WHERE id = $1")
    .bind(id)
    .fetch_one(&mut conn).await?;

    let product = get_one_row(query);
    
    let table_display = product.data.table().title(product.headers.iter().map(|e| e.cell().bold(true)));

    print_stdout(table_display).unwrap();

    Ok(())

}