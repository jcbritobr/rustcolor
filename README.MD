 # Rust Color
Rust Color is a terminal color rendering library,
thats supports 8/16 colors, 256 colors, RGB color
rendering output, compatible with windows.

Rust Color uses **ansi scaped sequences**. This [article](https://en.wikipedia.org/wiki/ANSI_escape_code) in
wikipedia explain how it works.

* How to install - Add the rustcolor crate to your Cargo.toml 
```
[dependencies]
rustcolor = {path = "../rustcolor"}
```

* How to use - The below code is a demo and prints basic/extended 3bit/4bit color and 8 bit color
```rust
use rustcolor::{color16::{self, ColorPrinter16}, color256::ColorPrinter256};

fn main() {

    println!("Prints all 16 color basic/extended and 256 colors\n");

    for i in 0..8 {
        let background = "this is a simple color text".to_owned().print_c16(
            if i == 0 {
                color16::Color16::usize_to_color(37)
            } else {
                color16::Color16::FgBlack
            },
            color16::Color16::usize_to_color(i + 40),
        );

        let bright_background = "this is a simple color text".to_owned().print_c16(
            color16::Color16::FgBlack,
            color16::Color16::usize_to_color(i + 100),
        );

        println!("{} {}", background, bright_background);
    }

    println!();

    for i in 0..16 {
        let bg_color = format!("  {:<4}", i).print_c256(0, i);
        print!("{}", bg_color);
    }

    println!();

    let mut counter: usize = 0;

    for i in 16..=231 {
        let bg_color = format!("  {:<4}", i).print_c256(0, i);
        if counter % 18 == 0 {
            println!();
        }
        print!("{}", bg_color);
        counter += 1;
    }

    println!();

    for i in 232..255 {
        let bg_color = format!("  {:<4}", i).print_c256(0, i);
        print!("{}", bg_color);
    }
}
```

![color-demo](thumbs/color-demo.png)