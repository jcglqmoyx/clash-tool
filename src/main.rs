use std::{env, thread};
use std::time::Duration;

use chrono::Local;
use clipboard::{ClipboardContext, ClipboardProvider};
use enigo::{Button, Enigo, Key, Keyboard, Mouse, Settings};
use enigo::Coordinate::Abs;
use enigo::Direction::{Click, Press};
use fern::Dispatch;
use log::LevelFilter;

use clash_tool::{
    cyan,
    gou,
    mail_tm,
    panda,
    util,
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
    let mut clash_subscription_link: Option<String> = None;
    match option {
        "1" => {
            let record = util::Record::new(
                util::get_random_username(8, 10).to_string(),
                util::get_random_username(8, 10).to_string(),
                util::get_random_email(&util::get_random_username(8, 10)).to_string(),
            );
            log::info!("You chose to register a Cyanmori account.");
            cyan::register(&record).await?;
            let cookies = cyan::login(&record).await;
            let subscription_link = cyan::get_subscription_link(&cookies.unwrap()).await;
            clash_subscription_link = Option::from(subscription_link.clone().unwrap().to_string());
            // util::download_subscription_configuration_file(&subscription_link.unwrap()).await;
        }
        "2" => {
            log::info!("You chose to register a Panda account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            panda::verify(temp_email_account.address.clone()).await?;
            let verification_code = mail_tm::get_verification_code(temp_email_account.clone()).await?;
            panda::register(temp_email_account.clone(), verification_code).await?;
            clash_subscription_link = Option::from(panda::login(temp_email_account.clone()).await?);
        }
        "3" => {
            log::info!("You chose to register a 加速狗 account.");
            let temp_email_account = mail_tm::create_temp_mail_account().await?;
            gou::send_verification_code_to_email(temp_email_account.address.clone()).await?;
            let verification_code = mail_tm::get_verification_code(temp_email_account.clone()).await?;
            gou::register(temp_email_account.clone(), verification_code).await?;
            let cookies = gou::login(temp_email_account.clone()).await?;
            clash_subscription_link = Option::from(gou::get_subscription_link(&cookies).await?);
        }
        "h" => { print!("1: Cyanmori\n2: Panda\n3: 加速狗\nh: show help\n"); }
        _ => { println!("doing nothing"); }
    }
    match clash_subscription_link {
        Some(link) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(link).unwrap();
            match env::consts::OS {
                "macos" => {
                    let mut enigo = Enigo::new(&Settings::default()).unwrap();
                    enigo.move_mouse(1315, 20, Abs);
                    thread::sleep(Duration::from_millis(100));
                    enigo.button(Button::Left, Click);
                    thread::sleep(Duration::from_millis(100));
                    enigo.key(Key::Meta, Press).unwrap();
                    thread::sleep(Duration::from_millis(100));
                    enigo.key(Key::Unicode('M'), Press).unwrap();
                    thread::sleep(Duration::from_millis(100));

                    enigo.move_mouse(720, 470, Abs);
                    thread::sleep(Duration::from_millis(100));
                    enigo.button(Button::Left, Click);
                    thread::sleep(Duration::from_millis(500));
                    enigo.move_mouse(890, 370, Abs);
                    thread::sleep(Duration::from_millis(500));
                    enigo.button(Button::Left, Click);
                    thread::sleep(Duration::from_millis(500));
                    enigo.key(Key::Meta, Press).unwrap();
                    thread::sleep(Duration::from_millis(100));
                    enigo.key(Key::Unicode('V'), Click).unwrap();
                    thread::sleep(Duration::from_millis(100));
                    enigo.move_mouse(920, 440, Abs);
                    thread::sleep(Duration::from_millis(100));
                    enigo.button(Button::Left, Click);
                }
                _ => {}
            }
        }
        None => { println!("No subscription link found."); }
    }
    Ok(())
}
