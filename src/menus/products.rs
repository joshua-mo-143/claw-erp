use crate::database::{
    create_record, retrieve_data, view_all_records, view_one_record, RecordType,
};
use crate::utils::{invalid_input, read_input};
use std::error::Error;

pub async fn products_menu() -> Result<(), Box<dyn Error>> {
    println!("-PRODUCTS MENU-");
    println!("Type `help` for a list of commands you can run, or type `home` to go back to the main menu.");

    let data = retrieve_data("SELECT * from product").await?;

    loop {
        let input = read_input();

        let mut input = input.split(' ');

        let cmd = input.next().unwrap();

        match cmd.trim() {
            "help" => {
                println!("`create` -- Create a product.");
                println!("`view` -- View all products.");
                println!("`home` -- Go back to the main menu.");
            }
            "new" => create_record(RecordType::Product).await?,
            "view" => match input.next() {
                Some(x) => match x.trim().parse::<i32>() {
                    Ok(num) => {
                        view_one_record(data.clone(), num);
                        continue;
                    }
                    Err(err) => {
                        if x.trim() == "all" {
                            view_all_records(data.clone());
                        } else {
                            println!("Error: {}", err);
                            println!("Usage: `view <ITEM_ID_HERE>`");
                            continue;
                        }
                    }
                },
                _ => {
                    view_all_records(data.clone());
                    continue;
                }
            },
            "home" => {
                println!("-HOME MENU-");
                break;
            }
            _ => {
                invalid_input();
                continue;
            }
        }
    }

    Ok(())
}
