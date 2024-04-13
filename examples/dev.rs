pub fn main() {
    let v = video_levels::common::chroma_multiplier(
        1500,
        yuv::color::ChromaSampling::Cs444,
        yuv::color::Depth::Depth12,
    );

    dbg!(v);
}
