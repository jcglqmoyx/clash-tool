use std::collections::HashMap;
use std::fs::File;
use std::io::copy;

use reqwest::{
    Client,
    Error,
    header,
    Response,
};

use crate::api::cyan::*;
use crate::util::{log, Record};

fn cookies_to_string(cookies: &HashMap<String, String>) -> String {
    cookies
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("; ")
}

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

pub async fn register_cyan_account(record: &Record) -> Result<Response, Error> {
    println!("Registering cyan account..");
    let url = REGISTRATION_API;
    log(&record);
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
    println!("Response: {:?} {}", resp.version(), resp.status());
    println!("Headers: {:#?}\n", resp.headers());
    Ok(resp)
}

pub async fn login_cyan_account(record: &Record) -> Result<HashMap<String, String>, Error> {
    println!("Logging in Cyan account...");
    let params = [("email", &record.email), ("passwd", &record.password)];
    let response = Client::new().post(LOGIN_API).form(&params).send().await?;
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
    println!("cookies: {:#?}", cookies);
    Ok(cookies)
}

pub async fn get_subscription_link(cookies: &HashMap<String, String>) -> Result<String, ()> {
    println!("Getting subscription link...");
    let resp = Client::new()
        .get(USER_PROFILE_API)
        .header(header::COOKIE, cookies_to_string(cookies))
        .send()
        .await;
    let response_text = resp.unwrap().text().await;
    let string = response_text.unwrap_or(String::new());
    let substring = "index.oneclickImport(\'clash\',\'";
    match string.find(substring) {
        Some(index) => {
            let substring = string.get(index + substring.len()..).unwrap();
            let next_index = substring.find("\'");
            println!("Subscription link: {}", &substring[..next_index.unwrap()]);
            Ok(substring[..next_index.unwrap()].parse().unwrap())
        }
        None => {
            println!("Subscription link not found in the response text.");
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
    println!("Downloading configuration file...");
    let resp = reqwest::get(link).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let filename = get_file_name(link).await.unwrap();
    let path = get_subscription_file_destination() + &filename;
    let mut out = File::create(&path).expect("failed to create file");
    copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
    println!("Configuration file downloaded, path: {}", path);
}
