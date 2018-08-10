//! Image utilities

use failure::Error;
use png;
use std::path::Path;

/// PNG image in RGB format
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
    pub fdata: Vec<f32>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![0_u8; width * height * 3 /* RGBA */],
            fdata: vec![0_f32; width * height * 3 /* RGBA */],
        }
    }

    /// Image's `(width, height)`
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// Writes the pixels into a png image at `output`.
    ///
    /// `soa` specifies whether the bytes in `fdata` are in a Struct of Arrays (rrr...ggg...bbb...)
    /// or Array of Structs (rgbrgbrgb...) format.
    pub fn write_png(
        &mut self,
        output: &Path,
        soa: bool,
    ) -> Result<(), Error> {
        use png::HasParameters;
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(output)?;
        let ref mut buf_writer = BufWriter::new(file);
        let mut encoder = png::Encoder::new(
            buf_writer,
            self.width as u32,
            self.height as u32,
        );

        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        fn clamp(x: f32) -> u8 {
            let mut i = (x * 255.5) as isize;

            if i < 0 {
                i = 0
            };
            if i > 255 {
                i = 255
            };

            return i as u8;
        }

        if !soa {
            for (&fp, up) in self.fdata.iter().zip(self.data.iter_mut()) {
                (*up) = clamp(fp);
            }
        } else {
            let len = (self.width * self.height) as usize;
            let (r, tail) = self.fdata.split_at(len);
            let (g, b) = tail.split_at(len);
            assert!(r.len() == len);
            assert!(g.len() == len);
            assert!(b.len() == len);

            for i in 0..len {
                self.data[3 * i + 0] = clamp(r[i]);
                self.data[3 * i + 1] = clamp(g[i]);
                self.data[3 * i + 2] = clamp(b[i]);
            }
        }

        writer.write_image_data(&self.data)?;
        Ok(())
    }
}
