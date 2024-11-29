use std::collections::HashMap;
use std::error;
use std::str;
use std::string::String;

use crate::api::wall;
use crate::mail_tm;
use crate::util::{cookies_to_string, generate_http_request_headers};
use regex::Regex;
use reqwest::header::COOKIE;
use reqwest::{Client, Error};
use serde_json;

pub async fn send_verification_code_to_email(email_address: &str) -> Result<(), Box<dyn error::Error>> {
    println!("Sending verification code to email...");

    let client = match Client::builder()
        .use_rustls_tls()
        .build() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let response = match client
        .post(wall::MAIL_VERIFICATION_CODE_API)
        .json(&serde_json::json!({"email": email_address}))
        .headers(generate_http_request_headers())
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => return Err(Box::new(e)),
    };

    println!("Result: {:#?}", response.status());

    Ok(())
}


pub async fn register(
    email: &mail_tm::TempEmailAccount,
    verification_code: String,
) -> Result<(), Box<dyn error::Error>> {
    println!("Registering account...");

    let client = match Client::builder()
        .use_rustls_tls()
        .build() {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let payload = serde_json::json!({
        r#"emailcode"#: verification_code,
        "code": "0",
        "name": email.address,
        "email": email.address,
        "passwd": email.password,
        r#"repasswd"#: email.password,
    });

    let response = match client
        .post(wall::REGISTRATION_API)
        .json(&payload)
        .headers(generate_http_request_headers())
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => return Err(Box::new(e)),
    };

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(Box::new(e)),
    };

    println!("Result: {:#?}", response_text);

    Ok(())
}

pub async fn login(email: &mail_tm::TempEmailAccount) -> Result<HashMap<String, String>, Error> {
    let response = match Client::new()
        .post(wall::LOGIN_API)
        .json(&serde_json::json!({
            "code":"",
            "email": email.address,
            "passwd": email.password,
        }))
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => return Err(*Box::new(e)),
    };

    let cookies = crate::util::parse_cookies(response.cookies().collect::<Vec<_>>());
    Ok(cookies)
}

pub async fn get_subscription_link(
    cookies: &HashMap<String, String>
) -> Result<String, Box<dyn error::Error>> {
    let response = match Client::new()
        .get(wall::USER_PROFILE_API)
        .header(COOKIE, cookies_to_string(cookies))
        .send()
        .await {
        Ok(resp) => match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => return Err(Box::new(e)),
        },
        Err(e) => return Err(Box::new(e)),
    };

    let response_str = match str::from_utf8(&response) {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };

    let re = match Regex::new(r"https://\S+?\.top/link/\S+?\?clash=1") {
        Ok(regex) => regex,
        Err(e) => return Err(Box::new(e)),
    };

    for mat in re.find_iter(response_str) {
        println!("Subscription link: {}", mat.as_str());
        return Ok(mat.as_str().to_string());
    }

    Ok("".to_string())
}