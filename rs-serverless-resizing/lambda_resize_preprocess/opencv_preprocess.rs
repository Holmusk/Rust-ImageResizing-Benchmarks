use core::ops::Deref;
use opencv::core::Vec3;
use opencv::core::Vec3b;
use opencv::imgcodecs;
use opencv::prelude::*;
use std::vec;

pub fn readImgAndPreprocess(currentImgPath: String) -> Result<Vec<f32>, String> {
    let imgMatrix = match imgcodecs::imread(&currentImgPath, imgcodecs::IMREAD_COLOR) {
        Ok(imgMatVal) => imgMatVal,
        Err(opencvReadErr) => {
            return Err(format!("Error while reading food image: {}", opencvReadErr))
        }
    };

    let matDataVec = match Mat::data_typed::<Vec3b>(&imgMatrix) {
        Ok(matData) => matData,
        Err(matDataErr) => {
            return Err(format!(
                "Error while transforming img pixels: {}",
                matDataErr
            ))
        }
    };

    // 89401 -> No of pixels per channel = 299 x 299
    let pixelCountPerChan = 89401;
    let mut redPixels: Vec<f32> = vec![0.0; pixelCountPerChan];
    let mut greenPixels: Vec<f32> = vec![0.0; pixelCountPerChan];
    let mut bluePixels: Vec<f32> = vec![0.0; pixelCountPerChan];

    for i in 0..matDataVec.len() {
        let newVar: &[u8; 3] = Vec3::deref(&matDataVec[i]);
        let bluePixVal = standardizeBlue(convert_to_float(newVar[0]));
        let greenPixVal = standardizeGreen(convert_to_float(newVar[1]));
        let redPixVal = standardizeRed(convert_to_float(newVar[2]));

        // NOTE : OpenCV reads in BGR order by default.
        // NOTE : Changed back to RGB as the model expects it in that order

        redPixels[i] = redPixVal;
        greenPixels[i] = greenPixVal;
        bluePixels[i] = bluePixVal;
    }

    redPixels.append(&mut greenPixels);
    redPixels.append(&mut bluePixels);

    let allPixels = redPixels;
    return Ok(allPixels);
}

fn standardizeRed(inputPixelVal: f32) -> f32 {
    return (inputPixelVal - 0.5) / 0.5;
}
fn standardizeGreen(inputPixelVal: f32) -> f32 {
    return (inputPixelVal - 0.5) / 0.5;
}

fn standardizeBlue(inputPixelVal: f32) -> f32 {
    return (inputPixelVal - 0.5) / 0.5;
}

fn convert_to_float(pixel: u8) -> f32 {
    return (pixel as f32 / 255.0);
}
