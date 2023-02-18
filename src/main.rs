use colored::Colorize;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use npm_package_json::Package;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::io::{self, Write};

fn main() {
    let mut new_package = Package::new();

    let items = vec!["Javascript".yellow(), "Typescript".blue()];

    println!("Enter project name:");

    io::stdin()
        .read_line(&mut new_package.name)
        .expect("Enter a valid name");

    println!("Select language:");

    let selection = Select::with_theme(&ColorfulTheme::default())
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

    file.write_all(new_package.as_bytes())
        .expect("write failed");
    println!("data written to file");
}
