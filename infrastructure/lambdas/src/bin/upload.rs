use http::Method;
use lambda_http::{handler, IntoResponse, Request, Response};
use lambda_runtime::{self, Context, Error};
use rusoto_core::Region;
use rusoto_credential::{DefaultCredentialsProvider, ProvideAwsCredentials};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::PutObjectRequest;
use serde_json::json;
use serde_json::Value;

use lib::ErrorResponse;
use serde::Deserialize;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let result = match event.method() {
        &Method::POST => handle_post(event).await,
        x => Err(ErrorResponse {
            error: format!("Invalid method {}", x),
        }),
    };

    match result {
        Ok(value) => Ok(Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&value).expect("Could not convert JSON to string"))
            .expect("Failed to render response")),
        Err(e) => Ok(Response::builder()
            .status(500)
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&e).expect("Could not convert JSON to string"))
            .expect("Failed to render r}esponse")),
    }
}

async fn handle_post(event: Request) -> Result<Value, ErrorResponse> {
    // Receive the file's name and type
    #[derive(Deserialize)]
    struct Data {
        file_name: String,
    }
    let data: Data = serde_json::from_slice(event.body()).expect("Could not read JSON body");

    // Create the object key from the file name, type and current time
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get time")
        .as_millis();
    let key = format!("{}-{}", data.file_name, time);

    // Create a PutObjectRequest and get a presigned URL for it
    let req = PutObjectRequest {
        bucket: "recipebook-images".to_string(),
        key: key.clone(),
        acl: Some("public-read".to_string()),
        ..Default::default()
    };

    let presigned_url = req.get_presigned_url(
        &Region::EuWest2,
        &DefaultCredentialsProvider::new()
            .unwrap()
            .credentials()
            .await
            .unwrap(),
        &PreSignedRequestOption {
            expires_in: Duration::from_secs(300),
        },
    );

    // Return the presigned URL for upload along with the url for reading the object
    return Ok(
        json!({ "uploadUrl": format!("{}", &presigned_url), "url": format!("https://recipebook-images.s3.amazonaws.com/{}", &key) }),
    );
}
