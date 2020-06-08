#![allow(non_snake_case)]
use image::{imageops::FilterType, GenericImageView, ImageBuffer};
mod img_resizing_lambda;

fn main() {
    let mut result: Vec<u8> = Vec::new();
    let img = image::open("images/original/test_image1.jpg").unwrap();
    img.write_to(&mut result, image::ImageOutputFormat::Jpeg(90))
        .unwrap();

    // let image_slice = &[result];
    let image_resized_vec = img_resizing_lambda::resize_image(&result);
    let dyn_resized_img = match image::load_from_memory(&image_resized_vec) {
        Ok(image) => image,
        Err(imgerr) => panic!("Couldn't convert image to Image Bytes in test! {}", imgerr),
    };

    assert_eq!(dyn_resized_img.dimensions(), (299, 299));
}
