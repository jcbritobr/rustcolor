/*!
# color
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

/// Implements 16 color rendering operations and describes 16 color enumerations.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color16 {
    ColorNone = -1,
    FgBlack = 30,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyan,
    FgWhite,
    FgDefault = 39,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgDefault = 49,
    FgDarkGray = 90,
    FgLightRed,
    FgLightGreen,
    FgLightYellow,
    FgLightBlue,
    FgLightMagenta,
    FgLightCyan,
    FgLightWhite,
    BgDarkGray = 100,
    BgLightRed,
    BgLightGreen,
    BgLightYellow,
    BgLightBlue,
    BgLightMagenta,
    BgLightCyan,
    BgLightWhite,
}

impl Color16 {
    /// Converts a color enumeration to usize.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let bg_blue = Color16::BgBlue;
    /// let result = Color16::color_to_usize(bg_blue);
    /// assert_eq!(44, result);
    /// ```
    pub fn color_to_usize(color16: Color16) -> usize {
        color16 as usize
    }

    /// Converts an usize color representation to its enumeraion.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let bg_blue = Color16::usize_to_color(44);
    /// let expected = Color16::BgBlue;
    /// assert_eq!(expected, bg_blue);
    /// ```
    pub fn usize_to_color(color16: usize) -> Color16 {
        match color16 {
            30 => Self::FgBlack,
            31 => Self::FgRed,
            32 => Self::FgGreen,
            33 => Self::FgYellow,
            34 => Self::FgBlue,
            35 => Self::FgMagenta,
            36 => Self::FgCyan,
            37 => Self::FgWhite,
            39 => Self::FgDefault,
            40 => Self::BgBlack,
            41 => Self::BgRed,
            42 => Self::BgGreen,
            43 => Self::BgYellow,
            44 => Self::BgBlue,
            45 => Self::BgMagenta,
            46 => Self::BgCyan,
            47 => Self::BgWhite,
            49 => Self::BgDefault,
            90 => Self::FgDarkGray,
            91 => Self::FgLightRed,
            92 => Self::FgLightGreen,
            93 => Self::FgLightYellow,
            94 => Self::FgLightBlue,
            95 => Self::FgLightMagenta,
            96 => Self::FgLightCyan,
            97 => Self::FgLightWhite,
            100 => Self::BgDarkGray,
            101 => Self::BgLightRed,
            102 => Self::BgLightGreen,
            103 => Self::BgLightYellow,
            104 => Self::BgLightBlue,
            105 => Self::BgLightMagenta,
            106 => Self::BgLightCyan,
            107 => Self::BgLightWhite,
            _ => Self::ColorNone,
        }
    }

    /// Lighten the given color to its bright version.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let fg_blue = Color16::FgBlue;
    /// let fg_light_blue = fg_blue.lighten();
    /// assert_eq!(Color16::FgLightBlue, fg_light_blue);
    /// ```
    pub fn lighten(&self) -> Self {
        let val = *self as usize;
        if val >= 30 && val <= 47 {
            return Self::usize_to_color(val + 60);
        }

        *self
    }

    /// Darken the given color to its dark version.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let fg_light_red = Color16::FgLightRed;
    /// let fg_red = fg_light_red.darken();
    /// assert_eq!(Color16::FgRed, fg_red);
    /// ```
    pub fn darken(&self) -> Self {
        let val = *self as usize;
        if val >= 90 && val <= 107 {
            return Self::usize_to_color(val - 60);
        }

        *self
    }
}

/// ColorPrinter is a trait thats enhances String data type with print_c16 and print_c256 functions.
/// function.
pub trait ColorPrinter {
    /// Enhance the given string with 16 color ansi scaped sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let red_fg_text = "this is a red foreground color text"
    ///     .to_owned()
    ///     .print_c16(Color16::FgRed, Color16::BgBlack);
    ///
    /// assert_eq!(
    ///     "\u{001b}[31;40mthis is a red foreground color text\u{001b}[0m",
    ///      red_fg_text
    ///  );
    /// ```
    fn print_c16(&self, foreground: Color16, background: Color16) -> String;

    /// Enhance the given string with 256 color ansi scaped sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::color::*;
    ///
    /// let red_fg_text = "this is a red foreground color text"
    /// .to_owned()
    /// .print_c256(1, 0);
    ///
    /// let expected = "\u{001b}[38;5;1;48;5;0mthis is a red foreground color text\u{001b}[0m";
    /// assert_eq!(expected, red_fg_text);
    /// ```
    fn print_c256(&self, foreground: usize, background: usize) -> String;

    /// Enhance the given string with a white fg, red bg color text.
    fn error(&self) -> String;

    /// Enhance the given string with a red fg, default bg color text.
    fn danger(&self) -> String;

    /// Enhance the given string with a green fg, default bg color text.
    fn info(&self) -> String;

    /// Enhance the given string with a blue fg, default bg color text.
    fn primary(&self) -> String;
}

impl ColorPrinter for String {
    fn print_c16(&self, foreground: Color16, background: Color16) -> String {
        let result = format!(
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Color16::color_to_usize(foreground),
            Color16::color_to_usize(background),
            self
        );
        result
    }

    fn print_c256(&self, foreground: usize, background: usize) -> String {
        let result = format!(
            "\u{001b}[38;5;{};48;5;{}m{}\u{001b}[0m",
            foreground, background, self
        );
        result
    }

    fn error(&self) -> String {
        let result = self.print_c16(Color16::FgWhite, Color16::BgRed);
        result
    }

    fn danger(&self) -> String {
        let result = self.print_c16(Color16::FgRed, Color16::BgDefault);
        result
    }

    fn info(&self) -> String {
        let result = self.print_c16(Color16::FgGreen, Color16::BgDefault);
        result
    }

    fn primary(&self) -> String {
        let result = self.print_c16(Color16::FgBlue, Color16::BgDefault);
        result
    }
}
