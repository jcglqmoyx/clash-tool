use reqwest::{Client, Error};
use serde_json::{json, Value};

use crate::{
    api::panda,
    mail_tm,
    util,
};

pub async fn verify(email_address: String) -> Result<(), Error> {
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

pub async fn register(email: mail_tm::TempEmailAccount, verification_code: String) -> Result<(), Error> {
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

pub async fn login(email: mail_tm::TempEmailAccount) -> Result<String, Error> {
    log::info!("Logging into Panda Node account...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client
        .post(panda::LOGIN_API)
        .json(&json!({
            "email": email.address,
            "password": email.password,
        }))
        .send()
        .await?;
    let response_text = response.text().await?;
    let v: Value = serde_json::from_str(&response_text).unwrap();
    let token = &v["data"]["token"];
    log::info!("Token: {:#?}", &token);
    log::info!("Subscription link: {:#?}", panda::SUBSCRIPTION_LINK.to_owned() + token.as_str().unwrap());
    Ok(String::from("ok"))
}
