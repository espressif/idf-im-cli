extern crate idf_im_lib;
use clap::{arg, Command};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
use dialoguer::MultiSelect;

fn main() {
    // let _matches = Command::new("ESP-IDF Installation Manager")
    //     .version(VERSION)
    //     .about("All you need to manage your ESP-IDF installations")
    //     .arg(arg!(
    //         -n --name <NAME> "state your name"
    //     ))
    //     .get_matches();

    let items = vec![
        "check prerequisities",
        "get avalible idf versions",
        "run hello world in rustpython",
        "run idf install",
    ];

    let selection = MultiSelect::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    println!("You chose:");

    for i in selection {
        match i {
            0 => println!("{:?}", idf_im_lib::check_prerequisites()),
            1 => println!("{:?}", idf_im_lib::idf_versions::get_idf_versions()),
            2 => println!(
                "{:?}",
                idf_im_lib::run_some_python(r#"print("Hello World!")"#)
            ),
            3 => println!("{:?}", idf_im_lib::python_utils::run_idf_tools()),
            _ => panic!("Invalid selection"),
        }
    }

    // if let Some(name) = matches.get_one::<String>("name") {
    //     println!(
    //         "{}",
    //         idf_im_lib::greet(&matches.get_one::<String>("name").unwrap())
    //     )
    // } else {
    //     println!("No argument provided!")
    // }

    // println!(
    //     "{:?}",
    //     idf_im_lib::run_some_python(r#"print("Hello World!")"#)
    // );

    // let versions = idf_im_lib::idf_versions::get_idf_versions();
    // println!("{:?}", versions);

    // println!(
    //     "{:?}",
    //     idf_im_lib::idf_versions::get_idf_versions_by_target(&versions)
    // );

    // idf_im_lib::check_prerequisites();

    // let neco = idf_im_lib::python_utils::run_idf_tools();
    // println!("{:?}", neco);
}
