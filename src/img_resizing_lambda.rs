use futures::TryStreamExt;
use image::{imageops::FilterType, ImageBuffer,GenericImageView};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use rusoto_s3::S3;

const BUCKET_NAME: &str = "rust-image-resizing";
const IMAGE_NAME: &str = "rust-image.jpg";
const REGION_NAME: &str = "ap-southeast-1";

fn main() {
    println!("Downloading file from S3 bucket...");
    let awsregion = get_region(REGION_NAME.to_string());
    println!("Region is {:?}", awsregion);
    let s3 = S3Client::new(awsregion);
    download_img_from_s3(s3, BUCKET_NAME.to_string(), IMAGE_NAME.to_string());
}

fn get_region(aws_region_name : String) -> Region{
  
        match aws_region_name.parse::<Region>() {
            Ok(valid_region) => valid_region,
            // Default fallback Region (Singapore)
            Err(_) =>  Region::ApSoutheast1,
        }
}

async fn download_img_from_s3(
    s3_client: rusoto_s3::S3Client,
    bucket_name: String,
    img_name: String,
) {
    let get_req = rusoto_s3::GetObjectRequest {
        bucket: bucket_name,
        key: img_name.clone(),
        ..Default::default()
    };

    let result = match s3_client.get_object(get_req).await {
        Ok(s3_result) => s3_result,
        Err(s3_geterr) => panic!("Couldn't GET image from S3! {}", s3_geterr),
    };

    let stream = match result.body {
        Some(stream_contentbody) => stream_contentbody,
        None => panic!("Did not get back content from S3 GET call!"),
    };

    let s3_object_bytes_mut = match stream
        .map_ok(|b| bytes::BytesMut::from(&b[..]))
        .try_concat()
        .await
    {
        Ok(objbytes) => objbytes,
        Err(converterr) => panic!("Couldn't convert S3 Object to Image Bytes! {}", converterr),
    };
    let bytes_mutref = s3_object_bytes_mut.as_ref();

    let resized_image = resize::resize_image(bytes_mutref);
    let resized_image_slice = &[..resized_image];
}

pub mod resize{
pub fn resize_image(bytes_img: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let image = match image::load_from_memory(bytes_img) {
        Ok(image) => image,
        Err(imgerr) => panic!("Couldn't convert S3 Object to Image Bytes! {}", imgerr),
    };
    let scaled = image.resize_exact(299, 299, FilterType::CatmullRom);

    scaled.write_to(&mut result, image::ImageOutputFormat::Jpeg(90)).unwrap();
    return result //returning as a vector because cannot return as a reference to local variable
    
}

}

