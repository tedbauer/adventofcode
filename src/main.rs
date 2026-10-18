use std::io::{self};
mod days;
use humanwritten::{Solution, DayEntry};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "Advent of code 2019")]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    solution_number: u8,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let entry = {
        let mut found = None;
        for e in inventory::iter::<DayEntry> {
            if e.day == args.solution_number.into() {
                found = Some(e);
                break;
            }
        }

        found.unwrap_or_else(|| {
            panic!(
                "No solution found for {0}, add one with `cargo run --bin scaffold {0}`.",
                args.solution_number
            )
        })
    };

    let f1 = fs::read_to_string(format!("input/day{:02}.txt", args.solution_number))?;
    let s1 = entry.solution.run_part1(&f1)?;
    println!("Day {0}, part 1 solution: {1}", args.solution_number, s1);

    let f2 = fs::read_to_string(format!("input/day{:02}.txt", args.solution_number))?;
    let s2 = entry.solution.run_part2(&f2)?;
    println!("Day {0}, part 2 solution: {1}", args.solution_number, s2);

    Ok(())
}
