use image::imageops::FilterType;
use std::path::Path;

pub fn resized_preprocess(bytes_img: &[u8]) -> String {
    image::save_buffer(&Path::new("image.jpeg"), bytes_img, 299, 299, image::ColorType::Rgb8);
   
    return "image.jpeg".to_string();
}
