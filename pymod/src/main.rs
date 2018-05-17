#[macro_use]
extern crate clap;
#[macro_use(die)]
extern crate common;

use clap::{App, Arg, ArgMatches};
use std::env;
use std::process::Command;

fn parse_cli() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("A tool to ease python module management.")
        .arg(
            Arg::with_name("mod")
                .help("The python module name")
                .required(true),
        )
        .get_matches()
}

fn echo_python_module_path(name: &str) {
    let python = env::var("PYTHON");
    let cmd = match python {
        Ok(ref s) => s,
        Err(_) => "python",
    };
    Command::new(cmd)
        .arg("-c")
        .arg(format!("import {name};print({name}.__file__)", name = name))
        .spawn()
        .expect("Fail to spawn python executable.")
        .wait()
        .expect("Fail to run python executable.");
}

fn main() {
    let args = parse_cli();
    match args.value_of("mod") {
        None => die!("{}", args.usage()),
        Some(s) => echo_python_module_path(s),
    }
}
