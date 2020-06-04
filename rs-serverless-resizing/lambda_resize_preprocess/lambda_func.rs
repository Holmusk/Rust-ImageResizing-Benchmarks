#[warn(unused_imports)]
#[warn(dead_code)]
#[macro_use]
extern crate lambda_runtime as lambda;
use aws_lambda_events::event::s3::{S3Event, S3EventRecord};
use lambda::error::HandlerError;
use log::{self, info};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use serde_json::Value;
use simple_logger;
use std::env;
use std::error::Error;

mod resize;
mod s3_utils;
mod resized_img_path;
mod opencv_preprocess;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(handle_event);
    Ok(())
}

#[tokio::main]
async fn handle_event(event: Value, ctx: lambda::Context) -> Result<(), HandlerError> {
    let s3_event: S3Event = match serde_json::from_value(event.clone()) {
        Ok(s3_json) => s3_json,
        Err(s3_err) => panic!(
            "Failed to convert S3 event to json object for aws request Id {} , error is {}",
            ctx.aws_request_id, s3_err
        ),
    };

    let region: Region = s3_event.records[0]
        .aws_region
        .as_ref()
        .expect("Could not get region from record")
        .parse()
        .expect("Could not parse region from record");

    let s3 = S3Client::new(region);
    let out_bucket = env::var("RESIZED_IMAGES_BUCKET_NAME").unwrap();
    for record in s3_event.records {
        handle_record(record, s3.clone(), &out_bucket).await;
    }
    Ok(())
}

async fn handle_record(
    record: S3EventRecord,
    s3_client: rusoto_s3::S3Client,
    upload_bucket: &String,
) {
    let bucket_name = record
        .s3
        .bucket
        .name
        .expect("Could not get bucket name from record");

    let image_name = record
        .s3
        .object
        .key
        .expect("Could not get key from object record");
    info!("Downloading image from S3...");
    let download_img =
        s3_utils::download_img_from_s3(s3_client.clone(), bucket_name, image_name.clone());
    info!("Resizing image...");
    let resized_image = resize::resize_image(&download_img.await);
    info!("Uploading image to S3...");
    s3_utils::upload_resized_img_to_s3(
        s3_client,
        upload_bucket.to_string(),
        image_name,
        resized_image.clone(),
    )
    .await;
    let path = resized_img_path::resized_preprocess(&resized_image);
    let pixels = opencv_preprocess::readImgAndPreprocess(path);


    info!("Done!");
}
