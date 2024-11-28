use rand::Rng;
use reqwest::{cookie::Cookie, header::{HeaderMap, HeaderValue}};
use std::collections::HashMap;

fn get_chars() -> Vec<char> {
    let mut chars = vec![];
    for i in 'a'..='z' {
        chars.push(i);
    }
    for i in '0'..='9' {
        chars.push(i);
    }
    chars
}

pub fn get_random_username(min_length: u32, max_length: u32) -> String {
    let mut random_username = String::new();
    let mut rng = rand::thread_rng();
    let len: u32 = rng.gen_range(min_length..=max_length);
    let chars = get_chars();
    for _ in 0..len {
        let idx = rng.gen_range(0..chars.len());
        random_username.push(chars[idx]);
    }
    random_username
}

pub fn generate_http_request_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static(r#"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36"#));
    headers.insert("Content-Type", HeaderValue::from_static("application/json; charset=utf-8"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
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
