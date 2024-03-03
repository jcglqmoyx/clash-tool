use std::{
    collections::HashMap,
    io::Read,
};

use encoding_rs_io::DecodeReaderBytesBuilder;
use reqwest::{
    Client,
    cookie::Cookie,
    Error,
    header::COOKIE,
};
use scraper::{Html, Selector};
use serde_json::json;

use crate::{
    api::gou::{
        self,
        LOGIN_API,
    },
    mail_tm,
    util::{
        cookies_to_string,
        generate_http_request_headers,
        get_random_username,
    }};

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
            "wechat": get_random_username(8, 10),
            "imtype": 1,
            "code": 0,
            "emailcode": verification_code,
        }))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", &response.status());
    println!("{:#?}", &response.text().await);
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

pub async fn get_subscription_link(cookies: &HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
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
    let selector = Selector::parse(r#"input.form-control-monospace.cust-link[name="input1"]"#).expect("Couldn't create selector.");
    let mut cnt = 1;
    for element in document.select(&selector) {
        if let Some(value) = element.value().attr("value") {
            cnt += 1;
            if cnt == 2 {
                log::info!("Subscription link: {}", value);
                return Ok(value.to_string());
            }
        }
    }

    Ok("".parse().unwrap())
}

fn parse_cookies(cookies: Vec<Cookie>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for cookie in cookies {
        map.insert(cookie.name().to_string(), cookie.value().to_string());
    }
    map
}