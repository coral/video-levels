use std::fmt;

use crate::common::ProfileConstraint;
use yuv::color::ChromaSampling;
use yuv::color::Depth;

#[derive(Debug)]
pub struct LevelSelector {
    // Constraints
    width: u32,
    height: u32,
    framerate: f32,
    tier: Tier,
    profile: Profile,
    min_level: Option<Level>,
    max_level: Option<Level>,
    max_bitrate: Option<u32>,
}

impl LevelSelector {
    pub fn new() -> Self {
        Self {
            // Define default behaviour if no constraints are set
            width: 1920,
            height: 1080,
            framerate: 30.0,
            tier: Tier::Main,
            profile: Profile::Main,
            // Ignore if not set
            min_level: None,
            max_level: None,
            max_bitrate: None,
        }
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn framerate(mut self, framerate: f32) -> Self {
        self.framerate = framerate;
        self
    }
    pub fn clamp(mut self, min: Level, max: Level) -> Self {
        self.min_level = Some(min);
        self.max_level = Some(max);
        self
    }
    pub fn max_bitrate(mut self, max_bitrate: u32) -> Self {
        self.max_bitrate = Some(max_bitrate);
        self
    }
    pub fn tier(mut self, tier: Tier) -> Self {
        self.tier = tier;
        self
    }
    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
    }

    pub fn select(self) -> Option<LevelSpecification> {
        let samples = self.width * self.height;
        let display_rate = (samples as f64 * self.framerate as f64) as u64;

        for level in LEVEL_DETAILS.iter() {
            if samples as u64 <= level.max_picture_size()
                && display_rate <= level.max_display_rate()
                && self.width <= level.max_width()
                && self.height <= level.max_height()
                && self.framerate as u32 <= level.max_header_rate()
            {
                let selected = match (self.max_bitrate, level.max_bit_rate(self.tier)) {
                    (Some(bitrate_constraint), Some(level_max_bitrate))
                        if level_max_bitrate >= bitrate_constraint.into() =>
                    {
                        *level
                    }
                    (None, Some(_)) => *level,
                    _ => continue,
                };

                // Clamp to min level
                match self.min_level {
                    Some(min) if selected.id() < min => {
                        continue;
                    }
                    _ => {}
                }

                // Check if exceds max level
                match self.max_level {
                    Some(max) if selected.id() > max => return None,
                    _ => {}
                }

                return Some(selected);
            }
        }

        Some(LEVEL_DETAILS[LEVEL_DETAILS.len() - 1])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    Main,
    High,
    Professional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    Main,
    High,
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

/// get returns the level specification for the given level
pub fn get(level: Level) -> LevelSpecification {
    for l in LEVEL_DETAILS.iter() {
        if l.id() == level {
            return *l;
        }
    }
    LEVEL_DETAILS[LEVEL_DETAILS.len() - 1]
}

#[derive(Debug, Clone, Copy)]
pub struct LevelSpecification {
    id: Level,
    max_picture_size: u64,
    max_horizontal: u32,
    max_vertical: u32,
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
    pub fn max_picture_size(&self) -> u64 {
        self.max_picture_size
    }
    pub fn max_horizontal(&self) -> u32 {
        self.max_horizontal
    }
    pub fn max_vertical(&self) -> u32 {
        self.max_vertical
    }
    pub fn max_width(&self) -> u32 {
        self.max_horizontal
    }
    pub fn max_height(&self) -> u32 {
        self.max_vertical
    }
    pub fn max_display_rate(&self) -> u64 {
        self.max_display_rate
    }
    pub fn max_decode_rate(&self) -> u64 {
        self.max_decode_rate
    }
    pub fn max_header_rate(&self) -> u32 {
        self.max_header_rate
    }
    pub fn max_bit_rate(&self, tier: Tier) -> Option<u64> {
        match tier {
            Tier::Main => Some(self.max_bit_rate_main),
            Tier::High => self.max_bit_rate_high.map(|v| v as u64),
        }
    }
    pub fn max_bit_rate_main(&self) -> u64 {
        self.max_bit_rate_main
    }
    pub fn max_bit_rate_high(&self) -> Option<u32> {
        self.max_bit_rate_high
    }
    pub fn min_comp_basis(&self) -> u32 {
        self.min_comp_basis
    }
    pub fn max_tiles(&self) -> u32 {
        self.max_tiles
    }
    pub fn max_tile_cols(&self) -> u32 {
        self.max_tile_cols
    }
}

pub const LEVEL_DETAILS: [LevelSpecification; 14] = [
    LevelSpecification {
        id: Level::L2,
        max_picture_size: 147456,
        max_horizontal: 1152,
        max_vertical: 2048,
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
        max_horizontal: 1584,
        max_vertical: 2816,
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
        max_horizontal: 2448,
        max_vertical: 4352,
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
        max_horizontal: 3096,
        max_vertical: 5504,
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
        max_horizontal: 3456,
        max_vertical: 6144,
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
        max_horizontal: 3456,
        max_vertical: 6144,
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
        max_horizontal: 4352,
        max_vertical: 8192,
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
        max_horizontal: 4352,
        max_vertical: 8192,
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
        max_horizontal: 4352,
        max_vertical: 8192,
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
        max_horizontal: 4352,
        max_vertical: 8192,
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
        max_horizontal: 8704,
        max_vertical: 16384,
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
        max_horizontal: 8704,
        max_vertical: 16384,
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
        max_horizontal: 8704,
        max_vertical: 16384,
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
        max_horizontal: 8704,
        max_vertical: 16384,
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
#[cfg(test)]
mod tests {
    #[test]
    fn level_mult() {
        use crate::av1::Level;
        assert_eq!(Level::L3, Level::from(4));
    }

    #[test]
    fn max_bitrate() {
        use crate::av1::{self, Level, Tier};

        let l = av1::get(Level::L3);
        assert_eq!(l.max_bit_rate(Tier::Main), Some(6_000_000));
        assert_eq!(l.max_bit_rate(Tier::High), None);

        let l = av1::get(Level::L5_2);
        assert_eq!(l.max_bit_rate(Tier::Main), Some(60_000_000));
        assert_eq!(l.max_bit_rate(Tier::High), Some(240_000_000));
    }

    #[test]
    fn select_base_cases() {
        use crate::av1::{Level, LevelSelector, Profile, Tier};
        assert_eq!(
            LevelSelector::new()
                .width(1920)
                .height(1080)
                .framerate(30.0)
                .tier(Tier::Main)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L4
        );

        assert_eq!(
            LevelSelector::new()
                .width(4380)
                .height(1080)
                .framerate(30.0)
                .tier(Tier::Main)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L6
        );

        assert_eq!(
            LevelSelector::new()
                .width(3840)
                .height(2160)
                .framerate(200.0)
                .tier(Tier::Main)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L6_1
        );
    }
}
