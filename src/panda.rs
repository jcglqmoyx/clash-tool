use reqwest::{Client, Error, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use api::panda::*;

use crate::api;

fn generate_http_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers
}

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), Error> {
    let headers = generate_http_request_headers();
    let response = Client::new()
        .post(MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address,}))
        .headers(headers)
        .send()
        .await?;
    println!("send_verification_code_to_email: {:?}", response);
    Ok(())
}

pub async fn register_panda_account() -> Result<Response, Error> {
    let url = "http://baidu.com";
    let resp = reqwest::get(url).await?;
    Ok(resp)
}
