use std::fs::File;
use std::io::Write;
use std::{env, io};

const DAY_TEMPLATE: &str = include_str!("../templates/day.rs.template");
const MOD_USE_TEMPLATE: &str = "pub mod day{{PADDED_DAY}};\n";
const INPUT_PLACEHOLDER: &str = "Paste today's input here.";

fn render_template(day: u32) -> String {
    let day_padded = format!("{:02}", day);
    DAY_TEMPLATE
        .replace("{{DAY}}", &day.to_string())
        .replace("{{PADDED_DAY}}", &day_padded)
}

fn scaffold_day(day: u32) -> io::Result<()> {
    let day_padded = format!("{:02}", day);
    let day_path = format!("src/days/day{}.rs", &day_padded);

    if !(std::path::Path::new(&day_path).exists()) {
        let mut new_day_file = File::create(day_path)?;
        new_day_file.write_all(render_template(day).as_bytes())?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Day {} already exists", day),
        ));
    }

    let mut new_day_input_file = File::create(format!("input/day{}.txt", &day_padded))?;
    new_day_input_file.write_all(INPUT_PLACEHOLDER.as_bytes())?;

    let mut mod_file = File::options().append(true).open("src/days/mod.rs")?;
    let mod_entry = MOD_USE_TEMPLATE.replace("{{PADDED_DAY}}", &day_padded);
    mod_file.write_all(mod_entry.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let command = args.next();

    match command.as_deref() {
        Some("scaffold") => {
            let day_str = args
                .next()
                .expect("Expected day number: cargo xtask scaffold <day>");
            let day: u32 = day_str.parse().expect("Day must be an integer");

            if let Err(err) = scaffold_day(day) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            } else {
                println!(
                    "Day {0} created. Add input to `input/day{0:02}.txt`, and solution to `src/days/day{0:02}.rs`.",
                    day
                );
            }
        }
        _ => {
            eprintln!("Usage: cargo xtask scaffold <day>");
            std::process::exit(1);
        }
    }

    Ok(())
}
