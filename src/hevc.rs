use strum_macros::Display;

use crate::types::Dimension;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Level {
    L1,
    L2,
    L2_1,
    L3,
    L3_1,
    L4,
    L4_1,
    L5,
    L5_1,
    L5_2,
    L6,
    L6_1,
    L6_2,
}

impl Level {
    pub fn select(dims: Dimension, framerate: f32) -> LevelDetail {
        let sr = dims.width * dims.height * framerate.ceil() as u32;
        for level in LEVEL_DETAILS.iter() {
            if sr <= level.max_luma_sample_rate {
                return *level;
            }
        }

        return LEVEL_DETAILS[LEVEL_DETAILS.len() - 1];
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::L1 => write!(f, "1"),
            Level::L2 => write!(f, "2"),
            Level::L2_1 => write!(f, "2.1"),
            Level::L3 => write!(f, "3"),
            Level::L3_1 => write!(f, "3.1"),
            Level::L4 => write!(f, "4"),
            Level::L4_1 => write!(f, "4.1"),
            Level::L5 => write!(f, "5"),
            Level::L5_1 => write!(f, "5.1"),
            Level::L5_2 => write!(f, "5.2"),
            Level::L6 => write!(f, "6"),
            Level::L6_1 => write!(f, "6.1"),
            Level::L6_2 => write!(f, "6.2"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LevelDetail {
    pub id: Level,
    /// Samples (pixels) per second
    pub max_luma_sample_rate: u32,
    pub max_luma_picture_size: u32,
    pub max_bit_rate_main: (u32, u32),
    pub max_bit_rate_main_10: (u32, u32),
    pub max_bit_rate_main_12: (u32, u32),
    pub max_bit_rate_main_444_12: (u32, u32),
    pub max_bit_rate_main_444_16_intra: (u32, u32),
    pub max_bit_rate_high_throughput_444_16_intra: (u32, u32),
}

pub const LEVEL_DETAILS: [LevelDetail; 13] = [
    LevelDetail {
        id: Level::L1,
        max_luma_sample_rate: 552_960,
        max_luma_picture_size: 36_864,
        max_bit_rate_main: (128, 0),
        max_bit_rate_main_10: (192, 0),
        max_bit_rate_main_12: (384, 0),
        max_bit_rate_main_444_12: (1024, 0),
        max_bit_rate_main_444_16_intra: (12_288, 0),
        max_bit_rate_high_throughput_444_16_intra: (12_288, 0),
    },
    LevelDetail {
        id: Level::L2,
        max_luma_sample_rate: 3_686_400,
        max_luma_picture_size: 122_880,
        max_bit_rate_main: (1_500, 0),
        max_bit_rate_main_10: (2_250, 0),
        max_bit_rate_main_12: (4_500, 0),
        max_bit_rate_main_444_12: (12_000, 0),
        max_bit_rate_main_444_16_intra: (144_000, 0),
        max_bit_rate_high_throughput_444_16_intra: (144_000, 0),
    },
    LevelDetail {
        id: Level::L2_1,
        max_luma_sample_rate: 7_372_800,
        max_luma_picture_size: 245_760,
        max_bit_rate_main: (3_000, 0),
        max_bit_rate_main_10: (4_500, 0),
        max_bit_rate_main_12: (9_000, 0),
        max_bit_rate_main_444_12: (24_000, 0),
        max_bit_rate_main_444_16_intra: (288_000, 0),
        max_bit_rate_high_throughput_444_16_intra: (288_000, 0),
    },
    LevelDetail {
        id: Level::L3,
        max_luma_sample_rate: 16_588_800,
        max_luma_picture_size: 552_960,
        max_bit_rate_main: (6_000, 0),
        max_bit_rate_main_10: (9_000, 0),
        max_bit_rate_main_12: (18_000, 0),
        max_bit_rate_main_444_12: (48_000, 0),
        max_bit_rate_main_444_16_intra: (576_000, 0),
        max_bit_rate_high_throughput_444_16_intra: (576_000, 0),
    },
    LevelDetail {
        id: Level::L3_1,
        max_luma_sample_rate: 33_177_600,
        max_luma_picture_size: 983_040,
        max_bit_rate_main: (10_000, 0),
        max_bit_rate_main_10: (15_000, 0),
        max_bit_rate_main_12: (30_000, 0),
        max_bit_rate_main_444_12: (80_000, 0),
        max_bit_rate_main_444_16_intra: (960_000, 0),
        max_bit_rate_high_throughput_444_16_intra: (960_000, 0),
    },
    LevelDetail {
        id: Level::L4,
        max_luma_sample_rate: 66_846_720,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: (12_000, 30_000),
        max_bit_rate_main_10: (18_000, 45_000),
        max_bit_rate_main_12: (36_000, 90_000),
        max_bit_rate_main_444_12: (96_000, 240_000),
        max_bit_rate_main_444_16_intra: (1_152_000, 2_880_000),
        max_bit_rate_high_throughput_444_16_intra: (1_152_000, 2_880_000),
    },
    LevelDetail {
        id: Level::L4_1,
        max_luma_sample_rate: 133_693_440,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: (20_000, 50_000),
        max_bit_rate_main_10: (30_000, 75_000),
        max_bit_rate_main_12: (60_000, 150_000),
        max_bit_rate_main_444_12: (160_000, 400_000),
        max_bit_rate_main_444_16_intra: (1_920_000, 4_800_000),
        max_bit_rate_high_throughput_444_16_intra: (1_920_000, 4_800_000),
    },
    LevelDetail {
        id: Level::L5,
        max_luma_sample_rate: 267_386_880,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: (25_000, 100_000),
        max_bit_rate_main_10: (37_500, 150_000),
        max_bit_rate_main_12: (75_000, 300_000),
        max_bit_rate_main_444_12: (200_000, 800_000),
        max_bit_rate_main_444_16_intra: (2_400_000, 9_600_000),
        max_bit_rate_high_throughput_444_16_intra: (2_400_000, 9_600_000),
    },
    LevelDetail {
        id: Level::L5_1,
        max_luma_sample_rate: 534_773_760,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: (40_000, 160_000),
        max_bit_rate_main_10: (60_000, 240_000),
        max_bit_rate_main_12: (120_000, 480_000),
        max_bit_rate_main_444_12: (320_000, 1_280_000),
        max_bit_rate_main_444_16_intra: (3_840_000, 15_360_000),
        max_bit_rate_high_throughput_444_16_intra: (3_840_000, 15_360_000),
    },
    LevelDetail {
        id: Level::L5_2,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: (60_000, 240_000),
        max_bit_rate_main_10: (90_000, 360_000),
        max_bit_rate_main_12: (180_000, 720_000),
        max_bit_rate_main_444_12: (480_000, 1_920_000),
        max_bit_rate_main_444_16_intra: (5_760_000, 23_040_000),
        max_bit_rate_high_throughput_444_16_intra: (5_760_000, 23_040_000),
    },
    LevelDetail {
        id: Level::L6,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: (60_000, 240_000),
        max_bit_rate_main_10: (90_000, 360_000),
        max_bit_rate_main_12: (180_000, 720_000),
        max_bit_rate_main_444_12: (480_000, 1_920_000),
        max_bit_rate_main_444_16_intra: (5_760_000, 23_040_000),
        max_bit_rate_high_throughput_444_16_intra: (5_760_000, 23_040_000),
    },
    LevelDetail {
        id: Level::L6_1,
        max_luma_sample_rate: 2_139_095_040,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: (120_000, 480_000),
        max_bit_rate_main_10: (180_000, 720_000),
        max_bit_rate_main_12: (360_000, 1_440_000),
        max_bit_rate_main_444_12: (960_000, 3_840_000),
        max_bit_rate_main_444_16_intra: (11_520_000, 46_080_000),
        max_bit_rate_high_throughput_444_16_intra: (11_520_000, 46_080_000),
    },
    LevelDetail {
        id: Level::L6_2,
        max_luma_sample_rate: 4_278_190_080,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: (240_000, 800_000),
        max_bit_rate_main_10: (360_000, 1_200_000),
        max_bit_rate_main_12: (720_000, 2_400_000),
        max_bit_rate_main_444_12: (1_920_000, 6_400_000),
        max_bit_rate_main_444_16_intra: (23_040_000, 76_800_000),
        max_bit_rate_high_throughput_444_16_intra: (23_040_000, 76_800_000),
    },
];
