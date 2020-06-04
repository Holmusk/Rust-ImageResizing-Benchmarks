use futures::TryStreamExt;
use rusoto_core::ByteStream;
use rusoto_s3::S3;

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

pub async fn upload_resized_img_to_s3(
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
