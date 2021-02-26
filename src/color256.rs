/*!
# color256
This module implements all 256 terminal color rendering operations,
color conversions, color name definitions and traits for rust color library.

### 8bit

As [256-color](https://en.wikipedia.org/wiki/8-bit_color) lookup tables became common on graphic cards, escape sequences were added to select from a pre-defined set of 256 colors

### Examples

* ESC[ 38;5;⟨n⟩ m Select foreground color
* ESC[ 48;5;⟨n⟩ m Select background color \
* ESC[ 38;5;⟨n1⟩;48;5;⟨n2⟩ m both foreground and background \
0 - 7:  standard colors (as in ESC [ 30–37 m) \
8 - 15:  high intensity colors (as in ESC [ 90–97 m) \
16 - 231:  6 × 6 × 6 cube (216 colors): 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5) \
232-255:  grayscale from black to white in 24 steps
*/

pub trait ColorPrinter256 {
    fn print_c256(&self, foreground: usize, background: usize) -> String;
}

impl ColorPrinter256 for String {
    fn print_c256(&self, foreground: usize, background: usize) -> String {
        let result = format!(
            "\u{001b}[38;5;{};48;5;{}m{}\u{001b}[0m",
            foreground, background, self
        );
        result
    }
}
