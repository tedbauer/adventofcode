use humanwritten::{Solution, register_day};
use std::io::{self};

pub struct Day01;

fn read_masses(input: &str) -> io::Result<Vec<u32>> {
    let mut masses = Vec::new();
    for line in input.lines() {
        let line: u32 = line
            .trim()
            .parse::<u32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        masses.push(line);
    }

    Ok(masses)
}

// Compute recursive fuel.
fn compute_recursive_fuel(acc: u32, mass: u32) -> u32 {
    match (mass / 3).checked_sub(2) {
        Some(fuel) => {
            if fuel > 0 {
                fuel + compute_recursive_fuel(acc, fuel)
            } else {
                acc
            }
        }
        None => acc,
    }
}

impl Solution for Day01 {
    fn run_part1(&self, input: &str) -> io::Result<String> {
        let masses = read_masses(input)?;
        let total_fuel = masses.iter().map(|mass| mass / 3 - 2).sum::<u32>();
        Ok(total_fuel.to_string())
    }

    fn run_part2(&self, input: &str) -> io::Result<String> {
        let masses = read_masses(input)?;
        let total_fuel_recursive = masses
            .iter()
            .map(|mass| compute_recursive_fuel(0, *mass))
            .sum::<u32>();
        Ok(total_fuel_recursive.to_string())
    }
}

register_day!(1, Day01);
