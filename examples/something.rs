pub fn main() {
    let dims = video_levels::Dimension {
        width: 1920,
        height: 1080,
    };
    let framerate = 100.0;
    let level = video_levels::Level::select(dims, framerate);
    println!("Level: {:?}", level);
    // Level: 5.1
}
