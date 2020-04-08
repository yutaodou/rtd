extern crate rtd;

use std::env;

use rtd::command::Add;
use rtd::command::Command;
use rtd::command::Done;
use rtd::command::List;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().unwrap().to_lowercase().as_str() {
        "add" => {
            Add::new(&args)?.run()?;
        }
        "list" => {
            List::new()?.run()?;
        }
        "done" => {
            Done::new(&args)?.run()?;
        }
        _ => panic!("wrong"),
    }
    Ok(())
}
