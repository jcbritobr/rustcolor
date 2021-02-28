/*! 
# style
This module has a builder thats helps to easily build syles.
Thre are rules that **must be followed** to build a style:
* All styles starts with a special byte.
* All styles ends with a special byte plus reset and eos(end of style) tag.
* All styles may have a background8bit tag in any order, but followed by color tag and a required(if background is the last tag) eos tag
* All styles may have a foreground8bit tag in any order, but followed by color tag and a required(if foreground is the last tag) eos tag

# Examples

* The below example builds a red fg default bg style.
```
use rustcolor::style::StyleBuilder;

let expected =  "\u{001b}[31;40mthis is a red foreground color text\u{001b}[0m";
let result = StyleBuilder::new()
    .special()
    .color(31)
    .separator()
    .color(40)
    .end_style()
    .message("this is a red foreground color text")
    .special()
    .reset()
    .end_style()
    .build();

assert_eq!(expected, expected);
```
 */
const SPECIAL: &str = "\u{001b}[";
const BACKGROUND_8BIT: &str = "48;5";
const FOREGROUND_8BIT: &str = "38;5";
const END_STYLE: char = 'm';
const RESET: char = '0';
const SEPARATOR: char = ';';

/// Implements a style builder pattern thats helps to build styles.
pub struct StyleBuilder {
    message: String,
}

impl StyleBuilder {

    /// Creates a new builder and initializes a new message.
    pub fn new() -> Self {
        Self {
            message: "".to_owned(),
        }
    }

    /// Inserts the special byte ESC[ to the style.
    pub fn special(mut self) -> StyleBuilder {
        self.message.push_str(SPECIAL);
        self
    }

    /// Inserts the tag 48;5 (background) to the style.
    pub fn backgound_8bit(mut self) -> StyleBuilder {
        self.message.push_str(BACKGROUND_8BIT);
        self
    }

    /// Inserts the color to the style.
    pub fn color(mut self, color: usize) -> StyleBuilder {
        self.message.push_str(color.to_string().as_ref());
        self
    }


    /// Inserts the tag 18;5 (foreground) to the style.
    pub fn foreground_8bit(mut self) -> StyleBuilder {
        self.message.push_str(FOREGROUND_8BIT);
        self
    }

    /// Inserts the tag 0 (reset) to the style.
    pub fn reset(mut self) -> StyleBuilder {
        self.message.push(RESET);
        self
    }

    /// Inserts the tag ; (separator) to the style
    pub fn separator(mut self) -> StyleBuilder {
        self.message.push(SEPARATOR);
        self
    }

    /// Inserts the tag m (end of style) to the style.
    pub fn end_style(mut self) -> StyleBuilder {
        self.message.push(END_STYLE);
        self
    }

    /// Inserts the message that will be printed to style.
    pub fn message(mut self, message: &str) -> StyleBuilder {
        self.message.push_str(message);
        self
    }

    /// Build the style.
    pub fn build(self) -> String {
        self.message
    }
}
