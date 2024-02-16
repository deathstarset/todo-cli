mod utils;
#[macro_use]
extern crate prettytable;
use prettytable::Table;
use std::{collections::HashMap, error::Error};
use utils::*;

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Todo {
    pub fn new() -> Self {
        // Something here for error handling
        let title = read_line("Enter Todo's title > ", &String::new()).unwrap();
        let created_at = gen_date();
        let updated_at = gen_date();
        Todo {
            title,
            created_at,
            updated_at,
        }
    }
}

// CRUD

pub fn add_todo(todos: &mut HashMap<String, Todo>, todo: Todo) {
    todos.insert(gen_random(), todo);
    println!("Todo Added");
}

pub fn delete_todo(todos: &mut HashMap<String, Todo>) -> Result<(), Box<dyn Error>> {
    let id = read_line("Enter Todo's id > ", &String::new())?;
    match todos.get(&id) {
        Some(_) => {
            todos.remove(&id);
        }
        None => return Err(String::from("Todo Not Found").into()),
    };
    println!("Todo Deleted");
    Ok(())
}

pub fn edit_todo(todos: &mut HashMap<String, Todo>) -> Result<(), Box<dyn Error>> {
    let id = read_line("Enter Todo's id > ", &String::new())?;
    let todo = match todos.get(&id) {
        Some(todo) => todo,
        None => return Err(String::from("Todo Not Found").into()),
    };
    let new_title = read_line("Todo's title > ", &todo.title)?;
    if let Some(todo) = todos.get_mut(&id) {
        todo.title = new_title;
        todo.updated_at = gen_date();
    };
    println!("Todo Edited");
    Ok(())
}

pub fn display_todo(todos: &HashMap<String, Todo>) -> Result<(), Box<dyn Error>> {
    if todos.len() == 0 {
        return Err(String::from("No Todos Found").into());
    }
    let mut table = Table::new();
    table.add_row(row!["Id", "Title", "Created At", "Updated At"]);
    for (id, todo) in todos {
        table.add_row(row![id, todo.title, todo.created_at, todo.updated_at]);
    }
    table.printstd();
    Ok(())
}
