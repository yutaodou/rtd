extern crate rtd;

use std::env;

use rtd::command::Add;
use rtd::command::List;
use rtd::command::Done;
use rtd::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.get(1).unwrap().as_str() {
        "add" => {
            Add::new(&args).unwrap().exec().unwrap();
        },
        "list" => {
            List::new().unwrap().exec().unwrap();
        }
        "done" => {
            Done::new(&args).unwrap().exec().unwrap();
        }
        _ => panic!("wrong")
    }
}
