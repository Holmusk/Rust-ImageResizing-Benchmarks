use image::imageops::FilterType;

pub fn resize_image(bytes_img: &[u8]) -> Vec<u8> {
    let mut img_result: Vec<u8> = Vec::new();
    let image = match image::load_from_memory(bytes_img) {
        Ok(image) => image,
        Err(imgerr) => panic!("Couldn't convert S3 Object to Image Bytes! {}", imgerr),
    };

    let scaled = image.resize_exact(299, 299, FilterType::CatmullRom);
    match scaled.write_to(&mut img_result, image::ImageOutputFormat::Jpeg(90)) {
        //setting the jpeg quality to 90
        Ok(()) => (),
        Err(write_err) => panic!("Couldn't convert S3 Object to Image Bytes! {}", write_err),
    }
    img_result
}
