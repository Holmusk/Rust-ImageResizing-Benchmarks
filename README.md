# Rust-ImageResizing-Lambda
An AWS Lambda function to resize images obtained from a S3 bucket using Rust. 
## Initial resizing Project 
1. Images in the `images' folder are resized using [Image](https://github.com/image-rs/image) crate and [Raster](https://github.com/kosinix/raster) crate. 
2. A test module to test that the resized image is of the required dimesnions
3. A lambda handler module which downloads the image from an S3 bucket resizes using the image crate and then uploads it back to another S3 bucket

## Rust serverless Resizing
Run Rust on AWS Lambda  using serverless framework. 
Modular restructure of the above project. Downloading and uploading from/to a S3 bucket in `s3utils.rs` , resizing function in `resize.rs` and lambda event handler in `lambda_func.rs`

### Plug in Requirements 
Install serverless-rust plug in
```sh
$ npm i -D serverless-rust
```
Install S3 plugin
```sh
$ npm install serverless-plugin-existing-s3
```

### Bucket names 
Modify bucket names in the `iamRoleStatements - Resource`  and `environment` attribute in `serverless.yml` 
