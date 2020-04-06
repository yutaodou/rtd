extern crate rtd;

use std::env;

use rtd::command::Add;
use rtd::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = Add::new(&args).unwrap();
    cmd.exec().unwrap();
}
