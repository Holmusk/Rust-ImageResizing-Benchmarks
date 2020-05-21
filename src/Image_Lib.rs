use image::imageops::FilterType;
use std::path::Path;

use std::env;


fn main() {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
                panic!("not enough arguments please specify the file name");
        }
        let filename = &args[1]; 
        open_scale_image(filename.to_string());
      
}
pub fn open_scale_image(filename: String){
    let img = image::open(filename).unwrap();
    let scaled = img.resize(299, 299, FilterType::CatmullRom);
    let path = Path::new("image_resizes.jpg");
    scaled.save(path).unwrap();
}