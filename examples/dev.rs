use video_levels::hevc::{self, *};
pub fn main() {
    let l = video_levels::hevc::get(video_levels::hevc::Level::L5_2);
    l.max_bit_rate(
        video_levels::hevc::Profile::Main444_16IntraHighThroughput,
        video_levels::hevc::Tier::Main,
    );

    let l: LevelSpecification = crate::hevc::LevelSelector::new()
        .width(3840)
        .height(2160)
        .framerate(60.0)
        .tier(Tier::Main)
        .profile(Profile::Main)
        .select()
        .unwrap();
    dbg!(l);
}
