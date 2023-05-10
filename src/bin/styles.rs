use rustcolor::{blink, danger, error, info, primary, printer::ColorPrinter, underline, warn};

fn main() {
    info!("this is the info style");
    primary!("this is the primary style");
    warn!("this is the warn style");
    danger!("this is the danger style");
    error!("this is the error style");
    underline!("this is the underlined style");
    blink!("this is the blink style");
}
