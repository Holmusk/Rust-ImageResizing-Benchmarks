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

fn open_scale_image(filename: String){

    let input_file =  Path::new("images/original").join(&filename);
    let img = image::open(input_file).unwrap();
    
    let scaled = img.resize_exact(299, 299,FilterType:: CatmullRom);
   
    let file = output_file(filename); //to get the filename 
    let path = Path::new("images/resized").join(file);
    scaled.save(path).unwrap();
}

fn output_file (filename: String) ->String{

        format!("{}{}", filename, "_resized.jpg".to_string())  //TODO - sort out ugly naming convention
}

