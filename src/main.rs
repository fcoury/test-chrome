use headless_chrome::{Browser, LaunchOptionsBuilder};

fn main() -> anyhow::Result<()> {
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Default should not panic");
    let browser = Browser::new(options)?;
    #[allow(deprecated)]
    let tab = browser.wait_for_initial_tab()?;
    let tab = tab
        .navigate_to("https://google.com")?
        .wait_until_navigated()?;
    println!("Page title: {}", tab.get_title()?);

    Ok(())
}
