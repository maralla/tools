#[macro_use]
extern crate clap;
#[macro_use]
extern crate common;

use clap::{App, Arg, ArgMatches};
use std::i32;

fn parse_cli() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("A hex/ascii/bin to decimal convertion utility.")
        .arg(
            Arg::with_name("value")
                .help("The hex/ascii/bin value")
                .required(true),
        )
        .get_matches()
}

fn parse_int(data: &str, radix: u32) {
    match i32::from_str_radix(data, radix) {
        Ok(d) => println!("{}", d),
        Err(_) => die!("Fail to parse hex value `{}`", data),
    }
}

fn main() {
    let args = parse_cli();
    match args.value_of("value") {
        None => die!("{}", args.usage()),
        Some(s) => {
            if s.starts_with("0x") || s.starts_with("0X") {
                parse_int(&s[2..], 16)
            } else if s.starts_with("0b") || s.starts_with("0B") {
                parse_int(&s[2..], 2)
            } else {
                let res = s.as_bytes()
                    .into_iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", res);
            };
        }
    }
}
