pub use inventory;
use std::io::{self};

pub trait Solution: Send + Sync {
    fn run_part1(&self, input: &str) -> io::Result<String>;
    fn run_part2(&self, input: &str) -> io::Result<String>;
}

pub struct DayEntry {
    pub day: u32,
    pub solution: &'static (dyn Solution + 'static),
}

#[macro_export]
macro_rules! register_day {
    ($day:expr,$type:ident) => {
        $crate::inventory::submit! {$crate::DayEntry {
                day: $day,
                solution: &$type,
            }
        }
    };
}

inventory::collect!(DayEntry);
