use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::{env, time::Duration};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting global default");

    LogTracer::init()?;

    let url = &args[1];

    let options = LaunchOptionsBuilder::default()
        .enable_logging(true)
        .port(Some(8000))
        .idle_browser_timeout(Duration::from_secs(10))
        .build()
        .expect("Default should not panic");
    let browser = Browser::new(options)?;
    #[allow(deprecated)]
    let tab = browser.wait_for_initial_tab()?;
    let tab = tab.navigate_to(url)?.wait_until_navigated()?;
    println!("Page title: {}", tab.get_title()?);

    Ok(())
}
