use crate::common::ProfileConstraint;
/// Implementing the H.264/AVC spec for levels
///
/// https://www.itu.int/rec/T-REC-H.264
use std::fmt;
use yuv::color::ChromaSampling;
use yuv::color::Depth;

#[derive(Debug)]
pub struct LevelSelector {
    width: u32,
    height: u32,
    framerate: f32,
    profile: Profile,
    min_level: Option<Level>,
    max_level: Option<Level>,
    max_bitrate: Option<u32>,
}

impl Default for LevelSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelSelector {
    pub fn new() -> Self {
        Self {
            width: 1920,
            height: 1080,
            framerate: 30.0,
            profile: Profile::Main,
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
    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
    }

    pub fn select(self) -> Option<LevelSpecification> {
        let width_mbs = (self.width as u64).div_ceil(16);
        let height_mbs = (self.height as u64).div_ceil(16);
        let frame_mbs = width_mbs * height_mbs;
        let mb_rate = frame_mbs * self.framerate.ceil() as u64;

        for level in LEVEL_DETAILS.iter() {
            if mb_rate <= level.max_macroblock_rate
                && frame_mbs <= level.max_frame_size_mbs as u64
            {
                // Check if level fits within the max specified bitrate
                let selected = match self.max_bitrate {
                    Some(bitrate_constraint)
                        if level.max_bit_rate(self.profile) >= bitrate_constraint.into() =>
                    {
                        *level
                    }
                    None => *level,
                    _ => continue,
                };

                // Clamp to min level
                match self.min_level {
                    Some(min) if selected.id() < min => {
                        continue;
                    }
                    _ => {}
                }

                // Check if exceeds max level
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
    Baseline,
    Main,
    Extended,
    High,
    High10,
    High10Intra,
    High422,
    High422Intra,
    High444,
    High444Intra,
    CAVLC444Intra,
}

impl Profile {
    /// Returns the cpbBrVclFactor multiplier relative to the Baseline/Main/Extended base bitrate
    pub fn bitrate_multiplier(&self) -> f64 {
        match self {
            Profile::Baseline | Profile::Main | Profile::Extended => 1.0,
            Profile::High => 1.25,
            Profile::High10 | Profile::High10Intra => 3.0,
            Profile::High422 | Profile::High422Intra => 4.0,
            Profile::High444 | Profile::High444Intra | Profile::CAVLC444Intra => 4.0,
        }
    }
}

impl From<&Profile> for ProfileConstraint {
    fn from(profile: &Profile) -> Self {
        match profile {
            Profile::Baseline | Profile::Main | Profile::Extended => ProfileConstraint::new(
                Depth::Depth8,
                vec![ChromaSampling::Cs420],
            ),
            Profile::High => ProfileConstraint::new(
                Depth::Depth8,
                vec![ChromaSampling::Monochrome, ChromaSampling::Cs420],
            ),
            Profile::High10 | Profile::High10Intra => ProfileConstraint::new(
                Depth::Depth10,
                vec![ChromaSampling::Monochrome, ChromaSampling::Cs420],
            ),
            Profile::High422 | Profile::High422Intra => ProfileConstraint::new(
                Depth::Depth10,
                vec![
                    ChromaSampling::Monochrome,
                    ChromaSampling::Cs420,
                    ChromaSampling::Cs422,
                ],
            ),
            Profile::High444 | Profile::High444Intra | Profile::CAVLC444Intra => {
                ProfileConstraint::new(
                    Depth::Depth16,
                    vec![
                        ChromaSampling::Monochrome,
                        ChromaSampling::Cs420,
                        ChromaSampling::Cs422,
                        ChromaSampling::Cs444,
                    ],
                )
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    L1,
    L1b,
    L1_1,
    L1_2,
    L1_3,
    L2,
    L2_1,
    L2_2,
    L3,
    L3_1,
    L3_2,
    L4,
    L4_1,
    L4_2,
    L5,
    L5_1,
    L5_2,
    L6,
    L6_1,
    L6_2,
}

/// H.264 level_idc values
/// Level 1b uses level_idc=9 (or 11 with constraint_set3_flag)
impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            10 => Level::L1,
            9 => Level::L1b,
            11 => Level::L1_1,
            12 => Level::L1_2,
            13 => Level::L1_3,
            20 => Level::L2,
            21 => Level::L2_1,
            22 => Level::L2_2,
            30 => Level::L3,
            31 => Level::L3_1,
            32 => Level::L3_2,
            40 => Level::L4,
            41 => Level::L4_1,
            42 => Level::L4_2,
            50 => Level::L5,
            51 => Level::L5_1,
            52 => Level::L5_2,
            60 => Level::L6,
            61 => Level::L6_1,
            62 => Level::L6_2,
            _ => Level::L6_2,
        }
    }
}

impl Level {
    fn usize(&self) -> usize {
        match self {
            Level::L1 => 10,
            Level::L1b => 9,
            Level::L1_1 => 11,
            Level::L1_2 => 12,
            Level::L1_3 => 13,
            Level::L2 => 20,
            Level::L2_1 => 21,
            Level::L2_2 => 22,
            Level::L3 => 30,
            Level::L3_1 => 31,
            Level::L3_2 => 32,
            Level::L4 => 40,
            Level::L4_1 => 41,
            Level::L4_2 => 42,
            Level::L5 => 50,
            Level::L5_1 => 51,
            Level::L5_2 => 52,
            Level::L6 => 60,
            Level::L6_1 => 61,
            Level::L6_2 => 62,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level_str = match self {
            Level::L1 => "1",
            Level::L1b => "1b",
            Level::L1_1 => "1.1",
            Level::L1_2 => "1.2",
            Level::L1_3 => "1.3",
            Level::L2 => "2",
            Level::L2_1 => "2.1",
            Level::L2_2 => "2.2",
            Level::L3 => "3",
            Level::L3_1 => "3.1",
            Level::L3_2 => "3.2",
            Level::L4 => "4",
            Level::L4_1 => "4.1",
            Level::L4_2 => "4.2",
            Level::L5 => "5",
            Level::L5_1 => "5.1",
            Level::L5_2 => "5.2",
            Level::L6 => "6",
            Level::L6_1 => "6.1",
            Level::L6_2 => "6.2",
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
    /// MaxMBPS (macroblocks per second)
    max_macroblock_rate: u64,
    /// MaxFS (max frame size in macroblocks)
    max_frame_size_mbs: u32,
    /// MaxBR in kbit/s (base value for Baseline/Main/Extended profiles)
    max_bit_rate: u32,
    /// MaxDpbMbs (max decoded picture buffer in macroblocks)
    max_dpb_mbs: u32,
}

impl LevelSpecification {
    pub fn id(&self) -> Level {
        self.id
    }

    pub fn max_macroblock_rate(&self) -> u64 {
        self.max_macroblock_rate
    }

    pub fn max_frame_size_mbs(&self) -> u32 {
        self.max_frame_size_mbs
    }

    /// Returns max bitrate in kbit/s, scaled by the profile's cpbBrVclFactor
    pub fn max_bit_rate(&self, profile: Profile) -> u64 {
        (self.max_bit_rate as f64 * profile.bitrate_multiplier()) as u64
    }

    pub fn max_dpb_mbs(&self) -> u32 {
        self.max_dpb_mbs
    }

    /// Returns the maximum number of decoded picture buffer frames for the given resolution
    /// Formula: min(floor(MaxDpbMbs / (ceil(w/16) * ceil(h/16))), 16)
    pub fn max_dpb_frames(&self, width: u32, height: u32) -> u32 {
        let width_mbs = (width as u64).div_ceil(16);
        let height_mbs = (height as u64).div_ceil(16);
        let frame_mbs = width_mbs * height_mbs;
        if frame_mbs == 0 {
            return 16;
        }
        std::cmp::min((self.max_dpb_mbs as u64 / frame_mbs) as u32, 16)
    }
}

pub const LEVEL_DETAILS: [LevelSpecification; 20] = [
    LevelSpecification {
        id: Level::L1,
        max_macroblock_rate: 1_485,
        max_frame_size_mbs: 99,
        max_bit_rate: 64,
        max_dpb_mbs: 396,
    },
    LevelSpecification {
        id: Level::L1b,
        max_macroblock_rate: 1_485,
        max_frame_size_mbs: 99,
        max_bit_rate: 128,
        max_dpb_mbs: 396,
    },
    LevelSpecification {
        id: Level::L1_1,
        max_macroblock_rate: 3_000,
        max_frame_size_mbs: 396,
        max_bit_rate: 192,
        max_dpb_mbs: 900,
    },
    LevelSpecification {
        id: Level::L1_2,
        max_macroblock_rate: 6_000,
        max_frame_size_mbs: 396,
        max_bit_rate: 384,
        max_dpb_mbs: 2_376,
    },
    LevelSpecification {
        id: Level::L1_3,
        max_macroblock_rate: 11_880,
        max_frame_size_mbs: 396,
        max_bit_rate: 768,
        max_dpb_mbs: 2_376,
    },
    LevelSpecification {
        id: Level::L2,
        max_macroblock_rate: 11_880,
        max_frame_size_mbs: 396,
        max_bit_rate: 2_000,
        max_dpb_mbs: 2_376,
    },
    LevelSpecification {
        id: Level::L2_1,
        max_macroblock_rate: 19_800,
        max_frame_size_mbs: 792,
        max_bit_rate: 4_000,
        max_dpb_mbs: 4_752,
    },
    LevelSpecification {
        id: Level::L2_2,
        max_macroblock_rate: 20_250,
        max_frame_size_mbs: 1_620,
        max_bit_rate: 4_000,
        max_dpb_mbs: 8_100,
    },
    LevelSpecification {
        id: Level::L3,
        max_macroblock_rate: 40_500,
        max_frame_size_mbs: 1_620,
        max_bit_rate: 10_000,
        max_dpb_mbs: 8_100,
    },
    LevelSpecification {
        id: Level::L3_1,
        max_macroblock_rate: 108_000,
        max_frame_size_mbs: 3_600,
        max_bit_rate: 14_000,
        max_dpb_mbs: 18_000,
    },
    LevelSpecification {
        id: Level::L3_2,
        max_macroblock_rate: 216_000,
        max_frame_size_mbs: 5_120,
        max_bit_rate: 20_000,
        max_dpb_mbs: 20_480,
    },
    LevelSpecification {
        id: Level::L4,
        max_macroblock_rate: 245_760,
        max_frame_size_mbs: 8_192,
        max_bit_rate: 20_000,
        max_dpb_mbs: 32_768,
    },
    LevelSpecification {
        id: Level::L4_1,
        max_macroblock_rate: 245_760,
        max_frame_size_mbs: 8_192,
        max_bit_rate: 50_000,
        max_dpb_mbs: 32_768,
    },
    LevelSpecification {
        id: Level::L4_2,
        max_macroblock_rate: 522_240,
        max_frame_size_mbs: 8_704,
        max_bit_rate: 50_000,
        max_dpb_mbs: 34_816,
    },
    LevelSpecification {
        id: Level::L5,
        max_macroblock_rate: 589_824,
        max_frame_size_mbs: 22_080,
        max_bit_rate: 135_000,
        max_dpb_mbs: 110_400,
    },
    LevelSpecification {
        id: Level::L5_1,
        max_macroblock_rate: 983_040,
        max_frame_size_mbs: 36_864,
        max_bit_rate: 240_000,
        max_dpb_mbs: 184_320,
    },
    LevelSpecification {
        id: Level::L5_2,
        max_macroblock_rate: 2_073_600,
        max_frame_size_mbs: 36_864,
        max_bit_rate: 240_000,
        max_dpb_mbs: 184_320,
    },
    LevelSpecification {
        id: Level::L6,
        max_macroblock_rate: 4_177_920,
        max_frame_size_mbs: 139_264,
        max_bit_rate: 240_000,
        max_dpb_mbs: 696_320,
    },
    LevelSpecification {
        id: Level::L6_1,
        max_macroblock_rate: 8_355_840,
        max_frame_size_mbs: 139_264,
        max_bit_rate: 480_000,
        max_dpb_mbs: 696_320,
    },
    LevelSpecification {
        id: Level::L6_2,
        max_macroblock_rate: 16_711_680,
        max_frame_size_mbs: 139_264,
        max_bit_rate: 800_000,
        max_dpb_mbs: 696_320,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_idc() {
        assert_eq!(Level::L3_1, Level::from(31));
        assert_eq!(Level::L1b, Level::from(9));
        assert_eq!(Level::L1, Level::from(10));
        assert_eq!(Level::L4_2, Level::from(42));
        assert_eq!(Level::L6_2, Level::from(62));
        // Unknown values fall back to L6.2
        assert_eq!(Level::L6_2, Level::from(99));
    }

    #[test]
    fn max_bitrate() {
        let l = get(Level::L5_1);
        assert_eq!(l.id(), Level::L5_1);
        // Base bitrate for Main profile: 240,000 kbit/s
        assert_eq!(l.max_bit_rate(Profile::Main), 240_000);
        // High: 240,000 * 1.25 = 300,000
        assert_eq!(l.max_bit_rate(Profile::High), 300_000);
        // High10: 240,000 * 3.0 = 720,000
        assert_eq!(l.max_bit_rate(Profile::High10), 720_000);
        // High422: 240,000 * 4.0 = 960,000
        assert_eq!(l.max_bit_rate(Profile::High422), 960_000);
    }

    #[test]
    fn max_dpb_frames() {
        // L4, 1920x1080: ceil(1920/16)*ceil(1080/16) = 120*68 = 8160
        // MaxDpbMbs=32768, 32768/8160 = 4
        let l = get(Level::L4);
        assert_eq!(l.max_dpb_frames(1920, 1080), 4);

        // L4, 1280x720: ceil(1280/16)*ceil(720/16) = 80*45 = 3600
        // MaxDpbMbs=32768, 32768/3600 = 9
        assert_eq!(l.max_dpb_frames(1280, 720), 9);

        // L5.1, 1920x1080: MaxDpbMbs=184320, 184320/8160 = 22 -> capped at 16
        let l = get(Level::L5_1);
        assert_eq!(l.max_dpb_frames(1920, 1080), 16);
    }

    #[test]
    fn select_base_cases() {
        // 1280x720@30 -> L3.1
        // frame_mbs = 80*45 = 3600, mb_rate = 3600*30 = 108000
        // L3.1: MaxMBPS=108000, MaxFS=3600 -> fits
        assert_eq!(
            LevelSelector::new()
                .width(1280)
                .height(720)
                .framerate(30.0)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L3_1
        );

        // 1920x1080@30 -> L4
        // frame_mbs = 120*68 = 8160, mb_rate = 8160*30 = 244800
        // L4: MaxMBPS=245760, MaxFS=8192 -> fits
        assert_eq!(
            LevelSelector::new()
                .width(1920)
                .height(1080)
                .framerate(30.0)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L4
        );

        // 1920x1080@60 -> L4.2
        // frame_mbs = 8160, mb_rate = 8160*60 = 489600
        // L4.2: MaxMBPS=522240, MaxFS=8704 -> fits
        assert_eq!(
            LevelSelector::new()
                .width(1920)
                .height(1080)
                .framerate(60.0)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L4_2
        );

        // 3840x2160@30 -> L5.1
        // frame_mbs = 240*135 = 32400, mb_rate = 32400*30 = 972000
        // L5.1: MaxMBPS=983040, MaxFS=36864 -> fits
        assert_eq!(
            LevelSelector::new()
                .width(3840)
                .height(2160)
                .framerate(30.0)
                .profile(Profile::Main)
                .select()
                .unwrap()
                .id(),
            Level::L5_1
        );
    }

    #[test]
    fn select_clamp_cases() {
        // Min clamp forces higher level
        // 1280x720@30 would normally select L3.1, but min clamp to L5 forces L5
        assert_eq!(
            LevelSelector::new()
                .width(1280)
                .height(720)
                .framerate(30.0)
                .profile(Profile::Main)
                .clamp(Level::L5, Level::L6_2)
                .select()
                .unwrap()
                .id(),
            Level::L5
        );

        // Max clamp returns None when exceeded
        // 1920x1080@30 selects L4, but max clamp to L3.2 means it exceeds
        assert!(LevelSelector::new()
            .width(1920)
            .height(1080)
            .framerate(30.0)
            .profile(Profile::Main)
            .clamp(Level::L1, Level::L3_2)
            .select()
            .is_none());
    }
}
