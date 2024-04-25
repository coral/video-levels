use std::fmt;

use crate::common::ProfileConstraint;
use yuv::color::ChromaSampling;
use yuv::color::Depth;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    Main,
    High,
    Professional,
}

impl From<&Profile> for ProfileConstraint {
    fn from(profile: &Profile) -> Self {
        match profile {
            Profile::Main => ProfileConstraint {
                max_bit_depth: Depth::Depth10,
                chroma_formats: vec![ChromaSampling::Monochrome, ChromaSampling::Cs420],
            },
            Profile::High => ProfileConstraint {
                max_bit_depth: Depth::Depth10,
                chroma_formats: vec![
                    ChromaSampling::Monochrome,
                    ChromaSampling::Cs420,
                    ChromaSampling::Cs444,
                ],
            },
            Profile::Professional => ProfileConstraint {
                max_bit_depth: Depth::Depth12,
                chroma_formats: vec![
                    ChromaSampling::Monochrome,
                    ChromaSampling::Cs420,
                    ChromaSampling::Cs422,
                    ChromaSampling::Cs444,
                ],
            },
        }
    }
}

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

pub struct LevelSpecification {
    id: Level,
    max_picture_size: u64,
    max_height: u32,
    max_width: u32,
    max_display_rate: u64,
    max_decode_rate: u64,
    max_header_rate: u32,
    max_bit_rate_main: u64,
    max_bit_rate_high: Option<u32>,
    min_comp_basis: u32,
    max_tiles: u32,
    max_tile_cols: u32,
}

impl LevelSpecification {
    pub fn id(&self) -> Level {
        self.id
    }
}

pub const LEVEL_DETAILS: [LevelSpecification; 14] = [
    LevelSpecification {
        id: Level::L2,
        max_picture_size: 147456,
        max_height: 1152,
        max_width: 2048,
        max_display_rate: 4423680,
        max_decode_rate: 5529600,
        max_header_rate: 150,
        max_bit_rate_main: 1_500_000,
        max_bit_rate_high: None,
        min_comp_basis: 2,
        max_tiles: 8,
        max_tile_cols: 4,
    },
    LevelSpecification {
        id: Level::L2_1,
        max_picture_size: 278784,
        max_height: 1584,
        max_width: 2816,
        max_display_rate: 8363520,
        max_decode_rate: 10454400,
        max_header_rate: 150,
        max_bit_rate_main: 3_000_000,
        max_bit_rate_high: None,
        min_comp_basis: 2,
        max_tiles: 8,
        max_tile_cols: 4,
    },
    LevelSpecification {
        id: Level::L3,
        max_picture_size: 665856,
        max_height: 2448,
        max_width: 4352,
        max_display_rate: 19975680,
        max_decode_rate: 24969600,
        max_header_rate: 150,
        max_bit_rate_main: 6_000_000,
        max_bit_rate_high: None,
        min_comp_basis: 2,
        max_tiles: 16,
        max_tile_cols: 6,
    },
    LevelSpecification {
        id: Level::L3_1,
        max_picture_size: 1065024,
        max_height: 3096,
        max_width: 5504,
        max_display_rate: 31950720,
        max_decode_rate: 39938400,
        max_header_rate: 150,
        max_bit_rate_main: 10_000_000,
        max_bit_rate_high: None,
        min_comp_basis: 2,
        max_tiles: 16,
        max_tile_cols: 6,
    },
    LevelSpecification {
        id: Level::L4,
        max_picture_size: 2359296,
        max_height: 3456,
        max_width: 6144,
        max_display_rate: 70778880,
        max_decode_rate: 77856768,
        max_header_rate: 300,
        max_bit_rate_main: 12_000_000,
        max_bit_rate_high: Some(30_000_000),
        min_comp_basis: 4,
        max_tiles: 32,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L4_1,
        max_picture_size: 2359296,
        max_height: 3456,
        max_width: 6144,
        max_display_rate: 141557760,
        max_decode_rate: 155713536,
        max_header_rate: 300,
        max_bit_rate_main: 20_000_000,
        max_bit_rate_high: Some(50_000_000),
        min_comp_basis: 4,
        max_tiles: 32,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L5,
        max_picture_size: 8912896,
        max_height: 4352,
        max_width: 8192,
        max_display_rate: 267386880,
        max_decode_rate: 273715200,
        max_header_rate: 300,
        max_bit_rate_main: 30_000_000,
        max_bit_rate_high: Some(100_000_000),
        min_comp_basis: 6,
        max_tiles: 64,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L5_1,
        max_picture_size: 8912896,
        max_height: 4352,
        max_width: 8192,
        max_display_rate: 534773760,
        max_decode_rate: 547430400,
        max_header_rate: 300,
        max_bit_rate_main: 40_000_000,
        max_bit_rate_high: Some(160_000_000),
        min_comp_basis: 8,
        max_tiles: 64,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L5_2,
        max_picture_size: 8912896,
        max_height: 4352,
        max_width: 8192,
        max_display_rate: 1069547520,
        max_decode_rate: 1094860800,
        max_header_rate: 300,
        max_bit_rate_main: 60_000_000,
        max_bit_rate_high: Some(240_000_000),
        min_comp_basis: 8,
        max_tiles: 64,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L5_3,
        max_picture_size: 8912896,
        max_height: 4352,
        max_width: 8192,
        max_display_rate: 1069547520,
        max_decode_rate: 1176502272,
        max_header_rate: 300,
        max_bit_rate_main: 60_000_000,
        max_bit_rate_high: Some(240_000_000),
        min_comp_basis: 8,
        max_tiles: 64,
        max_tile_cols: 8,
    },
    LevelSpecification {
        id: Level::L6,
        max_picture_size: 35651584,
        max_height: 8704,
        max_width: 16384,
        max_display_rate: 1069547520,
        max_decode_rate: 1176502272,
        max_header_rate: 300,
        max_bit_rate_main: 60_000_000,
        max_bit_rate_high: Some(240_000_000),
        min_comp_basis: 8,
        max_tiles: 128,
        max_tile_cols: 16,
    },
    LevelSpecification {
        id: Level::L6_1,
        max_picture_size: 35651584,
        max_height: 8704,
        max_width: 16384,
        max_display_rate: 2139095040,
        max_decode_rate: 2189721600,
        max_header_rate: 300,
        max_bit_rate_main: 100_000_000,
        max_bit_rate_high: Some(480_000_000),
        min_comp_basis: 8,
        max_tiles: 128,
        max_tile_cols: 16,
    },
    LevelSpecification {
        id: Level::L6_2,
        max_picture_size: 35651584,
        max_height: 8704,
        max_width: 16384,
        max_display_rate: 4278190080,
        max_decode_rate: 4379443200,
        max_header_rate: 300,
        max_bit_rate_main: 160_000_000,
        max_bit_rate_high: Some(800_000_000),
        min_comp_basis: 8,
        max_tiles: 128,
        max_tile_cols: 16,
    },
    LevelSpecification {
        id: Level::L6_3,
        max_picture_size: 35651584,
        max_height: 8704,
        max_width: 16384,
        max_display_rate: 4278190080,
        max_decode_rate: 4706009088,
        max_header_rate: 300,
        max_bit_rate_main: 160_000_000,
        max_bit_rate_high: Some(800_000_000),
        min_comp_basis: 8,
        max_tiles: 128,
        max_tile_cols: 16,
    },
];
