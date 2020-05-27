    use image::{imageops::FilterType, ImageBuffer,GenericImageView};
 mod resize;
    
    fn main(){
        let mut result: Vec<u8> = Vec::new();
        let img = image::open("images/original/test_image1.jpg").unwrap();   
        img.write_to(&mut result, image::ImageOutputFormat::Jpeg(90)).unwrap();

       // let image_slice = &[result];
       let image_resized =  resize::resize_image(&result);
        let img_resized = match image::load_from_memory(&image_resized) {
            Ok(image) => image,
            Err(imgerr) => panic!("Couldn't convert image to Image Bytes in test! {}", imgerr),
        };
        
       assert_eq!(img_resized.dimensions(), (299,299));  
       
    }
