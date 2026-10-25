use humanwritten::{Solution, register_day};
use std::io::{self};

pub struct Day02;

#[derive(Debug, Clone)]
enum Opcode {
    Add,
    Multiply,
    Done,
}

fn read_opcode(val: usize) -> io::Result<Opcode> {
    match val {
        1 => Ok(Opcode::Add),
        2 => Ok(Opcode::Multiply),
        99 => Ok(Opcode::Done),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "expected opcode, got number not in {1,2,99}",
        )),
    }
}

fn parse(prog_string: &str) -> io::Result<Vec<usize>> {
    prog_string
        .trim()
        .split(",")
        .map(|s| {
            s.parse::<usize>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<Result<Vec<usize>, io::Error>>()
}

fn eval_prog(prog: &mut Vec<usize>) -> io::Result<()> {
    let mut idx = 0;
    loop {
        let opcode_int = prog
            .get(idx)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no opcode"))?;
        let opcode = read_opcode(*opcode_int)?;

        if let Opcode::Done = opcode {
            break;
        }

        let arg_1_pos = *prog
            .get(idx + 1)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))?;
        let arg_1 = *prog
            .get(arg_1_pos)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))?;
        let arg_2_pos = *prog
            .get(idx + 2)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))?;
        let arg_2 = *prog
            .get(arg_2_pos)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))?;
        let target = *prog
            .get(idx + 3)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))?;

        match opcode {
            Opcode::Add => {
                if let Some(t) = prog.get_mut(target) {
                    *t = arg_1 + arg_2;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid target",
                    ));
                }
            }
            Opcode::Multiply => {
                if let Some(t) = prog.get_mut(target) {
                    *t = arg_1 * arg_2;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid target",
                    ));
                }
            }
            Opcode::Done => panic!("impossible; handled done case earlier"),
        }

        idx += 4;
    }

    Ok(())
}

fn eval_prog_with_inputs(prog: &mut Vec<usize>, noun: usize, verb: usize) -> io::Result<usize> {
    if let Some(t) = prog.get_mut(1) {
        *t = noun;
    }

    if let Some(t) = prog.get_mut(2) {
        *t = verb;
    }

    eval_prog(prog);

    prog.first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))
        .copied()
}

impl Solution for Day02 {
    fn run_part1(&self, input: &str) -> io::Result<String> {
        let mut prog = parse(input)?;

        // Replace pos 1 with 12, and 2 with 2.
        if let Some(t) = prog.get_mut(1) {
            *t = 12;
        }

        if let Some(t) = prog.get_mut(2) {
            *t = 2;
        }

        eval_prog(&mut prog);

        prog.first()
            .map(|u| u.to_string())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "missing arg"))
    }

    fn run_part2(&self, input: &str) -> io::Result<String> {
        let target = 19690720;

        for noun in 0..100 {
            for verb in 0..100 {
                let mut prog = parse(input)?;
                let result = eval_prog_with_inputs(&mut prog, noun, verb)?;

                if result == target {
                    let answer = 100 * noun + verb;
                    return Ok(format!("noun: {noun}. verb: {verb}. answer: {answer}"));
                }
            }
        }
        Ok("none found".to_string())
    }
}

register_day!(2, Day02);
