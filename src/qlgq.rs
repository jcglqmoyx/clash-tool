use std::collections::HashMap;

use reqwest::{Client, Error};
use serde_json::json;

use crate::api::qlgq;
use crate::mail_tm;
use crate::util::generate_http_request_headers;

pub async fn send_verification_code_to_email(email_address: String) -> Result<(), Error> {
    log::info!("Sending verification code to email...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;
    let response = client
        .post(qlgq::MAIL_VERIFICATION_CODE_API)
        .json(&json!({"email": email_address}))
        .headers(generate_http_request_headers())
        .send()
        .await?;
    log::info!("Result: {:#?}", response.status());
    Ok(())
}

pub async fn register(email: mail_tm::TempEmailAccount, verification_code: String) -> Result<(), Error> {
    log::info!("Registering 墙了个墙 account...");
    let client = Client::builder()
        .use_rustls_tls()
        .build()?;

    println!("register qlgq account\nemailcode:{}\n,address:{}\n,passwd:{}\n", &verification_code, &email.address.as_str(), email.password.as_str());
    let response = client
        .post(qlgq::REGISTRATION_API)
        .json(&json!({
            "emailcode": verification_code.as_str(),
            "code": "0",
            "name": email.address.as_str(),
            "email": email.address.as_str(),
            "passwd": email.password.as_str(),
            "repasswd": email.password.as_str(),
        }))
        // .headers(generate_http_request_headers())
        .send()
        .await?;

    log::info!("Result: {:#?}", &response.text().await);
    Ok(())
}
pub async fn login(email: mail_tm::TempEmailAccount) -> Result<HashMap<String, String>, Error> {
    log::info!("Logging into 墙了个墙 account...");
    println!("email is {}.", email.address.clone());
    let response = Client::new()
        .post(qlgq::LOGIN_API)
        .json(&json!({
            "code":"",
            "email": email.address,
            "passwd": email.password,
        }))
        .send()
        .await?;

    // let cookies = crate::gou::parse_cookies(response.cookies().collect::<Vec<_>>());
    // log::info!("Cookies: {:#?}", cookies);
    // log::info!("Result: {:#?}", response.status());
    // Ok(cookies)
    println!("response:....{:?}....", response.text().await);
    let mut res = HashMap::new();
    res.insert("hello".to_string(), "hello".to_string());
    Ok(res)
}
