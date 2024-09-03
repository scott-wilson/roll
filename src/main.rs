use std::{fmt::Display, sync::LazyLock};

use clap::{CommandFactory, Parser};
use rand::prelude::*;
use regex::Regex;
use unicode_width::UnicodeWidthStr;

static DICE: &str = "🎲";
static CRIT_MAX: &str = "😀";
static CRIT_MIN: &str = "😰";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    dice: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
struct Dice {
    pub count: usize,
    pub sides: usize,
}

impl TryFrom<&str> for Dice {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)d(\d+)").unwrap());

        let result = RE.captures(value).ok_or(Error)?;
        let count = result
            .get(1)
            .ok_or(Error)?
            .as_str()
            .parse::<usize>()
            .map_err(|_| Error)?;
        let sides = result
            .get(2)
            .ok_or(Error)?
            .as_str()
            .parse::<usize>()
            .map_err(|_| Error)?;

        Ok(Self { count, sides })
    }
}

impl TryFrom<String> for Dice {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}

#[derive(Debug)]
struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing dice")
    }
}

impl std::error::Error for Error {}

fn main() {
    let args = Args::parse();
    let mut dice = Vec::with_capacity(args.dice.len());
    let mut rng = rand::thread_rng();
    let mut max_size = 0;

    for d in args.dice {
        let d = match Dice::try_from(d) {
            Ok(d) => d,
            Err(err) => {
                let mut cmd = Args::command();
                cmd.error(clap::error::ErrorKind::InvalidValue, err).exit();
            }
        };
        let sides_width = d.sides.to_string().width();
        max_size = max_size.max(DICE.width() + 1 + d.to_string().width());
        max_size = max_size.max(sides_width + 1 + CRIT_MIN.width());
        max_size = max_size.max(sides_width + 1 + CRIT_MAX.width());
        max_size = max_size.max((d.count * d.sides).to_string().width());
        dice.push(d);
    }

    for d in dice {
        println!("{}", "=".repeat(max_size));
        println!(
            "{DICE}{:>max_size$}\n",
            d.to_string(),
            max_size = max_size - DICE.width()
        );
        let mut sum = 0;

        for _ in 0..d.count {
            let value = rng.gen_range(1..=d.sides);
            sum += value;

            if value == 1 {
                println!(
                    "{CRIT_MIN}{:>max_size$}",
                    value,
                    max_size = max_size - CRIT_MIN.width()
                );
            } else if value == d.sides {
                println!(
                    "{CRIT_MAX}{:>max_size$}",
                    value,
                    max_size = max_size - CRIT_MAX.width()
                );
            } else {
                println!("{:>max_size$}", value, max_size = max_size);
            }
        }

        println!("{}", "-".repeat(max_size));
        println!("{:>max_size$}", sum, max_size = max_size);
        println!("{}", "=".repeat(max_size));
    }
}
