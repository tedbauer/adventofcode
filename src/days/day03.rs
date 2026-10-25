use humanwritten::{Solution, register_day};
use std::collections::HashSet;
use std::io::{self, BufRead};

pub struct Day03;

// First, parse each wire.
//
// Lay out the coordinates that the wire crosses.
//
// Determine the set of coordinates that are in both wires.
//
// Pick the set of coordinates with min Manhattan distance.

#[derive(Debug)]
struct WireInstructions {
    wire_1_instrs: Vec<Instruction>,
    wire_2_instrs: Vec<Instruction>,
}

#[derive(Debug)]
struct WireCoordinates {
    wire_1_coords: Vec<(i32, i32)>,
    wire_2_coords: Vec<(i32, i32)>,
}

#[derive(Debug)]
enum Instruction {
    Left { distance: i32 },
    Right { distance: i32 },
    Down { distance: i32 },
    Up { distance: i32 },
}

fn manhattan_from_center(coords: (i32, i32)) -> i32 {
    coords.0.abs() + coords.1.abs()
}

fn find_common_coordinates(wire_coords: &WireCoordinates) -> HashSet<(i32, i32)> {
    let wire_1_hashset: HashSet<(i32, i32)> =
        wire_coords.wire_1_coords.clone().into_iter().collect();
    let wire_2_hashset: HashSet<(i32, i32)> =
        wire_coords.wire_2_coords.clone().into_iter().collect();

    wire_1_hashset
        .intersection(&wire_2_hashset)
        .copied()
        .collect()
}

fn layout_coordinates(instructions: &[Instruction]) -> Vec<(i32, i32)> {
    let mut curr_coords = (0, 0);

    let mut coordinates = Vec::new();

    for instr in instructions {
        match instr {
            Instruction::Left { distance } => {
                for x in 0..*distance {
                    curr_coords.0 -= 1;
                    coordinates.push(curr_coords);
                }
            }
            Instruction::Right { distance } => {
                for x in 0..*distance {
                    curr_coords.0 += 1;
                    coordinates.push(curr_coords);
                }
            }
            Instruction::Up { distance } => {
                for y in 0..*distance {
                    curr_coords.1 += 1;
                    coordinates.push(curr_coords);
                }
            }
            Instruction::Down { distance } => {
                for y in 0..*distance {
                    curr_coords.1 -= 1;
                    coordinates.push(curr_coords);
                }
            }
        }
    }

    coordinates
}

fn layout_all_coordinates(wire_instructions: &WireInstructions) -> WireCoordinates {
    WireCoordinates {
        wire_1_coords: layout_coordinates(&wire_instructions.wire_1_instrs),
        wire_2_coords: layout_coordinates(&wire_instructions.wire_2_instrs),
    }
}

fn parse_wire(wire: &str) -> io::Result<Vec<Instruction>> {
    wire.split(",")
        .map(|d| {
            let mut chars = d.chars();

            let dir = chars
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing dir"))?;
            let distance = chars
                .as_str()
                .trim()
                .parse::<i32>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            match dir {
                'L' => Ok(Instruction::Left { distance }),
                'R' => Ok(Instruction::Right { distance }),
                'U' => Ok(Instruction::Up { distance }),
                'D' => Ok(Instruction::Down { distance }),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "there aren't two lines to parse",
                )),
            }
        })
        .collect()
}

fn parse_wires(input: &str) -> io::Result<WireInstructions> {
    let mut wire_1_instrs: Vec<Instruction> = Vec::new();
    let mut wire_2_instrs: Vec<Instruction> = Vec::new();

    let mut lines = input.split("\n");

    if let (Some(w1), Some(w2)) = (lines.next(), lines.next()) {
        let wire_1_instrs = parse_wire(w1)?;
        let wire_2_instrs = parse_wire(w2)?;
        Ok(WireInstructions {
            wire_1_instrs,
            wire_2_instrs,
        })
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "there aren't two lines to parse",
        ))
    }
}

impl Solution for Day03 {
    fn run_part1(&self, input: &str) -> io::Result<String> {
        let wire_instructions = parse_wires(input)?;
        let wire_coordinates = layout_all_coordinates(&wire_instructions);
        let common_coordinates = find_common_coordinates(&wire_coordinates);
        let closest = common_coordinates
            .clone()
            .into_iter()
            .map(|c| manhattan_from_center(c))
            .min();
        closest
            .map(|c| format!("{:?}", c))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no result"))
    }

    fn run_part2(&self, input: &str) -> io::Result<String> {
        Ok("Not yet implemented".to_string())
    }
}

register_day!(3, Day03);
