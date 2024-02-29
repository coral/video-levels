pub fn main() {
    let framerate = 100.0;
    let level = video_levels::hevc::select(1920, 1080, framerate);
    println!("Level: {:?}", level);
    // Level: 5.1
}
