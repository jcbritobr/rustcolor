/*! # Rust Color
Rust Color is a terminal color rendering library,
thats supports 8/16 colors, 256 colors, RGB color
rendering output, compatible with windows.

Rust Color uses **ansi scaped sequences**. This [article](https://en.wikipedia.org/wiki/ANSI_escape_code) in
wikipedia explain how it works.
*/
pub mod color16;
pub mod color256;

#[cfg(test)]
mod tests {
    use color16::*;
    use color256::*;
    use super::*;
    #[test]
    fn test_color16_enum_conversion_to_usize() {
        let mut expected = 31;
        let mut got = color16::Color16::color_to_usize(color16::Color16::FgRed);
        assert_eq!(expected, got);
        expected = 41;
        got = color16::Color16::color_to_usize(color16::Color16::BgRed);
        assert_eq!(expected, got)
    }

    #[test]
    fn test_color16_usize_conversion_to_enum() {
        let mut fg_expected = color16::Color16::FgRed;
        let mut fg_got = color16::Color16::usize_to_color(31);
        assert_eq!(fg_expected, fg_got);
        fg_expected = color16::Color16::FgLightBlue;
        fg_got = color16::Color16::usize_to_color(94);
        assert_eq!(fg_expected, fg_got);
    }

    #[test]
    fn test_color16_lighten() {
        let fg_red = color16::Color16::FgRed;
        let fg_light_red = fg_red.lighten();
        assert_eq!(color16::Color16::FgLightRed, fg_light_red);
        let fg_blue = color16::Color16::FgBlue;
        let fg_light_blue = fg_blue.lighten();
        assert_eq!(color16::Color16::FgLightBlue, fg_light_blue);
    }

    #[test]
    fn test_color16_darken() {
        let fg_light_red = color16::Color16::FgLightRed;
        let fg_red = fg_light_red.darken();
        assert_eq!(color16::Color16::FgRed, fg_red);
    }

    #[test]
    fn test_color16_printer() {
        let red_fg_text = "this is a red foreground color text"
            .to_owned()
            .print_c16(color16::Color16::FgRed, color16::Color16::BgBlack);

        assert_eq!(
            "\u{001b}[31;40mthis is a red foreground color text\u{001b}[0m",
            red_fg_text
        );
    }

    #[test]
    fn test_color256_printer() {
        let red_fg_text = "this is a red foreground color text"
        .to_owned()
        .print_c256(1, 0);

        let expected = "\u{001b}[38;5;1;48;5;0mthis is a red foreground color text\u{001b}[0m";
        assert_eq!(expected, red_fg_text);
    }
}