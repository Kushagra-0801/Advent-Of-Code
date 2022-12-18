use clap::{error::ErrorKind, value_parser, Arg, Args, Error, FromArgMatches, Parser, ValueEnum};
use date_time::date_tuple::Date;

#[derive(Debug)]
struct DayArgs {
    day: u8,
    year: u16,
}

impl FromArgMatches for DayArgs {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let today = current_date();
        let day = matches.get_one("day").copied();
        let year = matches.get_one("year").copied();
        let (day, year) = match (day, year, today.0) {
            (Some(d), Some(y), _) => (d, y),
            (Some(d), None, _) => (d, today.1),
            (None, None, Some(d)) => (d, today.1),
            _ => {
                return Err(Error::raw(
                    ErrorKind::MissingRequiredArgument,
                    "day needs to be provided or script should be called during AOC",
                ))
            }
        };
        Ok(Self { day, year })
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        let today = current_date();
        let day = matches.get_one("day").copied().or(today.0);
        let year = matches.get_one::<u16>("year").copied().unwrap_or(today.1);
        self.day = day.unwrap_or(self.day);
        self.year = year;
        Ok(())
    }
}

impl Args for DayArgs {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .help("Solution day. Default is the current day if AOC is running.")
                .value_parser(value_parser!(u8).range(1..=25)),
        )
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .help("Solution year. Default is the current year. Day must be provided if year is provided.")
                .value_parser(value_parser!(u16).range(2015..)),
        )
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd.arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .value_parser(value_parser!(u8).range(1..=25)),
        )
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .value_parser(value_parser!(u16).range(2015..)),
        )
    }
}

/// Create a new solution template in the requested language and download the corresponding input
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(flatten)]
    dayarg: DayArgs,
    /// AOC session cookie to download the solution input. Can be filled from the AOC_SESSION_KEY environment variable
    #[arg(short, long = "session", env = "AOC_SESSION_KEY", default_value_t = String::new())]
    #[arg(hide_default_value = true, hide_env = true)]
    session_key: String,
    /// Solution template to generate
    #[arg(short = 'l', long = "lang", default_value_t = SolutionLang::Rust, value_enum)]
    solution_lang: SolutionLang,
}

fn current_date() -> (Option<u8>, u16) {
    let today = Date::today();
    if today.get_month() == 12 && today.get_date() <= 25 {
        (Some(today.get_date()), today.get_year())
    } else {
        (None, today.get_year())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum SolutionLang {
    Rust,
    Python,
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
