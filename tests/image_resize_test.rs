use image::{imageops::FilterType, ImageBuffer,GenericImageView};

#[test]
fn image_dimension_pass(){
    let img = image::open("images/original/test_image1.jpg").unwrap();    
    let image_resized =   img.resize_exact(299, 299,FilterType:: CatmullRom);
    
    assert_eq!(image_resized.dimensions(), (299,299));  
}

#[test]
fn image_dimension_fail(){
    let img = image::open("images/original/test_image2.jpg").unwrap();    
    let image_resized =   img.resize_exact(299, 299,FilterType:: CatmullRom);
    
    assert_eq!(image_resized.dimensions(), (299,56));
}