use std::env;

use fern::Dispatch;
use log::LevelFilter;

use clash_tool::cyan::{
    download_subscription_configuration_file,
    get_subscription_link,
    login_cyan_account,
    register_cyan_account,
};
use clash_tool::mail_tm::{
    create_temp_mail_account,
    get_verification_code,
};
use clash_tool::panda::{
    login_panda_node_account,
    register_panda_node_account,
    send_verification_code_to_email,
};
use clash_tool::util::{
    get_random_email,
    get_random_username,
    Record,
};

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
    let record = Record::new(
        get_random_username().to_string(),
        get_random_username().to_string(),
        get_random_email(&get_random_username()).to_string(),
    );
    match option {
        "1" => {
            log::info!("You chose to register a Cyanmori account.");
            register_cyan_account(&record).await?;
            let cookies = login_cyan_account(&record).await;
            let subscription_link = get_subscription_link(&cookies.unwrap()).await;
            download_subscription_configuration_file(&subscription_link.unwrap()).await;
        }
        "2" => {
            log::info!("You chose to register a Panda account.");
            let temp_email_account = create_temp_mail_account().await?;
            send_verification_code_to_email(temp_email_account.address.clone()).await?;
            let verification_code = get_verification_code(temp_email_account.clone()).await?;
            register_panda_node_account(temp_email_account.clone(), verification_code).await?;
            login_panda_node_account(temp_email_account.clone()).await?;
        }
        "h" => { print!("1: Cyanmori\n2: Panda\nh: show help\n"); }
        _ => { println!("doing nothing"); }
    }
    Ok(())
}
