use std::{process, time::Duration};

use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{api::mail_tm, util::get_random_username};

fn find_substring_after_index(s: &str, substring: &str, start_index: usize) -> Option<usize> {
    if let Some(substring_index) = s[start_index..].find(substring) {
        Some(substring_index + start_index)
    } else {
        None
    }
}

async fn get_domain() -> Result<String, reqwest::Error> {
    let client = Client::new();
    match client.get(mail_tm::GET_DOMAIN_API).send().await {
        Ok(response) => match response.text().await {
            Ok(s) => {
                let p = "\"domain\":\"";
                match s.find(p) {
                    Some(index) => {
                        let next_index =
                            find_substring_after_index(s.as_str(), "\"", index + p.len()).unwrap();
                        let result = &s[index + p.len()..next_index];
                        Ok(result.parse().unwrap())
                    }
                    None => {
                        process::exit(1);
                    }
                }
            }
            Err(_) => {
                process::exit(1);
            }
        },
        Err(_) => {
            process::exit(1);
        }
    }
}

pub async fn create_temp_mail_account() -> Result<TempEmailAccount, reqwest::Error> {
    println!("Creating a temporary email account...");
    let domain = get_domain().await?;
    let username = get_random_username(8, 10);
    let address = format!("{}@{}", username, domain);
    let password = get_random_username(9, 10);
    Client::new()
        .post(mail_tm::CREATE_ACCOUNT_API)
        .json(&json!({"address": address, "password": password}))
        .send()
        .await?;
    println!(
        "Temporary email account created, address: {}, password: {}",
        address, password
    );
    Ok(TempEmailAccount::new(address.to_lowercase(), password))
}

fn extract_token_from_json(json_str: &str) -> Result<String, serde_json::Error> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Data {
        token: String,
    }
    let data: Data = serde_json::from_str(json_str)?;
    Ok(data.token)
}

fn extract_verification_code_from_json(json_str: &str) -> Result<String, serde_json::error::Error> {
    #[derive(Serialize, Deserialize, Debug)]
    struct MessageCollection {
        #[serde(rename = "hydra:member")]
        member: Vec<Message>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct Message {
        intro: String,
    }
    let parsed: MessageCollection = serde_json::from_str(json_str)?;
    if let Some(first_message) = parsed.member.first() {
        let regex = Regex::new(r"[0-9]{6}").unwrap();
        if let Some(caps) = regex.captures(&first_message.intro) {
            if let Some(code) = caps.get(0) {
                let code = code.as_str().to_string();
                return Ok(code.as_str().to_string());
            }
        }
    }
    Ok(String::from(""))
}

async fn get_token(account: &TempEmailAccount) -> Result<String, reqwest::Error> {
    let response = Client::new()
        .post(mail_tm::ACCESS_TOKEN_API)
        .json(&json!({"address": account.address,"password": account.password,}))
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(extract_token_from_json(&response_text).unwrap())
}

pub async fn get_verification_code(
    temp_email_account: &TempEmailAccount,
) -> Result<String, reqwest::Error> {
    println!("Getting verification code...");
    for _ in 0..600 {
        let token = get_token(&temp_email_account).await?;
        let response = Client::new()
            .get(mail_tm::GET_MESSAGE_API)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        let response_text = response.text().await?;
        let verification_code = extract_verification_code_from_json(&response_text).unwrap();
        if verification_code != String::from("") {
            println!("Verification code: {}", verification_code);
            return Ok(verification_code);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(String::from(""))
}

#[derive(Debug, PartialEq)]
pub struct TempEmailAccount {
    pub address: String,
    pub password: String,
}

impl TempEmailAccount {
    pub fn new(address: String, password: String) -> TempEmailAccount {
        TempEmailAccount { address, password }
    }
}
