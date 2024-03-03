use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::copy;

use rand::Rng;
use reqwest::{Client, header};
use reqwest::header::{HeaderMap, HeaderValue};

const EMAIL_DOMAINS: [&str; 20] = ["gmail.com", "hotmail.com", "live.com", "yahoo.com", "icloud.com", "outlook.com", "protonmail.com",
    "tutanota.de", "tutanota.com", "tutamail.com", "tuta.io", "yandex.com", "sina.com", "qq.com",
    "naver.com", "163.com", "yeah.net", "126.com", "aliyun.com", "foxmail.com"];

fn get_chars() -> Vec<char> {
    let mut chars = vec![];
    for i in 'a'..='z' {
        chars.push(i);
        chars.push(i.to_ascii_uppercase())
    }
    for i in '0'..='9' {
        chars.push(i);
    }
    chars
}

pub fn get_random_username(min_length: u32, max_length: u32) -> String {
    let mut random_username = String::new();
    let mut rng = rand::thread_rng();
    let len: u32 = rng.gen_range(min_length..max_length);
    let chars = get_chars();
    for _ in 0..len {
        let idx = rng.gen_range(0..chars.len());
        random_username.push(chars[idx]);
    }
    random_username
}

pub fn get_random_email(prefix: &str) -> String {
    let mut random_email = prefix.to_string();
    random_email.push('@');
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..EMAIL_DOMAINS.len());
    random_email.push_str(EMAIL_DOMAINS[idx]);
    random_email
}

pub fn generate_http_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json; charset=utf-8"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers
}

pub fn cookies_to_string(cookies: &HashMap<String, String>) -> String {
    cookies
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("; ")
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

#[derive(Debug)]
pub struct Record {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl Record {
    pub fn new(username: String, password: String, email: String) -> Self {
        Record {
            username,
            password,
            email,
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "username: {}\npassword: {}\nemail: {}\n", self.username, self.password, self.email).expect("TODO: panic message"))
    }
}