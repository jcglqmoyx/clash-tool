use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;

use rand::Rng;

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

pub fn get_random_username() -> String {
    let mut random_username = String::new();
    let mut rng = rand::thread_rng();
    let len: u32 = rng.gen_range(7..10);
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

pub fn log(record: &Record) {
    let mut file = File::create("result.txt").expect("Error occurred writing record");
    file.write_all(record.to_string().as_ref()).expect("TODO: panic message");
}
