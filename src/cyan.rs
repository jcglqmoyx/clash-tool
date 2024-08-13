use std::collections::HashMap;

use reqwest::{
    header,
    Client,
    Error,
    Response,
};

use crate::{
    api::cyan,
    util,
};

pub async fn register(record: &util::Record) -> Result<Response, Error> {
    log::info!("Registering 青森 account..");
    let url = cyan::REGISTRATION_API;
    log::info!("{:#?}", &record);
    let resp = Client::new()
        .post(url)
        .form(&[
            ("name", &record.username),
            ("email", &record.email),
            ("passwd", &record.password),
            (r#"repasswd"#, &record.password),
        ])
        .send()
        .await?;
    log::info!("Register 青森 account response: {}", resp.status());
    Ok(resp)
}

pub async fn login(record: &util::Record) -> Result<HashMap<String, String>, Error> {
    log::info!("Logging into 青森 account...");
    let params = [("email", &record.email), ("passwd", &record.password)];
    let response = Client::new().post(cyan::LOGIN_API).form(&params).send().await?;
    let headers = response.headers();
    let cookies = headers.get_all("set-cookie").iter().flat_map(|value| {
        value.to_str().ok().and_then(|s| {
            let cookie_parts: Vec<&str> = s.split(';').collect();
            if let Some(first_part) = cookie_parts.first() {
                let key_value: Vec<&str> = first_part.split('=').collect();
                if key_value.len() == 2 {
                    Some((key_value[0].trim().to_string(), key_value[1].trim().to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }).collect::<HashMap<_, _>>();
    log::info!("Cookies: {:#?}", cookies);
    Ok(cookies)
}

pub async fn get_subscription_link(cookies: &HashMap<String, String>) -> Result<String, ()> {
    log::info!("Getting subscription link...");
    let resp = Client::new()
        .get(cyan::USER_PROFILE_API)
        .header(header::COOKIE, util::cookies_to_string(cookies))
        .send()
        .await;
    let response_text = resp.unwrap().text().await;
    let string = response_text.unwrap_or(String::new());
    let substring = r#"index.oneclickImport(\'clash\',\'"#;
    match string.find(substring) {
        Some(index) => {
            let substring = string.get(index + substring.len()..).unwrap();
            let next_index = substring.find("\'");
            log::info!("Subscription link: {}", &substring[..next_index.unwrap()]);
            Ok(substring[..next_index.unwrap()].parse().unwrap())
        }
        None => {
            log::info!("Subscription link not found in the response text.");
            Ok("Failed.".parse().unwrap())
        }
    }
}
