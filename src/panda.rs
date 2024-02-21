use reqwest::{Client, Error, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use api::panda::*;

use crate::api;
use crate::mail_tm::TempEmailAccount;
use crate::util::generate_http_request_headers;

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), Error> {
    let headers = generate_http_request_headers();
    let response = Client::new()
        .post(MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(headers)
        .send()
        .await?;
    Ok(())
}

pub async fn register_panda_node_account(email: TempEmailAccount, verification_code: String) -> Result<(), Error> {
    let headers = generate_http_request_headers();
    let response = Client::new()
        .post(REGISTRATION_API)
        .json(&json!({
            "email": email.address,
            "email_code": verification_code,
            "password": email.password,
        }))
        .headers(headers)
        .send()
        .await?;
    Ok(())
}

pub async fn login_panda_node_account(email: TempEmailAccount) -> Result<String, Error> {
    let response = Client::new()
        .post(LOGIN_API)
        .json(&json!({
            "email": email.address,
            "password": email.password,
        }))
        .send()
        .await?;
    println!("login_panda_node_account: {:?}", response);
    Ok("".to_string())
}
