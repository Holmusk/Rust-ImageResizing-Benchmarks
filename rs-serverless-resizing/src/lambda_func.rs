#[warn(unused_imports)]
#[warn(dead_code)]
#[macro_use]
extern crate lambda_runtime as lambda;
use std::error::Error;
use log::{self, info, error};
use lambda::error::HandlerError;
use simple_logger;
use aws_lambda_events::event::s3::{S3Event, S3EventRecord};
use serde_json::Value;
use rusoto_s3::S3Client;
use rusoto_core::Region;
use std::env;



mod img_resizing_lambda;


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    lambda!(handle_event);
    Ok(())
}

#[tokio::main]
async fn handle_event(event: Value, ctx: lambda::Context) -> Result<(), HandlerError> {
    
    let s3_event: S3Event = match serde_json::from_value(event.clone()){
        Ok(s3_json) => s3_json,
        Err(s3_err) => panic!("Failed to convert S3 event to json object for aws request Id {} , error is {}",ctx.aws_request_id,s3_err),
        };
    info!("The event recd by the Lambda is: {}",event);
    let region: Region = s3_event.records[0]
    .aws_region.as_ref()
    .expect("Could not get region from record")
    .parse()
    .expect("Could not parse region from record");
    
    let s3 = S3Client::new(region);  
    let out_bucket = env::var("RESIZED_IMAGES_BUCKET_NAME").unwrap();    
    for record in s3_event.records {
      handle_record(record,s3.clone(),&out_bucket).await;
    }
    Ok(())
}

async fn handle_record(record: S3EventRecord,s3_client: rusoto_s3::S3Client, upload_bucket: &String){
    let bucket_name = record
            .s3
            .bucket
            .name
            .expect("Could not get bucket name from record");

    let key_name = record
    .s3
    .object
    .key
    .expect("Could not get key from object record");
    let s3_uploadclient = s3_client.clone();
    let download_img = img_resizing_lambda::download_img_from_s3(s3_client,bucket_name,key_name);
    let resized_image = img_resizing_lambda::resize_image(&download_img.await);
  
    img_resizing_lambda::upload_resized_img_to_s3(
        s3_uploadclient,
        upload_bucket.to_string(),
        img_resizing_lambda::IMAGE_NAME.to_string(),
        resized_image,
    )
    .await;
    
}