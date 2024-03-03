use reqwest::{Client, Error};
use serde_json::json;

use crate::api::gou;
use crate::util;
use crate::mail_tm;
use crate::util::{generate_http_request_headers, get_random_username};

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), Error> {
    log::info!("Sending verification code to email...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(gou::MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(util::generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn register(email: mail_tm::TempEmailAccount, verification_code: String) -> Result<(), Error> {
    log::info!("Registering 加速狗 account...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(gou::REGISTRATION_API)
        .json(&json!({
            "email": email.address,
            "name": email.address,
            "password": email.password,
            "repasswd": email.password,
            "wechat": get_random_username(),

        }))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}
