#[macro_use]
extern crate prettytable;
use chrono::prelude::*;
use prettytable::Table;
use rand::{distributions::Alphanumeric, Rng};
use rustyline::DefaultEditor;
use std::{
    collections::HashMap,
    time::{Duration, Instant, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Todo {
    pub fn new() -> Self {
        let mut rl = DefaultEditor::new().unwrap();
        let mut title = rl
            .readline_with_initial("Enter Todo's title > ", (&String::new(), ""))
            .unwrap();
        title = title.trim_end().to_string();
        let created_at = gen_date();
        let updated_at = gen_date();
        Todo {
            title,
            created_at,
            updated_at,
        }
    }
}

// UTILS

pub fn gen_date() -> String {
    let instant = Instant::now();
    let instant_sec_epoch =
        instant.elapsed().as_secs() + UNIX_EPOCH.elapsed().expect("error in time").as_secs();
    let time_instant_created = UNIX_EPOCH + Duration::from_secs(instant_sec_epoch);
    let datetime: DateTime<Local> = time_instant_created.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn gen_random() -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(10)
        .map(|i| char::from(i))
        .collect()
}

// CRUD

pub fn add_todo(todos: &mut HashMap<String, Todo>, todo: Todo) {
    println!();
    todos.insert(gen_random(), todo);
    println!("Todo Added\n");
}

pub fn delete_todo(id: String, todos: &mut HashMap<String, Todo>) {
    match todos.get(&id) {
        Some(_) => {
            todos.remove(&id);
        }
        None => panic!("no todos found"),
    };
    println!("Todo Deleted\n");
}

pub fn edit_todo(id: String, todos: &mut HashMap<String, Todo>) {
    let todo = match todos.get(&id) {
        Some(todo) => todo,
        None => panic!("no todos found"),
    };
    let mut rl = DefaultEditor::new().unwrap();
    let new_title = rl
        .readline_with_initial("Todo's title > ", (&todo.title, ""))
        .unwrap();
    if let Some(todo) = todos.get_mut(&id) {
        todo.title = new_title;
        todo.updated_at = gen_date();
    };
    println!("Todo Edited\n");
}

pub fn display_todo(todos: &HashMap<String, Todo>) {
    if todos.len() == 0 {
        println!("\nNo Todos Found\n");
    } else {
        println!();
        let mut table = Table::new();
        table.add_row(row!["Id", "Title", "Created At", "Updated At"]);
        for (id, todo) in todos {
            table.add_row(row![id, todo.title, todo.created_at, todo.updated_at]);
        }
        table.printstd();
        println!();
    }
}
