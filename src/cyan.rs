use std::{
    collections::HashMap,
    fs::File,
    io::copy,
};

use reqwest::{
    Client,
    Error,
    header,
    Response,
};

use crate::{
    api::cyan,
    util,
};


fn get_subscription_file_destination() -> String {
    let os_type = std::env::consts::OS;
    let home_dir = match home::home_dir() {
        Some(path) => {
            if !path.as_os_str().is_empty() {
                path.display().to_string()
            } else {
                "./".to_string()
            }
        }
        _ => "./".to_string(),
    };
    match os_type {
        "macos" => home_dir + "/.config/clash/",
        "linux" => home_dir + "/.config/clash/profiles/",
        "windows" => home_dir + "\\.config\\clash\\profiles",
        _ => "./".to_string(),
    }
}

pub async fn register(record: &util::Record) -> Result<Response, Error> {
    log::info!("Registering Cyanmori account..");
    let url = cyan::REGISTRATION_API;
    log::info!("{:#?}", &record);
    let resp = Client::new()
        .post(url)
        .form(&[
            ("name", record.username.clone()),
            ("email", record.email.clone()),
            ("passwd", record.password.clone()),
            ("repasswd", record.password.clone()),
        ])
        .send()
        .await?;
    log::info!("Register Cyanmori account response: {}", resp.status());
    Ok(resp)
}

pub async fn login(record: &util::Record) -> Result<HashMap<String, String>, Error> {
    log::info!("Logging into Cyanmori account...");
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
    let substring = "index.oneclickImport(\'clash\',\'";
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

async fn get_file_name(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let resp = client.head(url).send().await?;

    let content_disposition = if let Some(filename) = resp.headers().get(header::CONTENT_DISPOSITION) {
        filename.to_owned()
    } else {
        let resp = client.get(url).send().await?;
        resp.headers().get(header::CONTENT_DISPOSITION).ok_or("No Content-Disposition header found")?.to_owned()
    };

    let content_disposition_str = content_disposition.to_str()?;
    let file_name = content_disposition_str
        .split(';')
        .find_map(|part| {
            let part = part.trim();
            if part.starts_with("filename=") {
                Some(part.trim_start_matches("filename=").trim_matches('"'))
            } else {
                None
            }
        })
        .ok_or("Filename not found in Content-Disposition header")?;
    Ok(file_name.to_string())
}

pub async fn download_subscription_configuration_file(link: &str) {
    log::info!("Downloading configuration file...");
    let resp = reqwest::get(link).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let filename = get_file_name(link).await.unwrap();
    let path = get_subscription_file_destination() + &filename;
    let mut out = File::create(&path).expect("failed to create file");
    copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    log::info!("Configuration file downloaded, path: {}", path);
}
