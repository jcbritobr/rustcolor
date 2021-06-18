use std::usize;

use rustcolor::{color::*, printer::ColorPrinter};

macro_rules! print_color_pallette {
    ($background:expr, $offset:expr, $op:tt) => {
        for i in (C8_000..C8_006) {
            let color_data_0 = format!("  {:<4}", $offset.0 $op i);
            let color_data_1 = format!("  {:<4}", $offset.1 $op i);
            let color_data_2 = format!("  {:<4}", $offset.2 $op i);
            let color_data_3 = format!("  {:<4}", $offset.3 $op i);
            let color_data_4 = format!("  {:<4}", $offset.4 $op i);
            let color_data_5 = format!("  {:<4}", $offset.5 $op i);
            let color_data_6 = format!("  {:<4}", $offset.6 $op i);
            let color_data_7 = format!("  {:<4}", $offset.7 $op i);
            let color_data_8 = format!("  {:<4}", $offset.8 $op i);
            let color_data_9 = format!("  {:<4}", $offset.9 $op i);
            let color_data_10 = format!("  {:<4}", $offset.10 $op i);
            let color_data_11 = format!("  {:<4}", $offset.11 $op i);
    
            println!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                color_data_0.print_c256($background, $offset.0 $op i),
                color_data_1.print_c256($background, $offset.1 $op i),
                color_data_2.print_c256($background, $offset.2 $op i),
                color_data_3.print_c256($background, $offset.3 $op i),
                color_data_4.print_c256($background, $offset.4 $op i),
                color_data_5.print_c256($background, $offset.5 $op i),
                color_data_6.print_c256($background, $offset.6 $op i),
                color_data_7.print_c256($background, $offset.7 $op i),
                color_data_8.print_c256($background, $offset.8 $op i),
                color_data_9.print_c256($background, $offset.9 $op i),
                color_data_10.print_c256($background, $offset.10 $op i),
                color_data_11.print_c256($background, $offset.11 $op i),
            );
        }
    };
}

struct Offset(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

fn main() {
    let cold_a = Offset(16, 22, 28, 34, 40, 46, 82, 76, 70, 64, 58, 52);
    let cold_b = Offset(93, 99, 105, 111, 117, 123, 159, 153, 147, 141, 135, 129);
    let warm_a = Offset(160, 166, 172, 178, 184, 190, 226, 220, 214, 208, 202, 196);
    print_color_pallette!(C8_129, cold_a, +);
    print_color_pallette!(C8_129, cold_b, -);
    print_color_pallette!(C8_129, warm_a, +);
}