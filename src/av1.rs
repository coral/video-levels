use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    L2,
    L2_1,
    L3,
    L3_1,
    L4,
    L4_1,
    L5,
    L5_1,
    L5_2,
    L5_3,
    L6,
    L6_1,
    L6_2,
    L6_3,
    Reserved,
    Max,
}

impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            0 => Level::L2,
            1 => Level::L2_1,
            4 => Level::L3,
            5 => Level::L3_1,
            8 => Level::L4,
            9 => Level::L4_1,
            12 => Level::L5,
            13 => Level::L5_1,
            14 => Level::L5_2,
            15 => Level::L5_3,
            16 => Level::L6,
            17 => Level::L6_1,
            18 => Level::L6_2,
            19 => Level::L6_3,
            24..=30 => Level::Reserved,
            32 => Level::Max,
            _ => Level::Max,
        }
    }
}

impl Level {
    fn usize(&self) -> usize {
        match self {
            Level::L2 => 0,
            Level::L2_1 => 1,
            Level::L3 => 4,
            Level::L3_1 => 5,
            Level::L4 => 8,
            Level::L4_1 => 9,
            Level::L5 => 12,
            Level::L5_1 => 13,
            Level::L5_2 => 14,
            Level::L5_3 => 15,
            Level::L6 => 16,
            Level::L6_1 => 17,
            Level::L6_2 => 18,
            Level::L6_3 => 19,
            Level::Reserved => 24,
            Level::Max => 32,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match self {
            Level::L2 => "L2",
            Level::L2_1 => "L2.1",
            Level::L3 => "L3",
            Level::L3_1 => "L3.1",
            Level::L4 => "L4",
            Level::L4_1 => "L4.1",
            Level::L5 => "L5",
            Level::L5_1 => "L5.1",
            Level::L5_2 => "L5.2",
            Level::L5_3 => "L5.3",
            Level::L6 => "L6",
            Level::L6_1 => "L6.1",
            Level::L6_2 => "L6.2",
            Level::L6_3 => "L6.3",
            Level::Reserved => "Reserved",
            Level::Max => "Max",
        };
        write!(f, "{}", level_str)
    }
}

pub struct LevelSpecification {}
