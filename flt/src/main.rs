use std::convert::TryInto;
use std::io;
use std::time::SystemTime;
use std::process;

use rand::thread_rng;
use rand::seq::SliceRandom;
use clap::{Arg, App};
use terminal_size::{Width, terminal_size};
use colored::*;

const DEFAULT_WIDTH: u16 = 80;
const COLORS: [&str; 7] = ["red", "green", "green", "yellow", "blue", "magenta", "cyan"];

struct Config {
    separator: String,
    wait_time: u128,
}

fn get_config() -> Config {
    let matches = App::new("flt - Fede's Log Tool")
        .version("1.0")
        .author("Federico Tedin <federicotedin@gmail.com>")
        .about("Colorize and separate log output.")
        .arg(Arg::with_name("separator")
             .short("s")
             .long("separator")
             .default_value("-")
             .value_name("SEPARATOR")
             .help("Enable log lines separation by timestamp."))
        .arg(Arg::with_name("wait_time")
             .short("w")
             .long("wait-time")
             .default_value("1500")
             .value_name("TIME (MS)")
             .help("Time (ms) to wait before a new log separator is printed."))
        .get_matches();

    Config {
        separator: String::from(matches.value_of("separator").unwrap()),
        wait_time: matches.value_of("wait_time").unwrap().parse().unwrap_or(1000),
    }
}

fn generate_separator(config: &Config) -> String {
    let size = terminal_size();
    let width: u16;

    if let Some((Width(w), _)) = size {
        width = w;
    } else {
        width = DEFAULT_WIDTH;
    }

    if config.separator.len() == 1 {
        config.separator.repeat(width.try_into().unwrap())
    } else {
        config.separator.clone()
    }
}

fn main() {
    let config = get_config();
    let base_separator = generate_separator(&config);
    let mut input = String::new();
    let mut rng = thread_rng();

    loop {
        let last = SystemTime::now();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let elapsed = last.elapsed().unwrap().as_millis();

                if elapsed > config.wait_time {
                    let color = COLORS.choose(&mut rng).unwrap();

                    let prefix = elapsed.to_string() + "ms ";
                    let separator = prefix.clone() + &String::from(&base_separator[prefix.len()..base_separator.len()]);

                    println!("{}", separator.color(*color));
                }

                print!("{}", input);

                input.clear();
            }
            Err(_) => process::exit(1),
        }
    }
}
