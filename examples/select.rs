use video_levels::hevc::{Level, LevelSelector, Profile, Tier};
pub fn main() {
    // Find the level that supports 1920x1080 resolution at 60fps.
    let level = LevelSelector::new()
        .width(1920)
        .height(1080)
        .framerate(60.0)
        .tier(Tier::Main)
        .profile(Profile::Main)
        .select();

    println!("Level: {:?}", level.unwrap().id());

    // Sometimes you are bound to the what the hardware encoder supports.
    // You can define the clamp to search within a range.
    // In this case, we are looking for a level that supports 3840x2160 resolution at 60fps.
    let level = LevelSelector::new()
        .width(3840)
        .height(2160)
        .framerate(60.0)
        .tier(Tier::Main)
        .profile(Profile::Main)
        // Clamping between L4 and L5.2
        .clamp(Level::L4, Level::L5_2)
        .select();

    println!("Level: {:?}", level.unwrap().id());
}
