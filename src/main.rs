use colored::Colorize;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::io::{self, Write};

struct UserSelections {
    project_name: String,
    laguage: String,
}

fn main() {
    let mut user_selections = UserSelections {
        project_name: String::new(),
        laguage: String::new(),
    };

    let items = vec!["Javascript".yellow(), "Typescript".blue()];

    //Finish File creation

    // let mut file = std::fs::File::create("data.txt").expect("create failed");

    // file.write_all("Hello World".as_bytes())
    //     .expect("write failed");
    // file.write_all("\nTutorialsPoint".as_bytes())
    //     .expect("write failed");
    // println!("data written to file");

    println!("Enter project name:");

    io::stdin()
        .read_line(&mut user_selections.project_name)
        .expect("Enter a valid name");

    println!("Select language:");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("Select an item");

    match selection {
        Some(index) => user_selections.laguage = items[index].to_string(),
        None => println!("User did not select anything"),
    }

    println!(
        "{}, {}",
        user_selections.project_name, user_selections.laguage
    );
}
