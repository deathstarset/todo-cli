use dialoguer::{theme::ColorfulTheme, Select};
use std::collections::HashMap;
use todo_app::*;

fn main() {
    let mut todos: HashMap<String, Todo> = HashMap::new();

    let options = vec![
        "Add Todo",
        "Edit Todo",
        "Delete Todo",
        "Display Todos List",
        "Exit",
    ];

    loop {
        println!();
        // Error handling here also
        let option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an operation")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();
        println!();

        match option {
            0 => {
                add_todo(&mut todos, Todo::new());
            }
            1 => match edit_todo(&mut todos) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
            2 => match delete_todo(&mut todos) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
            3 => match display_todo(&todos) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            },
            4 => break,
            _ => (),
        }
    }
}
