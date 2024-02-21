use std::str::FromStr;
use std::sync::Arc;

use reqwest::{Client, Error, Response};
use reqwest::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use rustls::{ClientConfig, SupportedCipherSuite};
use rustls::CipherSuite;
use serde_json::{json, Value};
use tokio;
use tokio::runtime::Runtime;

use api::panda::*;

use crate::api;
use crate::mail_tm::TempEmailAccount;
use crate::util::generate_http_request_headers;

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), reqwest::Error> {
    // send a post request to the api using TLS 1.2
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    Ok(())
}

pub async fn register_panda_node_account(email: TempEmailAccount, verification_code: String) -> Result<(), Error> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(REGISTRATION_API)
        .json(&json!({
            "email": email.address,
            "email_code": verification_code,
            "password": email.password,
        }))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    println!("register_panda_node_account: {:#?}", response.text().await?);
    Ok(())
}

pub async fn login_panda_node_account(email: TempEmailAccount) -> Result<String, Error> {
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    let response = client
        .post(LOGIN_API)
        .json(&json!({
            "email": email.address,
            "password": email.password,
        }))
        .send()
        .await?;
    let response_text = response.text().await?;
    let v: Value = serde_json::from_str(&response_text).unwrap();
    let token = &v["data"]["token"];
    println!("token: {:#?}", &token);
    println!("login_panda_node_account: {:#?}", SUBSCRIPTION_LINK.to_owned() + token.as_str().unwrap());
    Ok("".to_string())
}
