/*!
This module implements all terminal color rendering operations,
color conversions, color name definitions and traits for rust color library.

### 3bit and 4bit

The original specification only had 8 colors. The **SGR** parameters 30-37 selects
the foreground color, while 40-37 selects the background. Few terminals implements brighter
color, providing 8 additional foreground and background colors.

### Examples

* to get black letters on white background - **ESC[30;47m**.
* to get brighter colors with black letters on white background - **ESC[90;107m**. (+60)
* to reset all attributes - **ESC[0m**.

### 8bit

As [256-color](https://en.wikipedia.org/wiki/8-bit_color) lookup tables became common on graphic cards, escape sequences were added to select from a pre-defined set of 256 colors

### Examples

* ESC[ 38;5;⟨n⟩m Select foreground color
* ESC[ 48;5;⟨n⟩m Select background color \
* ESC[ 38;5;⟨n1⟩;48;5;⟨n2⟩m both foreground and background \
0 - 7:  standard colors (as in ESC [ 30–37 m) \
8 - 15:  high intensity colors (as in ESC [ 90–97 m) \
16 - 231:  6 × 6 × 6 cube (216 colors): 16 + 36 × r + 6 × g + b (0 ≤ r, g, b ≤ 5) \
232-255:  grayscale from black to white in 24 steps
*/

use std::usize;

pub const FG_BLACK: usize = 30;
pub const FG_RED: usize = 31;
pub const FG_GREEN: usize = 32;
pub const FG_YELLOW: usize = 33;
pub const FG_BLUE: usize = 34;
pub const FG_MAGENTA: usize = 35;
pub const FG_CYAN: usize = 36;
pub const FG_WHITE: usize = 37;
pub const DEFAULT: usize = 39;

pub const BG_BLACK: usize = 40;
pub const BG_RED: usize = 41;
pub const BG_GREEN: usize = 42;
pub const BG_YELLOW: usize = 43;
pub const BG_BLUE: usize = 44;
pub const BG_MAGENTA: usize = 45;
pub const BG_CYAN: usize = 46;
pub const BG_WHITE: usize = 47;
pub const BG_DEFAULT: usize = 49;

pub const FG_DARK_GRAY: usize = 90;
pub const FG_LIGHT_RED: usize = 91;
pub const FG_LIGHT_GREEN: usize = 92;
pub const FG_LIGHT_YELLOW: usize = 93;
pub const FG_LIGHT_BLUE: usize = 94;
pub const FG_LIGHT_MAGENTA: usize = 95;
pub const FG_LIGHT_CYAN: usize = 96;
pub const FG_LIGHT_WHITE: usize = 97;

pub const BG_DARK_GRAY: usize = 100;
pub const BG_LIGHT_RED: usize = 101;
pub const BG_LIGHT_GREEN: usize = 102;
pub const BG_LIGH_YELLOW: usize = 103;
pub const BG_LIGHT_BLUE: usize = 104;
pub const BG_LIGHT_MAGENTA: usize = 105;
pub const BG_LIGHT_CYAN: usize = 106;
pub const BG_LIGHT_WHITE: usize = 107;

/// darken the color if it has 4bit
pub fn darken(color: usize) -> usize {
    color - 60
}

/// lighten the color if it has 4bit
pub fn lighten(color: usize) -> usize {
    color + 60
}