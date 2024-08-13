use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};
use fern::Dispatch;
use log::LevelFilter;
use teloxide::{prelude::Requester, types::ChatId, Bot};

use clash_tool::{cyan, gou, mail_tm, panda, util, wall, xfx_ssr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("registration_result.log")?)
        .apply()?;

    let args: Vec<String> = env::args().collect();
    let option = if args.len() == 1 { "h" } else { &args[1] };
    let mut clash_subscription_link: Option<String> = None;
    match option {
        "1" => {
            let record = util::Record::new(
                util::get_random_username(8, 10).to_string(),
                util::get_random_username(8, 10).to_string(),
                util::get_random_email(&util::get_random_username(8, 10)).to_string(),
            );
            log::info!("You chose to register a 青森 account.");
            cyan::register(&record).await?;
            let cookies = cyan::login(&record).await;
            let subscription_link = cyan::get_subscription_link(&cookies.unwrap()).await;
            clash_subscription_link = Option::from(subscription_link.unwrap().to_string());
        }
        "2" => {
            log::info!("You chose to register a Panda account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            panda::verify(&temp_email_account.address).await?;
            let verification_code = mail_tm::get_verification_code(&temp_email_account).await?;
            panda::register(&temp_email_account, verification_code).await?;
            clash_subscription_link = Option::from(panda::login(&temp_email_account).await?);
        }
        "3" => {
            log::info!("You chose to register a 加速狗 account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            gou::send_verification_code_to_email(&temp_email_account.address).await?;
            let verification_code = mail_tm::get_verification_code(&temp_email_account).await?;
            gou::register(&temp_email_account, verification_code).await?;
            let cookies = gou::login(&temp_email_account).await?;
            clash_subscription_link = Option::from(gou::get_subscription_link(&cookies).await?);
        }
        "4" => {
            log::info!("You chose to register a 小飞侠SSR account.");
            let random_email_account = util::get_random_email(&util::get_random_username(8, 10)).to_string();
            xfx_ssr::register(&random_email_account).await?;
            clash_subscription_link = Option::from(xfx_ssr::login(&random_email_account).await?);
        }
        "5" => {
            log::info!("You chose to register a 墙了个墙 account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            wall::send_verification_code_to_email(&temp_email_account.address).await?;
            let verification_code = mail_tm::get_verification_code(&temp_email_account).await?;
            wall::register(&temp_email_account, verification_code).await?;
            let cookies = wall::login(&temp_email_account).await?;
            clash_subscription_link = Option::from(wall::get_subscription_link(&cookies).await?);
        }
        "h" => { print!("1: 青森\n2: Panda\n3: 加速狗\n4: 小飞侠SSR\n5: 墙了个墙\nh: show help\n"); }
        _ => { println!("doing nothing"); }
    }
    match clash_subscription_link {
        Some(ref link) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(link.to_string())?;
            match env::consts::OS {
                "macos" => {
                    let chat_id = ChatId(-1002092244317);

                    let bot = Bot::new(r#"6833152982:AAEh1LmvPwBzspY70aIHV817VGviA-Pl0pM"#);
                    bot.send_message(chat_id, clash_subscription_link.unwrap()).await.unwrap();
                }
                _ => {}
            }
        }
        None => {}
    }
    Ok(())
}