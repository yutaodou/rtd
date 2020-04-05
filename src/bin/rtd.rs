extern crate rtd;

use std::env;

use rtd::command::Command;
use rtd::command::Add;
use rtd::storage;
use rtd::task::Task;

fn show(todos: &Vec<Task>) {
    for todo in todos.iter(){
        println!("{}. {}", todo.id, todo.title);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = Add::new(&args).unwrap();
    cmd.exec().unwrap();

    let todos = storage::get_all().unwrap();
    show(&todos);
}

