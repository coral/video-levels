use crate::common::ProfileSpecification;
/// Implementing the HEVC spec for levels
///
/// https://itu.int/rec/T-REC-H.265-202309-I/en
use std::fmt::Display;
use yuv::color::ChromaSampling;
use yuv::color::Depth;

/// select clamps down to the lowest level that supports your spec
///
/// * width and height in pixels
/// * framerate in frames per second
/// * max_bitrate in bits per second / 1000
/// * profile and tier are the HEVC profile and tier
// pub fn select(
//     width: u32,
//     height: u32,
//     framerate: f32,
//     max_bitrate: u32,
//     profile: Profile,
//     tier: Tier,
// ) -> LevelSpecification {
//     let sr = (width * height) as u64 * framerate.ceil() as u64;
//     for level in LEVEL_DETAILS.iter() {
//         if sr <= level.max_luma_sample_rate {
//             if let Some(mb) = level.max_bit_rate(&profile, &tier) {
//                 if mb >= max_bitrate {
//                     return *level;
//                 }
//             }
//         }
//     }

//     LEVEL_DETAILS[LEVEL_DETAILS.len() - 1]
// }

/// get returns the level specification for the given level
pub fn get(level: Level) -> LevelSpecification {
    for l in LEVEL_DETAILS.iter() {
        if l.id() == level {
            return *l;
        }
    }

    LEVEL_DETAILS[LEVEL_DETAILS.len() - 1]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    /// The Main tier was designed for most applications
    Main,
    /// High tier was designed for very demanding applications
    High,
}

/// Not a complete list but like... feel free to commit more
pub enum Profile {
    Main,
    Main10,
    Main12,
    Main422_10,
    Main444,
    Main444_16Intra,
    Main444_16IntraHighThroughput,
}

impl Profile {
    pub fn bitrate_multiplier(&self, cc: ChromaSampling, bp: Depth) -> f32 {
        match cc {
            ChromaSampling::Cs420 => match bp {
                Depth::Depth8 => 1.0,
                Depth::Depth10 => 1.0,
                Depth::Depth12 => 1.5,
                Depth::Depth16 => 3.0,
            },
            ChromaSampling::Cs422 => match bp {
                Depth::Depth8 => 2.0,
                Depth::Depth10 => 2.0,
                Depth::Depth12 => 3.0,
                Depth::Depth16 => 6.0,
            },
            ChromaSampling::Cs444 => match bp {
                Depth::Depth8 => 3.0,
                Depth::Depth10 => 3.0,
                Depth::Depth12 => 3.0,
                Depth::Depth16 => 8.0,
            },
            ChromaSampling::Monochrome => match bp {
                Depth::Depth8 => 1.0,
                Depth::Depth10 => 1.0,
                Depth::Depth12 => 1.5,
                Depth::Depth16 => 3.0,
            },
        }
    }
}

impl From<&Profile> for ProfileSpecification {
    fn from(profile: &Profile) -> Self {
        match profile {
            Profile::Main => ProfileSpecification::new(
                yuv::color::Depth::Depth8,
                yuv::color::ChromaSampling::Cs420,
                true,
            ),
            Profile::Main10 => ProfileSpecification::new(
                yuv::color::Depth::Depth10,
                yuv::color::ChromaSampling::Cs420,
                true,
            ),
            Profile::Main12 => ProfileSpecification::new(
                yuv::color::Depth::Depth12,
                yuv::color::ChromaSampling::Cs420,
                true,
            ),
            Profile::Main422_10 => ProfileSpecification::new(
                yuv::color::Depth::Depth10,
                yuv::color::ChromaSampling::Cs422,
                true,
            ),
            Profile::Main444 => ProfileSpecification::new(
                yuv::color::Depth::Depth8,
                yuv::color::ChromaSampling::Cs444,
                true,
            ),
            Profile::Main444_16Intra => ProfileSpecification::new(
                yuv::color::Depth::Depth16,
                yuv::color::ChromaSampling::Cs444,
                true,
            ),
            Profile::Main444_16IntraHighThroughput => ProfileSpecification::new(
                yuv::color::Depth::Depth16,
                yuv::color::ChromaSampling::Cs444,
                true,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    /// Everything goes when you hit 8.5
    L8_5,
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

impl Level {
    fn usize(&self) -> usize {
        match self {
            Level::L1 => 30,
            Level::L2 => 60,
            Level::L2_1 => 63,
            Level::L3 => 90,
            Level::L3_1 => 93,
            Level::L4 => 120,
            Level::L4_1 => 123,
            Level::L5 => 150,
            Level::L5_1 => 153,
            Level::L5_2 => 156,
            Level::L6 => 180,
            Level::L6_1 => 183,
            Level::L6_2 => 186,
            Level::L6_3 => 189,
            Level::L7 => 210,
            Level::L7_1 => 213,
            Level::L7_2 => 216,
            Level::L8_5 => 255,
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
pub struct LevelSpecification {
    id: Level,
    /// Samples (pixels) per second
    max_luma_sample_rate: u64,
    /// MaxLumaPs (samples) per picture
    max_luma_picture_size: u32,
    max_bit_rate_main: u32,
    max_bit_rate_high: Option<u32>,
}

impl LevelSpecification {
    pub fn id(&self) -> Level {
        self.id
    }
    pub fn max_luma_sample_rate(&self) -> u64 {
        self.max_luma_sample_rate
    }
    pub fn max_luma_picture_size(&self) -> u32 {
        self.max_luma_picture_size
    }
    // pub fn max_bit_rate(&self, profile: &Profile, tier: &Tier) -> Option<u32> {
    //     match profile {
    //         Profile::Main => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main),
    //             Tier::High => self.max_bit_rate_high,
    //         },
    //         Profile::Main10 => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_10()),
    //             Tier::High => self.max_bit_rate_high_10(),
    //         },
    //         Profile::Main12 => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_12()),
    //             Tier::High => self.max_bit_rate_high_12(),
    //         },
    //         Profile::Main422_10 => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_444_12()),
    //             Tier::High => self.max_bit_rate_high_444_12(),
    //         },
    //         Profile::Main444 => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_444_12()),
    //             Tier::High => self.max_bit_rate_high_444_12(),
    //         },
    //         Profile::Main444_16Intra => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_444_16_intra()),
    //             Tier::High => self.max_bit_rate_high_444_16_intra(),
    //         },
    //         Profile::Main444_16IntraHighThroughput => match tier {
    //             Tier::Main => Some(self.max_bit_rate_main_throughput_444_16_intra()),
    //             Tier::High => self.max_bit_rate_high_throughput_444_16_intra(),
    //         },
    //     }
    // }

    pub fn max_bit_rate(&self, profile: &Profile) -> u32 {
        let spec = ProfileSpecification::from(profile);
        // (Profile::Main12.bitrate_multiplier(ChromaSampling::Cs420, Depth::Depth12)
        //     * self.max_bit_rate_main as f32) as u32
    }
    pub fn max_bit_rate_main(&self) -> u32 {
        self.max_bit_rate_main
    }
    // pub fn max_bit_rate_main(&self) -> u32 {
    //     self.max_bit_rate_main
    // }
    // pub fn max_bit_rate_main_10(&self) -> u32 {
    //     self.max_bit_rate_main
    // }
    pub fn max_bit_rate_main_12(&self) -> u32 {
        //self.max_bit_rate_main

        (Profile::Main12.bitrate_multiplier(ChromaSampling::Cs420, Depth::Depth12)
            * self.max_bit_rate_main as f32) as u32

        //(self.max_bit_rate_main as f32 * 1.5) as u32
    }
    // pub fn max_bit_rate_main_444_12(&self) -> u32 {
    //     self.max_bit_rate_main * 3
    // }
    // pub fn max_bit_rate_main_444_16_intra(&self) -> u32 {
    //     self.max_bit_rate_main * 8
    // }
    // pub fn max_bit_rate_main_throughput_444_16_intra(&self) -> u32 {
    //     self.max_bit_rate_main_444_16_intra() * 12
    // }
    // pub fn max_bit_rate_high(&self) -> Option<u32> {
    //     self.max_bit_rate_high
    // }
    // pub fn max_bit_rate_high_10(&self) -> Option<u32> {
    //     self.max_bit_rate_high
    // }
    // pub fn max_bit_rate_high_12(&self) -> Option<u32> {
    //     self.max_bit_rate_high.map(|v| (v as f32 * 1.5) as u32)
    // }
    // pub fn max_bit_rate_high_444_12(&self) -> Option<u32> {
    //     self.max_bit_rate_high.map(|v| v * 3)
    // }
    // pub fn max_bit_rate_high_444_16_intra(&self) -> Option<u32> {
    //     self.max_bit_rate_high.map(|v| v * 8)
    // }
    // pub fn max_bit_rate_high_throughput_444_16_intra(&self) -> Option<u32> {
    //     self.max_bit_rate_high_444_16_intra().map(|v| v * 12)
    // }
    pub fn max_decoder_picture_buffer_size(&self, width: u32, height: u32) -> u32 {
        let luma_samples = width * height;
        let max_dpb_pic_buf = 6;

        if luma_samples <= self.max_luma_picture_size >> 2 {
            std::cmp::min(4 * max_dpb_pic_buf, 16)
        } else if luma_samples <= self.max_luma_picture_size >> 1 {
            std::cmp::min(2 * max_dpb_pic_buf, 16)
        } else if luma_samples <= (3 * self.max_luma_picture_size) >> 2 {
            std::cmp::min((4 * max_dpb_pic_buf) / 3, 16)
        } else {
            max_dpb_pic_buf
        }
    }
}

pub const LEVEL_DETAILS: [LevelSpecification; 18] = [
    LevelSpecification {
        id: Level::L1,
        max_luma_sample_rate: 552_960,
        max_luma_picture_size: 36_864,
        max_bit_rate_main: 128,
        max_bit_rate_high: None,
    },
    LevelSpecification {
        id: Level::L2,
        max_luma_sample_rate: 3_686_400,
        max_luma_picture_size: 122_880,
        max_bit_rate_main: 1_500,
        max_bit_rate_high: None,
    },
    LevelSpecification {
        id: Level::L2_1,
        max_luma_sample_rate: 7_372_800,
        max_luma_picture_size: 245_760,
        max_bit_rate_main: 3_000,
        max_bit_rate_high: None,
    },
    LevelSpecification {
        id: Level::L3,
        max_luma_sample_rate: 16_588_800,
        max_luma_picture_size: 552_960,
        max_bit_rate_main: 6_000,
        max_bit_rate_high: None,
    },
    LevelSpecification {
        id: Level::L3_1,
        max_luma_sample_rate: 33_177_600,
        max_luma_picture_size: 983_040,
        max_bit_rate_main: 10_000,
        max_bit_rate_high: None,
    },
    LevelSpecification {
        id: Level::L4,
        max_luma_sample_rate: 66_846_720,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: 12_000,
        max_bit_rate_high: Some(30_000),
    },
    LevelSpecification {
        id: Level::L4_1,
        max_luma_sample_rate: 133_693_440,
        max_luma_picture_size: 2_228_224,
        max_bit_rate_main: 20_000,
        max_bit_rate_high: Some(50_000),
    },
    LevelSpecification {
        id: Level::L5,
        max_luma_sample_rate: 267_386_880,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 25_000,
        max_bit_rate_high: Some(100_000),
    },
    LevelSpecification {
        id: Level::L5_1,
        max_luma_sample_rate: 534_773_760,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 40_000,
        max_bit_rate_high: Some(160_000),
    },
    LevelSpecification {
        id: Level::L5_2,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 8_912_896,
        max_bit_rate_main: 60_000,
        max_bit_rate_high: Some(240_000),
    },
    LevelSpecification {
        id: Level::L6,
        max_luma_sample_rate: 1_069_547_520,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 60_000,
        max_bit_rate_high: Some(240_000),
    },
    LevelSpecification {
        id: Level::L6_1,
        max_luma_sample_rate: 2_139_095_040,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 120_000,
        max_bit_rate_high: Some(480_000),
    },
    LevelSpecification {
        id: Level::L6_2,
        max_luma_sample_rate: 4_278_190_080,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: Some(800_000),
    },
    LevelSpecification {
        id: Level::L6_3,
        max_luma_sample_rate: 4_812_963_840,
        max_luma_picture_size: 35_651_584,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: Some(1_600_000),
    },
    LevelSpecification {
        id: Level::L7,
        max_luma_sample_rate: 4_812_963_840,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 240_000,
        max_bit_rate_high: Some(1_600_000),
    },
    LevelSpecification {
        id: Level::L7_1,
        max_luma_sample_rate: 8_556_380_160,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 480_000,
        max_bit_rate_high: Some(3_200_000),
    },
    LevelSpecification {
        id: Level::L7_2,
        max_luma_sample_rate: 17_112_760_320,
        max_luma_picture_size: 142_606_336,
        max_bit_rate_main: 960_000,
        max_bit_rate_high: Some(6_400_000),
    },
    LevelSpecification {
        id: Level::L8_5,
        max_luma_sample_rate: std::u64::MAX,
        max_luma_picture_size: std::u32::MAX,
        max_bit_rate_main: std::u32::MAX,
        max_bit_rate_high: Some(std::u32::MAX),
    },
];

#[cfg(test)]
mod tests {
    #[test]
    fn level_mult() {
        use crate::hevc::Level;

        assert_eq!(Level::L6_2, Level::from(186));
    }

    #[test]
    fn max_bitrate() {
        use crate::hevc::Level;

        // test level 5.2
        let l = crate::hevc::get(Level::L5_2);
        assert_eq!(l.id(), Level::L5_2);
        assert_eq!(l.max_bit_rate_main(), 60_000);
        //assert_eq!(l.max_bit_rate_main_10(), 60_000);
        assert_eq!(l.max_bit_rate_main_12(), 90_000);
        // assert_eq!(l.max_bit_rate_main_444_12(), 180_000);
        // assert_eq!(l.max_bit_rate_main_444_16_intra(), 480_000);
        // assert_eq!(l.max_bit_rate_main_throughput_444_16_intra(), 5_760_000);
        // assert_eq!(l.max_bit_rate_high_444_12(), Some(720_000));
        // assert_eq!(l.max_bit_rate_high_444_16_intra(), Some(1_920_000));
        // assert_eq!(
        //     l.max_bit_rate_high_throughput_444_16_intra(),
        //     Some(23_040_000)
        // );

        // test level 2
        let l = crate::hevc::get(Level::L2);
        assert_eq!(l.id(), Level::L2);
        assert_eq!(l.max_bit_rate_main(), 1_500);
        //assert_eq!(l.max_bit_rate_high_12(), None);
    }

    #[test]
    fn max_dpb_pic_buf() {
        use crate::hevc::Level;

        let l = crate::hevc::get(Level::L4);
        assert_eq!(l.max_decoder_picture_buffer_size(1280, 720), 12);
        assert_eq!(l.max_decoder_picture_buffer_size(1920, 1080), 6);

        let l = crate::hevc::get(Level::L5_2);
        assert_eq!(l.max_decoder_picture_buffer_size(1920, 1080), 16);
        assert_eq!(l.max_decoder_picture_buffer_size(2560, 1440), 12);
        assert_eq!(l.max_decoder_picture_buffer_size(3840, 2160), 6);
    }

    #[test]
    fn selecting() {
        use crate::hevc::{Level, Profile, Tier};

        // let level = crate::hevc::select(1920, 1080, 100.0, 38_000, Profile::Main, Tier::Main);
        // assert_eq!(level.id(), Level::L5_1);
    }
}
