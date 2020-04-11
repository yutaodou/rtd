extern crate clap;
extern crate rtd;

use std::env;

use clap::{App, Arg, SubCommand};

use rtd::command::Add;
use rtd::command::Command;
use rtd::command::Done;
use rtd::command::List;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    let parser = App::new("Rust To Do")
        .version("0.1")
        .author("DYT. <yutaodou@gmail.com>")
        .about("Manage to-dos in command line")
        .subcommand(SubCommand::with_name("list")
            .about("Show tasks in all lists")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Show tasks from specified list only")
                .takes_value(true)
                .multiple(false))
            .arg(Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Show tasks in all lists including completed tasks")
                .conflicts_with("done")
                .takes_value(false))
            .arg(Arg::with_name("done")
                .short("d")
                .long("done")
                .conflicts_with("all")
                .help("Show completed tasks only from all lists")
                .takes_value(false)
            )
        );

    match args.first().unwrap().to_lowercase().as_str() {
        "add" => {
            Add::new(&args)?.run()?;
        }
        "list" => {
            let list_opts = parser.get_matches()
                .subcommand_matches("list")
                .unwrap()
                .to_owned();
            List::new(list_opts)?.run()?;
        }
        "done" => {
            Done::new(&args)?.run()?;
        }
        _ => panic!("wrong"),
    }
    Ok(())
}
