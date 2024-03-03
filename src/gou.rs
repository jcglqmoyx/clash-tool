use std::collections::HashMap;
use std::fmt;

use log::log;
use reqwest::{Client, Error, header};
use reqwest::cookie::Cookie;
use serde_json::json;

use crate::{mail_tm, util};
use crate::api::{gou, panda};
use crate::api::gou::LOGIN_API;
use crate::util::{generate_http_request_headers, get_random_username};

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), Error> {
    log::info!("Sending verification code to email...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(gou::MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(generate_http_request_headers())
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
            "imtype": 1,
            "code": 0,
            "emailcode": verification_code,
        }))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn login(email: mail_tm::TempEmailAccount) -> Result<HashMap<String, String>, Error> {
    let response = Client::new()
        .post(LOGIN_API)
        .json(&json!({
            "email": email.address,
            "passwd": email.password,
        }))
        .send()
        .await?;

    let cookies = parse_cookies(response.cookies().collect::<Vec<_>>());
    log::info!("Cookies: {:#?}", cookies);
    log::info!("Result: {:#?}", response.status());
    Ok(cookies)
}

pub async fn get_subscription_link(cookies: &HashMap<String, String>) -> Result<String, Error> {
    let response = Client::new()
        .get(gou::USER_PROFILE_API)
        .headers(generate_http_request_headers())
        .header(header::COOKIE, util::cookies_to_string(cookies))
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    println!("{:#?}", response.text().await?);
    Ok("".to_string())
}

fn parse_cookies(cookies: Vec<Cookie>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for cookie in cookies {
        map.insert(cookie.name().to_string(), cookie.value().to_string());
    }
    map
}