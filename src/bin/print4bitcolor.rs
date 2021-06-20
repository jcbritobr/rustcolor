use rustcolor::{color::{BG_BLACK, BG_DARK_GRAY, FG_BLACK, FG_DARK_GRAY}, printer::ColorPrinter};

fn main() {
    for i in 0..8 {
        let color_data = format!("  {:<4}", i);
        print!("{}", color_data.print_c16(FG_DARK_GRAY, i + BG_BLACK));
    }

    println!();

    for i in 0..8 {
        let color_data = format!("  {:<4}", i);
        print!("{}", color_data.print_c16(FG_BLACK, i + BG_DARK_GRAY));
    }

    println!();
}
