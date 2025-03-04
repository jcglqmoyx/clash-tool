use rand::Rng;
use reqwest::{
    cookie::Cookie,
    header::{HeaderMap, HeaderValue},
};
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

const EMAIL_DOMAINS: [&str; 20] = [
    "gmail.com",
    "hotmail.com",
    "live.com",
    "yahoo.com",
    "icloud.com",
    "outlook.com",
    "protonmail.com",
    "tutanota.de",
    "tutanota.com",
    "tutamail.com",
    "tuta.io",
    "yandex.com",
    "sina.com",
    "qq.com",
    "naver.com",
    "163.com",
    "yeah.net",
    "126.com",
    "aliyun.com",
    "foxmail.com",
];

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
    let mut rng = rand::rng();
    let len: u32 = rng.random_range(min_length..max_length);
    let chars = get_chars();
    for _ in 0..len {
        let idx = rng.random_range(0..chars.len());
        random_username.push(chars[idx]);
    }
    random_username
}

pub fn get_random_email(prefix: &str) -> String {
    let mut random_email = prefix.to_string();
    random_email.push('@');
    let mut rng = rand::rng();
    let idx = rng.random_range(0..EMAIL_DOMAINS.len());
    random_email.push_str(EMAIL_DOMAINS[idx]);
    random_email
}

pub fn generate_http_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"));
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.9"),
    );
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers
}

pub fn parse_cookies(cookies: Vec<Cookie>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for cookie in cookies {
        map.insert(cookie.name().to_string(), cookie.value().to_string());
    }
    map
}
pub fn cookies_to_string(cookies: &HashMap<String, String>) -> String {
    cookies
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("; ")
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
        Ok(write!(
            f,
            "username: {}\npassword: {}\nemail: {}\n",
            self.username, self.password, self.email
        )
        .expect("TODO: panic message"))
    }
}
