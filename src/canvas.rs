use std::io::{Cursor, ErrorKind};
use image::{ImageBuffer, ImageOutputFormat, Rgb};

pub struct CanvasPicture {
    width: u32,
    height: u32,
    data: Vec<u8>,
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    bytes: Vec<u8>
}

impl CanvasPicture {
    pub fn new(width: u32, height: u32) -> Self {
        let mut a = CanvasPicture {
            width,
            height,
            img: ImageBuffer::new(width, height),
            data: vec![0; (width * height) as usize],
            bytes: Vec::new()
        };
        a.re_export();
        a
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u8) -> std::io::Result<()> {
        if x > self.width || y > self.height {
            return Err(std::io::Error::new(ErrorKind::InvalidData, "Pixel out of bounds"));
        }
        self.data.insert((x * self.width + y) as usize, pixel);
        self.data.remove((x * self.width + y + 1) as usize);

        self.img.put_pixel(x, y, CanvasPicture::minecraft_to_rgb(pixel));

        self.re_export();

        Ok(())
    }

    fn re_export(&mut self) {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        self.img.write_to(&mut buf, ImageOutputFormat::Png).unwrap();

        self.bytes = buf.into_inner();
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn minecraft_to_rgb(color: u8) -> Rgb<u8> {
        match color {
            1 => Rgb([241, 118, 19]),
            2 => Rgb([190, 69, 180]),
            3 => Rgb([58, 175, 218]),
            4 => Rgb([249, 198, 39]),
            5 => Rgb([112, 185, 25]),
            6 => Rgb([238, 141, 173]),
            7 => Rgb([62, 68, 71]),
            8 => Rgb([142, 142, 135]),
            9 => Rgb([21, 138, 145]),
            10 => Rgb([122, 42, 173]),
            11 => Rgb([53, 57, 158]),
            12 => Rgb([114, 71, 40]),
            13 => Rgb([85, 109, 27]),
            14 => Rgb([161, 39, 34]),
            15 => Rgb([20, 21, 25]),
            _ => Rgb([234, 237, 237]),
        }
    }
}
