use futures::TryStreamExt;
use image::imageops::FilterType;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::S3Client;
use rusoto_s3::S3;

const BUCKET_NAME1: &str = "kahlil-test-image-upload-bucket";
const BUCKET_NAME2: &str = "kahlil-test-images-to-be-rated";
const IMAGE_NAME: &str = "foodImg.jpg";
const REGION_NAME: &str = "ap-southeast-1";

#[tokio::main]
async fn main() {
    println!("Downloading file from S3 bucket...");
    let awsregion = get_region(REGION_NAME.to_string());
    let s3 = S3Client::new(awsregion);
    let s3_upload = s3.clone();
    let img_bytes = download_img_from_s3(s3, BUCKET_NAME1.to_string(), IMAGE_NAME.to_string());
    let resized_image = resize_image(&img_bytes.await);
    upload_resized_img_to_s3(
        s3_upload,
        BUCKET_NAME2.to_string(),
        IMAGE_NAME.to_string(),
        resized_image,
    )
    .await;
}

fn get_region(aws_region_name: String) -> Region {
    match aws_region_name.parse::<Region>() {
        Ok(valid_region) => valid_region,
        // Default fallback Region (Singapore)
        Err(_) => Region::ApSoutheast1,
    }
}

pub async fn download_img_from_s3(
    s3_client: rusoto_s3::S3Client,
    bucket_name: String,
    img_name: String,
) -> Vec<u8> {
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

    return bytes_mutref.to_vec();
}

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
    return img_result;
}

async fn upload_resized_img_to_s3(
    s3_client: rusoto_s3::S3Client,
    bucket_name: String,
    img_name: String,
    body: Vec<u8>,
) {
    match s3_client
        .put_object(rusoto_s3::PutObjectRequest {
            bucket: bucket_name,
            key: img_name.clone(),
            body: Some(ByteStream::from(body)),
            ..Default::default()
        })
        .await
    {
        Ok(result) => result,
        Err(s3_geterr) => panic!("Couldn't PUT image to S3! {}", s3_geterr),
    };
}
