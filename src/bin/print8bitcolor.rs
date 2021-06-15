use rustcolor::printer::ColorPrinter;

fn main() {
    for i in 0..256 {
        let color_data = format!("  {:<4}", i);
        if i % 16 == 0 {
            println!();
        }
        print!("{}", color_data.print_c256(90, i));
    }

    println!()
}
