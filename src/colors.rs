/// An enumeration representing ANSI color codes for text styling.
pub enum AnsiColor {
    /// Cyan color.
    Cyan,
    /// Blue color.
    Blue,
    /// Yellow color.
    Yellow,
}

impl AnsiColor {
    /// Returns the ANSI escape code for the associated color.
    ///
    /// # Returns
    /// `&'static str` - The ANSI escape code for the color.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::AnsiColor;
    ///
    /// let cyan_code = AnsiColor::Cyan.escape_code();
    /// assert_eq!(cyan_code, "\u{001B}[36m");
    /// ```
    pub fn escape_code(&self) -> &'static str {
        match self {
            AnsiColor::Cyan => "\u{001B}[36m",
            AnsiColor::Blue => "\u{001B}[34m",
            AnsiColor::Yellow => "\u{001B}[33m",
        }
    }
}

/// A structure representing ANSI text styling.
pub struct AnsiStyle {
    /// A flag indicating whether text should be bold.
    pub bold: bool,
    /// An optional ANSI color for text styling.
    pub color: Option<AnsiColor>,
}

impl AnsiStyle {
    /// Returns the ANSI escape code for the associated text style.
    ///
    /// # Returns
    /// `String` - The ANSI escape code for the text style.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::{AnsiColor, AnsiStyle};
    ///
    /// let style = AnsiStyle { bold: true, color: Some(AnsiColor::Cyan) };
    /// let escape_code = style.escape_code();
    /// assert_eq!(escape_code, "\u{001B}[1m\u{001B}[36m");
    /// ```
    pub fn escape_code(&self) -> String {
        let mut code = String::new();

        if self.bold {
            code.push_str("\u{001B}[1m");
        }

        if let Some(color) = &self.color {
            code.push_str(color.escape_code());
        }

        code
    }

    /// Returns the ANSI escape code for resetting text styles.
    ///
    /// # Returns
    /// `&'static str` - The ANSI escape code for resetting text styles.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::AnsiStyle;
    ///
    /// let reset_code = AnsiStyle::reset_code();
    /// assert_eq!(reset_code, "\u{001B}[0m");
    /// ```
    pub fn reset_code() -> &'static str {
        "\u{001B}[0m"
    }
}
