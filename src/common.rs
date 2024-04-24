use yuv::color::ChromaSampling;
use yuv::color::Depth;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileConstraint {
    pub max_bit_depth: Depth,
    pub max_chroma_format: ChromaSampling,
    pub mono_chrome: bool,
}

impl ProfileConstraint {
    pub fn new(max_bit_depth: Depth, max_chroma_format: ChromaSampling, mono_chrome: bool) -> Self {
        Self {
            max_bit_depth,
            max_chroma_format,
            mono_chrome,
        }
    }

    pub fn supports_bit_depth(&self, bit_depth: Depth) -> bool {
        bit_depth <= self.max_bit_depth
    }

    pub fn supports_chroma_format(&self, chroma_format: ChromaSampling) -> bool {
        match self.max_chroma_format {
            ChromaSampling::Cs420 => chroma_format == ChromaSampling::Cs420,
            ChromaSampling::Cs422 => {
                chroma_format == ChromaSampling::Cs420 || chroma_format == ChromaSampling::Cs422
            }
            ChromaSampling::Cs444 => true,
            ChromaSampling::Monochrome => self.supports_mono_chrome(),
        }
    }

    pub fn supports_mono_chrome(&self) -> bool {
        self.mono_chrome
    }
}

pub fn yuv_bitrate(
    width: u32,
    height: u32,
    fps: f32,
    subsampling: ChromaSampling,
    bit_depth: Depth,
) -> f32 {
    let pixels = width * height;

    dbg!(pixels);

    // Calculate bits per pixel (bpp) for luma and chroma based on subsampling.
    let bpp = match subsampling {
        ChromaSampling::Cs444 => {
            // 4:4:4 subsampling has 3 samples per pixel, all of equal bit depth.
            3 * bit_depth as u32
        }
        ChromaSampling::Cs422 => {
            // 4:2:2 subsampling has 2 chroma samples for every 2 pixels, plus 1 luma per pixel.
            (2 * bit_depth as u32) + (bit_depth as u32)
        }
        ChromaSampling::Cs420 => {
            // 4:2:0 subsampling has 1 chroma sample for every 4 pixels, plus 1 luma per pixel.
            (bit_depth as u32) + (bit_depth as u32 / 2)
        }
        ChromaSampling::Monochrome => {
            // 4:0:0 subsampling has only luma samples.
            bit_depth as u32
        }
    };

    // Calculate total bitrate.
    (pixels as f32 * bpp as f32 * fps) / 1000.0
}

// pub fn chroma_multiplier(base: u64, subsampling: ChromaSampling, bit_depth: Depth) -> u64 {
//     let multiplier = match subsampling {
//         ChromaSampling::Cs444 => 1,
//         ChromaSampling::Cs422 => 2,
//         ChromaSampling::Cs420 => 3,
//         ChromaSampling::Monochrome => 1,
//     };

//     let bpp = match subsampling {
//         ChromaSampling::Cs444 => {
//             // 4:4:4 subsampling has 3 samples per pixel, all of equal bit depth.
//             3 * bit_depth as u32
//         }
//         ChromaSampling::Cs422 => {
//             // 4:2:2 subsampling has 2 chroma samples for every 2 pixels, plus 1 luma per pixel.
//             (2 * bit_depth as u32) + (bit_depth as u32)
//         }
//         ChromaSampling::Cs420 => {
//             // 4:2:0 subsampling has 1 chroma sample for every 4 pixels, plus 1 luma per pixel.
//             (bit_depth as u32) + (bit_depth as u32 / 2)
//         }
//         ChromaSampling::Monochrome => {
//             // 4:0:0 subsampling has only luma samples.
//             bit_depth as u32
//         }
//     };

//     base * bpp as u64
// }
