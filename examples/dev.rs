pub fn main() {
    let v = video_levels::common::chroma_multiplier(
        1500,
        yuv::color::ChromaSampling::Cs444,
        yuv::color::Depth::Depth12,
    );

    let l = video_levels::hevc::get(video_levels::hevc::Level::L5_2);
    l.max_bit_rate(
        video_levels::hevc::Profile::Main444_16IntraHighThroughput,
        video_levels::hevc::Tier::Main,
    );
}
