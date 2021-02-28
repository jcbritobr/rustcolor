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