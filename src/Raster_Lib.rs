use raster::editor;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments please specify the file name");
    }
    let filename = &args[1];
    open_scale_image(filename.to_string());
}

fn open_scale_image(filename: String) {
    let in_filepath = input_file(&filename); //to get the filename

    // Create an image from file
    let mut image = raster::open(&in_filepath).unwrap();
    editor::resize(&mut image, 299, 299, raster::editor::ResizeMode::Exact).unwrap();

    let out_filepath = output_file(filename); //to get the filename

    raster::save(&image, &out_filepath).unwrap();
}

fn input_file(filename: &String) -> String {
    format!("images/original/{}", filename)
}

fn output_file(filename: String) -> String {
    format!("images/resized/{}{}", filename, "_resized.jpg") //TODO - sort out ugly naming convention
}
