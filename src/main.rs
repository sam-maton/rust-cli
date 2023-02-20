use colored::Colorize;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use serde::Serialize;
use std::io::{self, Write};

#[derive(Serialize)]
struct Package {
    name: String,
}

fn main() {
    let mut new_package = Package {
        name: String::new(),
    };

    let items = vec!["Javascript".yellow(), "Typescript".blue()];

    println!("Enter project name:");

    io::stdin()
        .read_line(&mut new_package.name)
        .expect("No name entered");

    new_package.name = new_package.name.trim().to_lowercase().replace(" ", "_");

    println!("Select language:");

    let _selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("Select an item");

    // match selection {
    //     Some(index) => user_selections.laguage = items[index].to_string(),
    //     None => println!("User did not select anything"),
    // }

    //Finish File creation

    let mut file = std::fs::File::create("package.json").expect("create failed");

    let j = serde_json::to_string(&new_package).expect("Serialize failed");

    file.write_all(j.as_bytes()).expect("write failed");
    println!("data written to file");
}
