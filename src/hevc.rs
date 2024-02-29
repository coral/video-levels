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
    L6_3,
    L7,
    L7_1,
    L7_2,
    L8_5,
}

pub fn select(width: u32, height: u32, framerate: f32) -> LevelDetail {
    let sr = (width * height) as u64 * framerate.ceil() as u64;
    for level in LEVEL_DETAILS.iter() {
        if sr <= level.max_luma_sample_rate {
            return *level;
        }
    }

    return LEVEL_DETAILS[LEVEL_DETAILS.len() - 1];
}

/// HEVC spec states that the level is a multiple of 30
/// "general_level_idc and sub_layer_level_idc[ i ] shall be set equal to a value of 30 times the level number specified in able A.8."
impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            30 => Level::L1,
            60 => Level::L2,
            63 => Level::L2_1,
            90 => Level::L3,
            93 => Level::L3_1,
            120 => Level::L4,
            123 => Level::L4_1,
            150 => Level::L5,
            153 => Level::L5_1,
            156 => Level::L5_2,
            180 => Level::L6,
            183 => Level::L6_1,
            186 => Level::L6_2,
            189 => Level::L6_3,
            210 => Level::L7,
            213 => Level::L7_1,
            216 => Level::L7_2,
            255 => Level::L8_5,
            _ => Level::L8_5,
        }
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
            Level::L6_3 => write!(f, "6.3"),
            Level::L7 => write!(f, "7"),
            Level::L7_1 => write!(f, "7.1"),
            Level::L7_2 => write!(f, "7.2"),
            Level::L8_5 => write!(f, "8.1"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LevelDetail {
    pub id: Level,
    /// Samples (pixels) per second
    pub max_luma_sample_rate: u64,
    /// MaxLumaPs (samples) per picture
    pub max_luma_picture_size: u32,
    pub max_bit_rate_main: u32,
    pub max_bit_rate_high: u32,
}

pub const LEVEL_DETAILS: [LevelDetail; 18] = [
    LevelDetail {
        id: Level::L1,
        max_luma_sample_rate: 552_960,
        max_luma_picture_size: 36_864,
        max_bit_rate_main: 128,
        max_bit_rate_high: 0,
    },
    LevelDetail {
        id: Level::L2,
        max_luma_sample_rate: 3_686_400,
        max_luma_picture_size: 122_880,
        max_bit_rate_main: 1_500,
        max_bit_rate_high: 0,
    },
    LevelDetail {
        id: Level::L2_1,
        max_luma_sample_rate: 7_372_800,
        max_luma_picture_size: 245_760,
        max_bit_rate_main: 3_000,
        max_bit_rate_high: 0,
    },
    LevelDetail {
        id: Level::L3,
        max_luma_sample_rate: 16_588_800,
        max_luma_picture_size: 552_960,
        max_bit_rate_main: 6_000,
        max_bit_rate_high: 0,
    },
    LevelDetail {
        id: Level::L3_1,
        max_luma_sample_rate: 33_177_600,
        max_luma_picture_size: 983_040,
        max_bit_rate_main: 10_000,
        max_bit_rate_high: 0,
    },
    LevelDetail {
        id: Level::L4,
        max_luma_sample_rate: 66_846_720,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: 12_000,
        max_bit_rate_high: 30_000,
    },
    LevelDetail {
        id: Level::L4_1,
        max_luma_sample_rate: 133_693_440,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: 20_000,
        max_bit_rate_high: 50_000,
    },
    LevelDetail {
        id: Level::L5,
        max_luma_sample_rate: 267_386_880,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 25_000,
        max_bit_rate_high: 100_000,
    },
    LevelDetail {
        id: Level::L5_1,
        max_luma_sample_rate: 534_773_760,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 40_000,
        max_bit_rate_high: 160_000,
    },
    LevelDetail {
        id: Level::L5_2,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 60_000,
        max_bit_rate_high: 240_000,
    },
    LevelDetail {
        id: Level::L6,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 60_000,
        max_bit_rate_high: 240_000,
    },
    LevelDetail {
        id: Level::L6_1,
        max_luma_sample_rate: 2_139_095_040,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 120_000,
        max_bit_rate_high: 480_000,
    },
    LevelDetail {
        id: Level::L6_2,
        max_luma_sample_rate: 4_278_190_080,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: 800_000,
    },
    LevelDetail {
        id: Level::L6_3,
        max_luma_sample_rate: 4_812_963_840,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: 1_600_000,
    },
    LevelDetail {
        id: Level::L7,
        max_luma_sample_rate: 4_812_963_840,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: 1_600_000,
    },
    LevelDetail {
        id: Level::L7_1,
        max_luma_sample_rate: 8_556_380_160,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 480_000,
        max_bit_rate_high: 3_200_000,
    },
    LevelDetail {
        id: Level::L7_2,
        max_luma_sample_rate: 17_112_760_320,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 960_000,
        max_bit_rate_high: 6_400_000,
    },
    LevelDetail {
        id: Level::L8_5,
        max_luma_sample_rate: std::u64::MAX,
        max_luma_picture_size: std::u32::MAX,
        max_bit_rate_main: std::u32::MAX,
        max_bit_rate_high: std::u32::MAX,
    },
];
