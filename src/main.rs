use std::env;

use clash_tool::{gou, mail_tm};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let option = if args.len() == 1 { "h" } else { &args[1] };
    match option {
        "1" => {
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            gou::send_verification_code_to_email(&temp_email_account.address).await?;
            let verification_code = mail_tm::get_verification_code(&temp_email_account).await?;
            gou::register(&temp_email_account, verification_code).await?;
            let cookies = gou::login(&temp_email_account).await?;
            gou::get_subscription_link(&cookies).await?;
        }
        "h" => {
            print!("1: 加速狗\nh: show help\n");
        }
        _ => {
            println!("doing nothing");
        }
    }
    Ok(())
}
