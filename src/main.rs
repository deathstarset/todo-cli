use dialoguer::{theme::ColorfulTheme, Select};
use rustyline::DefaultEditor;
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
        let option = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an operation")
            .default(0)
            .items(&options)
            .interact()
            .unwrap();

        match option {
            0 => {
                add_todo(&mut todos, Todo::new());
            }
            1 => {
                println!();
                let mut rl = DefaultEditor::new().unwrap();
                let id = rl
                    .readline_with_initial("Enter Todo's id > ", (&String::new(), ""))
                    .unwrap();
                edit_todo(id, &mut todos);
            }
            2 => {
                println!();
                let mut rl = DefaultEditor::new().unwrap();
                let id = rl
                    .readline_with_initial("Enter Todo's id > ", (&String::new(), ""))
                    .unwrap();
                delete_todo(id, &mut todos)
            }
            3 => {
                display_todo(&todos);
            }
            4 => break,
            _ => (),
        }
    }
}
