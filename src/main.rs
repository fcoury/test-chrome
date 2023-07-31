use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];

    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Default should not panic");
    let browser = Browser::new(options)?;
    #[allow(deprecated)]
    let tab = browser.wait_for_initial_tab()?;
    let tab = tab.navigate_to(url)?.wait_until_navigated()?;
    println!("Page title: {}", tab.get_title()?);

    Ok(())
}
