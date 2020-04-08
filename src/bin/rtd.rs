extern crate rtd;

use std::env;

use rtd::command::Add;
use rtd::command::Command;
use rtd::command::Done;
use rtd::command::List;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().unwrap().as_str() {
        "add" => {
            Add::new(&args).unwrap().exec().unwrap();
        }
        "list" => {
            List::new().unwrap().exec().unwrap();
        }
        "done" => {
            Done::new(&args).unwrap().exec().unwrap();
        }
        _ => panic!("wrong"),
    }
}
