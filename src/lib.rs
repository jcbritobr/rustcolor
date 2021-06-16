/*!
# Rust Color
Rust Color is a terminal color rendering library,
thats supports 3/4 bit colors, 8 bit colors, 24 bit color
rendering output, compatible with windows.

## ANSI Escape Codes for Terminal Graphics
The ANSI escape code standard, formally adopted as ISO/IEC 6429, defines a series of control sequences.
Each control sequence begins with a **Control Sequence Introducer** (CSI), defined as a scape character
followed immediately by a bracket: **ESC[**.
*/
pub mod color;
pub mod printer;
pub mod style;

#[cfg(test)]
mod tests {
    use printer::*;
    use crate::color::*;

    use super::*;

    #[test]
    fn test_color16_printer() {
        let red_fg_text = "this is a red foreground color text"
            .to_owned()
            .print_c16(FG_RED, BG_BLACK);

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

    #[test]
    fn test_default_3bit_color() {
        let error_text = "white text with red bg".to_owned().error();
        let mut expected = "\u{001b}[37;41mwhite text with red bg\u{001b}[0m";
        assert_eq!(expected, error_text);

        let primary_text = "blue text with primary style".to_owned().primary();
        expected = "\u{001b}[34;49mblue text with primary style\u{001b}[0m";
        assert_eq!(expected, primary_text);

        let danger = "red text with danger style".to_owned().danger();
        expected = "\u{001b}[31;49mred text with danger style\u{001b}[0m";
        assert_eq!(expected, danger);

        let info_text = "green text with info style".to_owned().info();
        expected = "\u{001b}[32;49mgreen text with info style\u{001b}[0m";
        assert_eq!(expected, info_text);

        let warn_text = "yellow text with warn style".to_owned().warn();
        expected = "\u{001b}[33;49myellow text with warn style\u{001b}[0m";
        assert_eq!(expected, warn_text);

        let blink_text = "this is a text with blink style".to_owned().blink();
        expected = "\u{001b}[31;5mthis is a text with blink style\u{001b}[0m";
        assert_eq!(expected, blink_text);

        let underlined_text = "this is a text with underlined style".to_owned().underline();
        expected = "\u{001b}[33;4mthis is a text with underlined style\u{001b}[0m";
        assert_eq!(expected, underlined_text);
    }
}
