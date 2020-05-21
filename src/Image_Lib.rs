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
    
    let scaled = img.resize(299, 299, FilterType::CatmullRom);
   
    let file = output_file(filename); //to get the filename 
    let path = Path::new("images/resized").join(file);
    scaled.save(path).unwrap();
}

//extracting the filename without extension from the filename given by the user
fn output_file (filename: String) ->String{

        let mut file_name: Vec<String> = Vec::new();
        for character in filename.chars(){
                match character {
                '.' => break,
                _ => file_name.push(character.to_string()),
             };
        }
             file_name.push("_resized.jpg".to_string()); //appending _resized to the output filename
             return file_name.join("")
}

