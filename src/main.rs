#![allow(unused)]
use colored::Colorize;
use dialoguer::{console::Term, Select};
use serde::Serialize;
use serde_json::Value;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Serialize)]
struct Package {
    name: String,
}

fn main() {
    let mut name = String::new();

    println!("Enter project name:");

    io::stdin().read_line(&mut name).expect("No name entered");

    let mut template_path = create_file_path();

    println!("Templ path: {}", template_path);
}

fn create_file_path() -> String {
    let mut template = String::from("./templates/template-");

    let items = vec!["Javascript".yellow(), "Typescript".blue()];

    let languages = vec!["Angular".red(), "React".blue()];

    println!("Select language:");

    let language = Select::new()
        .with_prompt("Which framework would you like?")
        .items(&languages)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("Expected an input");

    let javascript = Select::new()
        .with_prompt("Choose a 'script'")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("Expected an input");

    if language == Some(0) {
        template += "angular";
    } else {
        template += "react";
    }

    if javascript == Some(1) {
        template += "-ts";
    }

    template
}

// fn create_package_json() {
//     let package_json = fs::read_to_string("src/templates/template-react/package.json")
//         .expect("LogRocket: error reading file");

//     let mut file = std::fs::File::create("package.json").expect("create failed");

//     file.write_all(package_json.as_bytes())
//         .expect("write failed");
//     println!("data written to file");
// }
