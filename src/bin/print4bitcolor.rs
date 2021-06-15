use rustcolor::printer::ColorPrinter;

fn main() {
    for i in 0..8 {
        let color_data = format!("  {:<4}", i);
        print!("{}", color_data.print_c16(90, i + 40));
    }

    println!();

    for i in 0..8 {
        let color_data = format!("  {:<4}", i);
        print!("{}", color_data.print_c16(30, i + 100));
    }

    println!();
}
