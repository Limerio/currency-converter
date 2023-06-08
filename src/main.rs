mod cli;
mod printer;
mod request;

use cli::cli;
use request::{get_exchange_rate, get_exchange_rate_all};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    let from = matches.get_one::<String>("from");
    let to = matches.get_one::<String>("to");
    let convert = matches.get_one::<String>("convert");
    let realtime = matches.get_one::<bool>("realtime").unwrap();

    if *realtime {
        if convert.is_some() {
            let convert: f64 = convert.unwrap().parse().unwrap();
            if convert.is_nan() {
                println!("You cannot convert and do realtime checking");
                std::process::exit(1);
            }
        }

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        ctrlc::set_handler(move || {
            println!("Exiting...");
            running_clone.store(false, Ordering::SeqCst);
        })
        .expect("Failed to register Ctrl+C handler");

        while running.load(Ordering::SeqCst) {
            clearscreen::clear().expect("failed to clear screen");

            let data = if from.is_none() && to.is_none() {
                get_exchange_rate_all().await.unwrap()
            } else {
                get_exchange_rate(
                    from.unwrap_or(&"".to_owned()).clone(),
                    to.unwrap_or(&"".to_owned()).clone(),
                )
                .await
                .unwrap()
            };

            printer::table_exchanges(data);

            sleep(Duration::from_secs(5)).await;
        }
    } else if convert.is_some() {
        let convert: f64 = convert.unwrap().parse().unwrap();
        let data = if from.is_none() && to.is_none() {
            get_exchange_rate_all().await.unwrap()
        } else {
            get_exchange_rate(
                from.unwrap_or(&"".to_owned()).clone(),
                to.unwrap_or(&"".to_owned()).clone(),
            )
            .await
            .unwrap()
        };

        printer::table_convertion(data, convert);
    } else {
        let data = if from.is_none() && to.is_none() {
            get_exchange_rate_all().await.unwrap()
        } else {
            get_exchange_rate(
                from.unwrap_or(&"".to_owned()).clone(),
                to.unwrap_or(&"".to_owned()).clone(),
            )
            .await
            .unwrap()
        };

        printer::table_exchanges(data);
    }
}
