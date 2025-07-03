use image::{GenericImageView, ImageReader};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct RtwImage {
    pub image_width: usize,
    pub image_height: usize,
    pub bytes_per_pixel: usize,
    pub bytes_per_scanline: usize,
    pub bdata: Vec<u8>,
    pub fdata: Vec<f32>,
}

impl RtwImage {
    pub fn new(filename: &str) -> Self {
        let mut image = RtwImage {
            image_width: 0,
            image_height: 0,
            bytes_per_pixel: 3,
            bytes_per_scanline: 0,
            bdata: Vec::new(),
            fdata: Vec::new(),
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
            env::var("RTW_IMAGES")
                .ok()
                .map(|dir| format!("{}/{}", dir, filename)),
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
        self.bdata = rgb_image.clone().into_raw();
        self.fdata = self.bdata.iter().map(|&b| b as f32 / 255.0).collect();

        true
    }

    pub fn width(&self) -> usize {
        self.image_width
    }

    pub fn height(&self) -> usize {
        self.image_height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.bdata.is_empty() {
            return &MAGENTA;
        }

        let x = Self::clamp(x, 0, self.image_width);
        let y = Self::clamp(y, 0, self.image_height);
        let offset = y * self.bytes_per_scanline + x * self.bytes_per_pixel;
        &self.bdata[offset..offset + 3]
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        // Return the value clamped to the range [low, high)
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }

    #[allow(dead_code)]
    fn float_to_byte(value: f32) -> u8 {
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
        self.bdata = self.fdata.iter().take(total_bytes)
            .map(|&v| Self::float_to_byte(v))
            .collect();
    }
}
