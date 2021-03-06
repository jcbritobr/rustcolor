/*!
# printer
This modules implements all functionalities to print fomatted ansi scape text, as
predefined styles. Thre trait ColorPrinter enhances the String type, adding new functions in it.
 */

use std::usize;

use crate::color::*;
use crate::style::StyleBuilder;

/// ColorPrinter is a trait thats enhances String data type with print_c16 and print_c256 functions.
/// function.
pub trait ColorPrinter {
    /// Enhance the given string with 16 color ansi scaped sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    /// use rustcolor::color::*;
    ///
    /// let red_fg_text = "this is a red foreground color text"
    ///     .to_owned()
    ///     .print_c16(FG_RED, BG_BLACK);
    ///
    /// assert_eq!(
    ///     "\u{001b}[31;40mthis is a red foreground color text\u{001b}[0m",
    ///      red_fg_text
    ///  );
    /// ```
    fn print_c16(&self, foreground: usize, background: usize) -> String;

    /// Enhance the given string with 256 color ansi scaped sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// let red_fg_text = "this is a red foreground color text"
    /// .to_owned()
    /// .print_c256(1, 0);
    ///
    /// let expected = "\u{001b}[38;5;1;48;5;0mthis is a red foreground color text\u{001b}[0m";
    /// assert_eq!(expected, red_fg_text);
    /// ```
    fn print_c256(&self, foreground: usize, background: usize) -> String;

    /// Enhance the given string with a yellow fg, default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the warn style".to_owned().warn());
    /// ```
    fn warn(&self) -> String;

    /// Enhance the given string with a white fg, red bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the error style".to_owned().error());
    /// ```
    fn error(&self) -> String;

    /// Enhance the given string with a red fg, default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the danger style".to_owned().danger());
    /// ```
    fn danger(&self) -> String;

    /// Enhance the given string with a green fg, default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the info style".to_owned().info());
    /// ```
    fn info(&self) -> String;

    /// Enhance the given string with a blue fg, default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the primary style".to_owned().primary());
    /// ```
    fn primary(&self) -> String;

    /// Enhance the given string with a red fg, blink tag and default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the blink style".to_owned().blink());
    /// ```
    fn blink(&self) -> String;

    /// Enhance the given string with a yellow fg, underline tag and default bg color text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustcolor::printer::*;
    ///
    /// println!("{}", "this is the underlined style".to_owned().underline());
    /// ```
    fn underline(&self) -> String;
}

impl ColorPrinter for String {
    fn print_c16(&self, foreground: usize, background: usize) -> String {
        let result = StyleBuilder::new()
            .csi()
            .color(foreground)
            .delimiter()
            .color(background)
            .end_sgr()
            .message(self)
            .csi()
            .reset()
            .end_sgr()
            .build();

        result
    }

    fn print_c256(&self, foreground: usize, background: usize) -> String {
        let result = StyleBuilder::new()
            .csi()
            .foreground_8bit()
            .delimiter()
            .color(foreground)
            .delimiter()
            .background_8bit()
            .delimiter()
            .color(background)
            .end_sgr()
            .message(self)
            .csi()
            .reset()
            .end_sgr()
            .build();

        result
    }

    fn error(&self) -> String {
        let result = self.print_c16(FG_WHITE, BG_RED);
        result
    }

    fn danger(&self) -> String {
        let result = self.print_c16(FG_RED, BG_DEFAULT);
        result
    }

    fn info(&self) -> String {
        let result = self.print_c16(FG_GREEN, BG_DEFAULT);
        result
    }

    fn primary(&self) -> String {
        let result = self.print_c16(FG_BLUE, BG_DEFAULT);
        result
    }

    fn warn(&self) -> String {
        let result = self.print_c16(FG_YELLOW, BG_DEFAULT);
        result
    }

    fn blink(&self) -> String {
        let result = StyleBuilder::new()
            .csi()
            .color(FG_RED)
            .delimiter()
            .blink()
            .end_sgr()
            .message(self)
            .csi()
            .reset()
            .end_sgr()
            .build();

        result
    }

    fn underline(&self) -> String {
        let result = StyleBuilder::new()
            .csi()
            .color(FG_YELLOW)
            .delimiter()
            .underline()
            .end_sgr()
            .message(self)
            .csi()
            .reset()
            .end_sgr()
            .build();

        result
    }
}
