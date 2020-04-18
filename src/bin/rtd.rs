extern crate clap;
extern crate rtd;

use std::process::exit;

use clap::{App, Arg, ArgMatches, SubCommand};

use rtd::command::Add;
use rtd::command::Command;
use rtd::command::Done;
use rtd::command::List;
use rtd::command::Today;

fn main() {
    let opts = App::new("Rust To Do")
        .version("v0.1")
        .about("Manage to-dos in command line")
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists tasks")
                .arg(
                    Arg::with_name("name")
                        .required(false)
                        .index(1)
                        .help("Show tasks from specified list")
                        .takes_value(true)
                        .multiple(false),
                )
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Show tasks in all lists including completed tasks")
                        .conflicts_with("done")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("done")
                        .short("d")
                        .long("done")
                        .conflicts_with("all")
                        .help("Show completed tasks only from all lists")
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("add").about("Add a new task").arg(
                Arg::with_name("INPUT")
                    .help("<todo-title> ~<list> !priority")
                    .required(true)
                    .index(1)
                    .takes_value(true)
                    .multiple(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("Mark task as completed/un-completed")
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("today")
                .about("Mark/un-mark task as today's priority")
                .arg(
                    Arg::with_name("INPUT")
                        .help("<mark-task-id> / ~<un-mark-task-id>")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .get_matches();

    match run(&opts) {
        Ok(_) => exit(0),
        Err(error) => {
            eprintln!("{}", error);
            eprintln!("{}", opts.usage());
            exit(1);
        }
    }
}

fn run(opts: &ArgMatches) -> Result<(), &'static str> {
    match opts.subcommand() {
        ("add", Some(add_opts)) => Add::new(add_opts).run(),
        ("list", Some(list_opts)) => List::new(list_opts).run(),
        ("done", Some(done_opts)) => Done::new(done_opts).run(),
        ("today", Some(today_opts)) => Today::new(today_opts).run(),
        _ => Err("Unsupported command."),
    }
}
