/*! # Rust Color
Rust Color is a terminal color rendering library,
thats supports 8/16 colors, 256 colors, RGB color
rendering output, compatible with windows.
*/
pub mod color16;

#[cfg(test)]
mod tests {
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
}
