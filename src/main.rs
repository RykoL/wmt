extern crate wmt;
use clap::{App, Arg, ArgMatches};
use rusqlite::Connection;
use wmt::errors::display_error;
use wmt::errors::{Error, Result};

fn open_db() -> Result<Connection> {
    Connection::open("wmt.db3").map_err(|_| Error::DBOpenError)
}

fn main() -> std::result::Result<(), wmt::errors::Error> {
    let matches = App::new("wmt")
        .version("0.0.1")
        .author("Rico Lang")
        .about("Time tracking tool to record things to waste time on")
        .subcommand(build_start_task_application())
        .subcommand(build_stop_task_application())
        .get_matches();

    match matches.subcommand() {
        ("start", Some(sub_match)) => handle_start_task(sub_match),
        ("stop", Some(sub_match)) => Ok(()),
        (&_, _) => Ok(()),
    }?;
    Ok(())
}

fn handle_start_task(args: &ArgMatches) -> Result<()> {
    open_db()
        .and_then(|conn| {
            wmt::task::start_task(
                &conn,
                args.value_of("project").unwrap().to_string(),
                args.value_of("description").unwrap().to_string(),
            )
        })
        .map(|task| {
            println!(
                "Started task '{}' on project '{}'.",
                task.description, task.tech_debt
            )
        })?;

    Ok(())
}

fn build_start_task_application<'a, 'b>() -> App<'a, 'b> {
    App::new("start")
        .about("Starts a new task")
        .arg(
            Arg::with_name("project")
                .help("Project or technical debt to track on")
                .required(true),
        )
        .arg(
            Arg::with_name("description")
                .help("Description of the task to be done")
                .required(true),
        )
}

fn build_stop_task_application<'a, 'b>() -> App<'a, 'b> {
    App::new("stop").about("Stops the current task")
}
