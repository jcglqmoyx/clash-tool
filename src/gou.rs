use std::{collections::HashMap, io::Read};

use encoding_rs_io::DecodeReaderBytesBuilder;
use reqwest::{header::COOKIE, Client, Error};
use scraper::{Html, Selector};
use serde_json::json;

use crate::util::get_random_username;
use crate::{
    api::gou::{self, LOGIN_API},
    mail_tm, util,
    util::{cookies_to_string, generate_http_request_headers},
};

pub async fn send_verification_code_to_email(email_address: &str) -> Result<(), Error> {
    println!("Sending verification code to email...");
    let client = Client::builder().use_rustls_tls().build()?;
    let response = client
        .post(gou::MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    println!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn register(
    email: &mail_tm::TempEmailAccount,
    verification_code: String,
) -> Result<(), Error> {
    println!("Registering 加速狗 account...");
    let client = Client::builder().use_rustls_tls().build()?;
    client
        .post(gou::REGISTRATION_API)
        .json(&json!({
            "email": email.address,
            "name": email.address,
            "passwd": email.password,
            r#"repasswd"#: email.password,
            "wechat": get_random_username(8, 10),
            r#"imtype"#: 1,
            "code": 0,
            r#"emailcode"#: verification_code,
        }))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    Ok(())
}

pub async fn login(email: &mail_tm::TempEmailAccount) -> Result<HashMap<String, String>, Error> {
    let response = Client::new()
        .post(LOGIN_API)
        .json(&json!({
            "email": email.address,
            "passwd": email.password,
        }))
        .send()
        .await?;

    let cookies = util::parse_cookies(response.cookies().collect::<Vec<_>>());
    println!("Result: {:#?}", response.status());
    Ok(cookies)
}

pub async fn get_subscription_link(
    cookies: &HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = Client::new()
        .get(gou::USER_PROFILE_API)
        .header(COOKIE, cookies_to_string(cookies))
        .send()
        .await?
        .bytes()
        .await?;

    let cursor = std::io::Cursor::new(response.to_vec());
    let mut decoder = DecodeReaderBytesBuilder::new().build(cursor);
    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents)?;
    let contents = String::from_utf8(contents).unwrap();

    let document = Html::parse_document(&contents);
    let selector = Selector::parse(r#"input.form-control-monospace.cust-link[name="input1"]"#)
        .expect("Couldn't create selector.");
    let mut subscription_link = String::new();
    for element in document.select(&selector) {
        if let Some(value) = element.value().attr("value") {
            println!("Found subscription link: {}", value);
            subscription_link = value.to_string();
        }
    }

    Ok(subscription_link)
}
