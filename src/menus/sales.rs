use std::io;

use crate::utils::invalid_input;

pub fn sales_menu() {
    println!("-SALES MENU-");

    loop {
        let mut command = String::new();

        let stdin = io::stdin();

        stdin
            .read_line(&mut command)
            .expect("Had a problem reading your input :(");

        match command.trim() {
            "help" => println!("Hello world"),
            "home" => {
                println!("-MAIN MENU-");
                break;
            }
            _ => {
                invalid_input();
                continue;
            }
        }
    }
}
