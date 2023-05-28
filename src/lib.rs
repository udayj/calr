use clap::{App, Arg};
use std::error::Error;
use chrono::{NaiveDate, Local, Datelike};

#[derive(Debug)]
pub struct Config {

    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {

    let matches = App::new("calr")
                        .version("0.1.0")
                        .author("udayj")
                        .about("Rust cal")
                        .arg(
                            Arg::with_name("month")
                                .short("m")
                                .long("month")
                                .value_name("MONTH")
                                .help("Month to display")
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("year")
                                .short("y")
                                .long("year")
                                .value_name("YEAR")
                                .help("Year to display")
                                .takes_value(true)
                        )
                        .get_matches();
    Ok(Config {
        month: matches.value_of("month").map(|m| m.parse().unwrap()),
        year: matches.value_of("year").map(|y| y.parse().unwrap()).unwrap_or(Local::now().date_naive().year()),
        today: Local::now().date_naive(),
    })


}

pub fn run(config: Config) ->MyResult<()> {

    println!("{:?}", config);
    Ok(())
}