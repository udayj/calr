use clap::{App, Arg};
use std::error::Error;
use chrono::{NaiveDate, Local, Datelike};
use std::str::FromStr;

#[derive(Debug)]
pub struct Config {

    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {

    // 
    let matches = App::new("calr")
                        .version("0.1.0")
                        .author("udayj")
                        .about("Rust cal")
                        .arg(
                            Arg::with_name("month")
                                .value_name("MONTH")
                                .short("m")
                                .help("Month to display")
                                .takes_value(true)
                        )
                        .arg(
                            Arg::with_name("show_current_year")
                                .value_name("SHOW_YEAR")
                                .short("y")
                                .long("year")
                                .conflicts_with_all(&["month", "year"])
                                .takes_value(false)
                        )
                        .arg(
                            Arg::with_name("year")
                                .value_name("YEAR")
                                .help("Year (1-9999)")
                        )
                        .get_matches();

    // let month = value of first positional argument
    let mut month = matches.value_of("month").map(parse_month).transpose()?;
    let mut year = matches.value_of("year").map(parse_year).transpose()?;
    let today = Local::today();
    if matches.is_present("show_current_year") {
        year = Some(today.year());
        month = None;
    }
    else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }
    
    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.naive_local()
    })


}

pub fn run(config: Config) ->MyResult<()> {

    println!("{:?}", config);
    Ok(())
}

fn parse_int<T: FromStr>(val: &str) -> MyResult<T> {

    val.parse().map_err(|_| format!("Could not parse '{}' as integer", val).into())

}

fn parse_year(year: &str) -> MyResult<i32> {

    let year = parse_int(year)?;
    if !(1..=9999).contains(&year) {
        return Err(From::from(format!("year {} not in the range 1 through 9999", year)));
    }
    Ok(year)
}

fn parse_month(month: &str) -> MyResult<u32> {

    let result = parse_int(month);
    let months = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    ];
    match result {

        Ok(num) => {

            if !(1..=12).contains(&num) {
                return Err(From::from(format!("{} is neither a month number (1..12) nor a name", num)));
            }
            Ok(num)
        },
        _ => {
            let matches: Vec<_> = months.iter().enumerate().filter_map(|(i, m)| 
            if m.to_lowercase().starts_with(month.to_lowercase().as_str()) {
                Some(i as u32 + 1)
            } else {
                None
            }
        ).collect();
        if matches.len() == 1 {
           Ok(matches[0] as u32)
        }
        else {
             Err(format!("Invalid month \"{}\"", month).into())
        }
    }
    }
}