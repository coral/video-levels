use yuv::color::ChromaSampling;
use yuv::color::Depth;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileConstraint {
    pub max_bit_depth: Depth,
    pub chroma_formats: Vec<ChromaSampling>,
}

impl ProfileConstraint {
    pub fn new(max_bit_depth: Depth, chroma_formats: Vec<ChromaSampling>) -> Self {
        Self {
            max_bit_depth,
            chroma_formats,
        }
    }

    pub fn max_chroma_format(&self) -> ChromaSampling {
        if self.chroma_formats.contains(&ChromaSampling::Cs444) {
            ChromaSampling::Cs444
        } else if self.chroma_formats.contains(&ChromaSampling::Cs422) {
            ChromaSampling::Cs422
        } else if self.chroma_formats.contains(&ChromaSampling::Cs420) {
            ChromaSampling::Cs420
        } else {
            ChromaSampling::Monochrome
        }
    }

    pub fn supports_chroma_format(&self, chroma_format: ChromaSampling) -> bool {
        self.chroma_formats.contains(&chroma_format)
    }

    pub fn supports_mono_chrome(&self) -> bool {
        self.chroma_formats.contains(&ChromaSampling::Monochrome)
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

    // Calculate bits per pixel (bpp) for luma and chroma based on subsampling.
    let bpp = match subsampling {
        ChromaSampling::Cs444 => {
            // 4:4:4 subsampling has 3 samples per pixel, all of equal bit depth.
            3 * bit_depth as u32
        }
        ChromaSampling::Cs422 => {
            // 4:2:2 subsampling has 2 samples per pixel (1Y + 0.5Cb + 0.5Cr).
            2 * bit_depth as u32
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

