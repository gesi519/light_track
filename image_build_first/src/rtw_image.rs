use std::env;
use std::fs;
use image::{ImageReader, GenericImageView};
use std::path::Path;

#[derive(Debug)]
pub struct RtwImage {
    pub image_width: usize,
    pub image_height: usize,
    pub bytes_per_pixel: usize,
    pub bytes_per_scanline: usize,
    pub bdata: Option<Vec<u8>>,
    pub fdata: Option<Vec<f32>>,
}

impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let mut image = RtwImage {
            image_width: 0,
            image_height: 0,
            bytes_per_pixel: 3,
            bytes_per_scanline: 0,
            bdata: None,
            fdata: None,
        };
        let image_path = Self::resolve_image_path(filename);
        if let Some(path) = image_path {
            eprintln!("Resolved image path: {}\n", path);
            if image.load(&path) {
                return image;
            }
        }

        eprintln!("ERROR: Could not load image file '{}'.\n", filename);
        image
    }

    fn resolve_image_path(filename: &str) -> Option<String> {
        let search_paths = vec![
            env::var("RTW_IMAGES").ok().map(|dir| format!("{}/{}", dir, filename)),
            Some(filename.to_string()),
            Some(format!("images/{}", filename)),
            Some(format!("../images/{}", filename)),
            Some(format!("../../images/{}", filename)),
            Some(format!("../../../images/{}", filename)),
            Some(format!("../../../../images/{}", filename)),
            Some(format!("../../../../../images/{}", filename)),
            Some(format!("../../../../../../images/{}", filename)),
        ];

        for path in search_paths.into_iter().flatten() {
            if fs::metadata(&path).is_ok() {
                return Some(path);
            }
        }

        None
    }

    fn load(&mut self, filename: &str) -> bool {
        let img = match ImageReader::open(&Path::new(filename)) {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(_) => return false,
            },
            Err(_) => return false,
        };

        // 获取图像尺寸
        let (width, height) = img.dimensions();
        self.image_width = width as usize;
        self.image_height = height as usize;
        self.bytes_per_pixel = 3; // 只支持 RGB
        self.bytes_per_scanline = self.image_width * self.bytes_per_pixel;

        // 将图像转换为 RGB8 格式
        let rgb_image = img.to_rgb8();
        let raw_pixels = rgb_image.into_raw();

        // 将字节数据转换为浮点数据 [0.0, 1.0]
        let fdata: Vec<f32> = raw_pixels.iter().map(|&b| b as f32 / 255.0).collect();
        self.fdata = Some(fdata);
        self.bdata = Some(raw_pixels);

        true
    }

    pub fn width(&self) -> usize {
        if self.fdata.is_none() {
            0
        } else {
            self.image_width
        }
    }

    pub fn height(&self) -> usize {
        if self.fdata.is_none() {
            0
        } else {
            self.image_height
        }
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        static MAGENTA : [u8; 3] = [255, 0, 255];
        if let Some(ref bdata) = self.bdata {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);
            let offset = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize;
            &bdata[offset..offset + 3]
        }else {
            &MAGENTA
        }
    }

    fn clamp(x : usize, low : usize, high : usize) -> usize {   // Return the value clamped to the range [low, high)
        if x < low {
            low
        }else if x < high {
            x
        }else {
            high - 1
        }
    }

    #[allow(dead_code)]
    fn float_to_byte(value : f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (256.0 * value) as u8
        }
    }

    #[allow(dead_code)]
    fn convert_to_bytes(&mut self) {
        let total_bytes = self.image_width * self.image_height * self.bytes_per_pixel;
        let mut bdata = Vec::with_capacity(total_bytes);

        for i in 0..total_bytes {
            bdata.push(Self::float_to_byte(self.fdata.as_ref().unwrap()[i]));
        }

        self.bdata = Some(bdata);
    }
}
