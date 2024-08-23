use std::error;
use reqwest::{Client, Error};
use serde_json::{json, Value};

use crate::{
    api::panda,
    mail_tm,
    util,
};

pub async fn verify(email_address: &str) -> Result<(), Error> {
    log::info!("Sending verification code to email...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(panda::MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(util::generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn register(email: &mail_tm::TempEmailAccount, verification_code: String) -> Result<(), Error> {
    log::info!("Registering Panda Node account...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(panda::REGISTRATION_API)
        .json(&json!({
            "email": email.address,
            "email_code": verification_code,
            "password": email.password,
        }))
        .headers(util::generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn login(email: &mail_tm::TempEmailAccount) -> Result<String, Box<dyn error::Error>> {
    log::info!("Logging into Panda Node account...");

    // Create and configure the HTTP client
    let client = match Client::builder()
        .use_rustls_tls()
        .build() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    // Prepare and send the POST request
    let response = match client
        .post(panda::LOGIN_API)
        .json(&serde_json::json!({
            "email": email.address,
            "password": email.password,
        }))
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => return Err(Box::new(e)),
    };

    // Get the response text
    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(Box::new(e)),
    };

    // Parse the response text as JSON
    let v: Result<Value, _> = serde_json::from_str(&response_text);
    let v = match v {
        Ok(val) => val,
        Err(e) => return Err(Box::new(e)),
    };

    let token = match v["data"]["token"].as_str() {
        Some(t) => t,
        None => return Err(Box::new("")),
    };

    log::info!("Token: {:#?}", token);

    let subscription_link = format!("{}{}", panda::SUBSCRIPTION_LINK_PREFIX, token);
    log::info!("Subscription link: {:#?}", subscription_link);

    Ok(subscription_link)
}