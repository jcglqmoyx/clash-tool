use reqwest::{Client, Error};
use serde::Deserialize;

use crate::api::xfx_ssr;

pub async fn register(email_account: &str) -> Result<(), Error> {
    log::info!("Registering 小飞侠SSR account...");
    let url = xfx_ssr::REGISTRATION_API;
    log::info!("{:#?}", &email_account);
    let resp = Client::new()
        .post(url)
        .form(&[
            ("email", email_account),
            ("password", email_account),
            ("invite_code", ""),
            ("email_code", ""),
        ])
        .send()
        .await?;
    log::info!("Result: {:#?}", resp.status());
    Ok(())
}

pub async fn login(email_account: &str) -> Result<String, Error> {
    log::info!("Logging into 小飞侠SSR account...");
    let url = xfx_ssr::LOGIN_API;
    let resp = Client::new()
        .post(url)
        .form(&[
            ("email", email_account),
            ("password", email_account),
        ])
        .send()
        .await?;
    #[derive(Deserialize)]
    struct Root {
        data: Data,
    }

    #[derive(Deserialize)]
    struct Data {
        token: String,
    }
    let json_str = resp.text().await.unwrap();
    let parsed: Root = serde_json::from_str(&json_str).unwrap();
    println!("Authorization: {}", parsed.data.token);
    Ok(xfx_ssr::SUBSCRIPTION_LINK_PREFIX.to_owned() + &parsed.data.token)
}

