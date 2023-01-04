use cli_table::{Table, Cell, print_stdout, Style};
use sqlx::{SqliteConnection, Connection, query, Row, Column};
use std::error::Error;
use crate::utils::{get_input, delete_lines, get_input_price, get_data, get_one_row, invalid_input};
use std::io::{stdout};
use std::collections::HashMap;

pub async fn products_menu() -> Result<(), Box<dyn Error>> {
    println!("-PRODUCTS MENU-");
    println!("Type `help` for a list of commands you can run, or type `home` to go back to the main menu.");

    let data = retrieve_data("SELECT * from product").await?;

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
                                view_one_prod(&data.records, num);
                                continue
                            },
                            Err(err) => {
                                if x.trim() == "all" {
                                    view_all_products().await?;
                                }
                                    else {
                                        println!("Error: {}", err);
                                        println!("Usage: `view <ITEM_ID_HERE>`");
                                        continue
                                    }
                                
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
    };
    
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

    let products = get_data(query);

    let table_display = products.data.table().title(products.headers.iter().map(|e| e.cell().bold(true)));

    print_stdout(table_display).unwrap();

    Ok(())
}

pub async fn retrieve_data(data: &str) -> Result<Data, sqlx::Error> {

    let mut conn = SqliteConnection::connect("sqlite://test.db").await?;

    let query = sqlx::query(data).fetch_all(&mut conn).await?;

    let mut items = Vec::new();
    let mut headers: HashMap<i32, String> = HashMap::new();

    for row in query.iter() {

        let mut prod: HashMap<String, String> = HashMap::new();

        let id: i32 = row.get("id");
        let id_string = id.to_string();

        prod.insert(row.column(0).name().to_string(), id_string);

        if id == 1 {
            let mut counter = 0;
            for hehe in row.columns() {
                headers.insert(counter, hehe.name().to_string());
                counter += 1;
            }
        }

        for i in 1..row.len() {
            
            let key =  row.column(i).name().to_string();
            let value = row.get(i);

            prod.insert(key, value);

        }
        items.push(prod);
    }

    println!("{:?}", headers);
    let returned_data = Data {records: items, headers: headers};
    Ok(returned_data)
}

pub fn view_one_prod(data: &Vec<HashMap<String, String>>, id: i32) {

    let mut record: Vec<String> = Vec::new();
    let mut headers = Vec::new();
    for i in data {
        if i.get("id") == Some(&id.to_string()) {
            for e in i.values() {
                record.push(e.to_owned());
            }

            for header in i.keys() {
                headers.push(header.to_owned());
            }
        }
    }

    let table_display = vec![&record].table().title(headers.iter().map(|e| e.cell().bold(true)));
    
    print_stdout(table_display).unwrap();
}

pub struct Data {
    records: Vec<HashMap<String, String>>,
    headers: HashMap<i32, String>
}