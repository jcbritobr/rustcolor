use rustcolor::{color::RGB, printer::ColorPrinter};

fn main() {
    let step = 12;
    println!("***RGB***");
    for i in (0..=255).step_by(step) {
        print!("{}", " ".print_24bit(RGB(0, 0, 0), RGB(i, 0, 0)));
    }
    println!();
    for i in (0..=255).step_by(step) {
        print!("{}", " ".print_24bit(RGB(0, 0, 0), RGB(0, i, 0)));
    }
    println!();
    for i in (0..=255).step_by(step) {
        print!("{}", " ".print_24bit(RGB(0, 0, 0), RGB(0, 0, i)));
    }
    println!("\n***CMY***");
    for i in (0..=255).step_by(step) {
        print!(
            "{}",
            " ".print_24bit(RGB(0, 0, 0), RGB(255 - i, 255 - 0, 255 - 0))
        );
    }
    println!();
    for i in (0..=255).step_by(step) {
        print!(
            "{}",
            " ".print_24bit(RGB(0, 0, 0), RGB(255 - 0, 255 - i, 255 - 0))
        );
    }
    println!();
    for i in (0..=255).step_by(step) {
        print!(
            "{}",
            " ".print_24bit(RGB(0, 0, 0), RGB(255 - 0, 255 - 0, 255 - i))
        );
    }
    println!();
}
