use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};
use fern::Dispatch;
use log::LevelFilter;
use teloxide::{prelude::Requester, types::ChatId, Bot};

use clash_tool::{gou, mail_tm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("clash.log")?)
        .apply()?;

    let args: Vec<String> = env::args().collect();
    let option = if args.len() == 1 { "h" } else { &args[1] };
    let mut clash_subscription_link: Option<String> = None;
    match option {
        "1" => {
            log::info!("You chose to register a 加速狗 account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            gou::send_verification_code_to_email(&temp_email_account.address).await?;
            let verification_code = mail_tm::get_verification_code(&temp_email_account).await?;
            gou::register(&temp_email_account, verification_code).await?;
            let cookies = gou::login(&temp_email_account).await?;
            clash_subscription_link = Option::from(gou::get_subscription_link(&cookies).await?);
        }
        "h" => {
            print!("1: 加速狗\nh: show help\n");
        }
        _ => {
            println!("doing nothing");
        }
    }
    match clash_subscription_link {
        Some(ref link) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new()?;
            ctx.set_contents(link.to_string())?;
            match env::consts::OS {
                "macos" => {
                    let chat_id = ChatId(-4723827575);
                    let bot = Bot::new(r#"7618630537:AAEyQ_WTF-OXM267x-MMQsLmLNWjda9MIRA"#);
                    bot.send_message(chat_id, clash_subscription_link.unwrap())
                        .await
                        .unwrap();
                }
                _ => {}
            }
        }
        None => {}
    }
    Ok(())
}
