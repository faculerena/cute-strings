#[cfg(feature = "regex")]
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "regex")]
use thiserror::Error;

use std::fmt::Display;

#[cfg(feature = "regex")]
#[derive(Error, Debug)]
pub enum CuteStringError {
    #[error("regex parse error: {0}")]
    RegexError(#[from] regex::Error),
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AsciiColor {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Orange,
    Pink,
    Teal,
    Black,
    End,
}

impl Display for AsciiColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "\x1b[30m"),
            Self::Red => write!(f, "\x1b[31m"),
            Self::Green => write!(f, "\x1b[32m"),
            Self::Yellow => write!(f, "\x1b[33m"),
            Self::Blue => write!(f, "\x1b[34m"),
            Self::Purple => write!(f, "\x1b[35m"),
            Self::Cyan => write!(f, "\x1b[36m"),
            Self::White => write!(f, "\x1b[37m"),
            Self::Orange => write!(f, "\x1b[38;5;208m"),
            Self::Pink => write!(f, "\x1b[38;5;205m"),
            Self::Teal => write!(f, "\x1b[38;5;51m"),
            Self::End => write!(f, "\x1b[0m"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ColoredRange {
    start: usize,
    end: usize,
    color: AsciiColor,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Underline {
    start: usize,
    end: usize,
    color: AsciiColor,
    character: char,
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CuteString {
    string: Option<String>,
    default_color: Option<AsciiColor>,
    colored_ranges: Vec<ColoredRange>,
    underlines: Vec<Underline>,
}

impl CuteString {
    /// Create a new instance of CuteString
    pub const fn new() -> Self {
        Self {
            string: None,
            default_color: None,
            colored_ranges: Vec::new(),
            underlines: Vec::new(),
        }
    }

    /// Return the string to be colored
    pub fn get_string(&self) -> Option<String> {
        self.string.clone()
    }

    /// Set the `string` to be saved for coloring
    pub fn set_string(&mut self, string: String) -> &mut Self {
        self.string = Some(string);
        self
    }

    /// Set `color` as the default for the string. If no color is set, the default color is white.
    pub fn set_default_color(&mut self, color: AsciiColor) -> &mut Self {
        self.default_color = Some(color);
        self
    }

    /// Color all characters in the range [`start`, `end`) with the specified `color`.
    pub fn color_range(&mut self, start: usize, end: usize, color: AsciiColor) -> &mut Self {
        self.colored_ranges.push(ColoredRange { start, end, color });
        self
    }

    /// Color all groups that matches the `pattern` passed with the given `color`.
    #[cfg(feature = "regex")]
    pub fn color_regex(
        &mut self,
        pattern: &str,
        color: AsciiColor,
    ) -> Result<&mut Self, CuteStringError> {
        let re = Regex::new(pattern);

        if let Err(e) = re {
            return Err(CuteStringError::RegexError(e));
        }

        let re = re.unwrap();

        if let Some(error) = &self.string {
            let error_str: &str = error;
            for m in re.find_iter(error_str) {
                self.colored_ranges.push(ColoredRange {
                    start: m.start(),
                    end: m.end(),
                    color,
                });
            }
        }
        Ok(self)
    }

    /// Color all occurrences of the character `ch` passed with the given `color`.
    pub fn color_char(&mut self, ch: char, color: AsciiColor) -> &mut Self {
        if let Some(error) = &self.string {
            let error_str: &str = error;
            for (i, c) in error_str.char_indices() {
                if c == ch {
                    self.colored_ranges.push(ColoredRange {
                        start: i,
                        end: i + c.len_utf8(),
                        color,
                    });
                }
            }
        }
        self
    }

    /// Colors the specified `line` with the specified `color`.
    pub fn color_line(&mut self, line: usize, color: AsciiColor) -> &mut Self {
        if let Some(error) = &self.string {
            let error_str: &str = error;
            let lines: Vec<&str> = error_str.lines().collect();
            if line > 0 && line <= lines.len() {
                let start = lines.iter().take(line - 1).map(|l| l.len() + 1).sum();
                let end = start + lines[line - 1].len();
                self.colored_ranges.push(ColoredRange { start, end, color });
            }
        }
        self
    }

    /// Deletes all ranges already set.
    pub fn reset_colors(&mut self) -> &mut Self {
        self.colored_ranges.clear();
        self.default_color = None;
        self
    }

    /// Colors all digits in the string with a different color (same digit is always of the same color).
    pub fn color_digits(&mut self) -> &mut Self {
        if let Some(error) = &self.string {
            let error_str: &str = error;
            let digit_colors = [
                AsciiColor::White,
                AsciiColor::Red,
                AsciiColor::Yellow,
                AsciiColor::Green,
                AsciiColor::Blue,
                AsciiColor::Purple,
                AsciiColor::Cyan,
                AsciiColor::Orange,
                AsciiColor::Pink,
                AsciiColor::Teal,
            ];

            for (i, c) in error_str.char_indices() {
                if let Some(digit) = c.to_digit(10) {
                    self.colored_ranges.push(ColoredRange {
                        start: i,
                        end: i + 1,
                        color: digit_colors[digit as usize],
                    });
                }
            }
        }
        self
    }

    /// Underlines the specified range [`start`, `end`) with the specified `character` colored with `color`.
    pub fn underline_with(
        &mut self,
        start: usize,
        end: usize,
        color: AsciiColor,
        character: char,
    ) -> &mut Self {
        self.underlines.push(Underline {
            start,
            end,
            color,
            character,
        });
        self
    }

    /// Underlines all the groups that match the given `pattern` with the specified `character` colored with `color`.

    #[cfg(feature = "regex")]
    pub fn underline_regex(
        &mut self,
        pattern: &str,
        color: AsciiColor,
        character: char,
    ) -> Result<&mut Self, CuteStringError> {
        let re = Regex::new(pattern);

        if let Err(e) = re {
            return Err(CuteStringError::RegexError(e));
        }

        let re = re.unwrap();

        if let Some(error) = &self.string {
            let error_str: &str = error;
            for m in re.find_iter(error_str) {
                self.underlines.push(Underline {
                    start: m.start(),
                    end: m.end(),
                    color,
                    character,
                });
            }
        }
        Ok(self)
    }
}

impl Display for CuteString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(error) = &self.string {
            let error_str: &str = error;
            let lines: Vec<&str> = error_str.lines().collect();

            let mut line_start = 0;
            for &line in lines.iter() {
                let line_end = line_start + line.len();

                let colored_chars: Vec<(char, Option<&AsciiColor>)> = line
                    .char_indices()
                    .map(|(i, c)| {
                        let color = self
                            .colored_ranges
                            .iter()
                            .find(|range| {
                                i + line_start >= range.start && i + line_start < range.end
                            })
                            .map(|range| &range.color);
                        (c, color)
                    })
                    .collect();

                if let Some(default_color) = &self.default_color {
                    write!(f, "{}", default_color)?;
                }

                let mut current_color: Option<&AsciiColor> = None;
                for (char, color) in colored_chars.iter() {
                    if current_color != *color {
                        if let Some(new_color) = color {
                            write!(f, "{}", new_color)?;
                        } else if self.default_color.is_some() {
                            write!(f, "{}", self.default_color.as_ref().unwrap())?;
                        } else {
                            write!(f, "{}", AsciiColor::End)?;
                        }
                        current_color = *color;
                    }
                    write!(f, "{}", char)?;
                }
                writeln!(f, "{}", AsciiColor::End)?;

                // Underlines for this line
                let mut underline = String::new();
                let mut last_color: Option<&AsciiColor> = None;
                for i in line_start..line_end {
                    let underline_char = self
                        .underlines
                        .iter()
                        .find(|u| i >= u.start && i < u.end)
                        .map_or(' ', |u| u.character);

                    let underline_color = self
                        .underlines
                        .iter()
                        .find(|u| i >= u.start && i < u.end)
                        .map(|u| &u.color);

                    if underline_color != last_color {
                        if let Some(color) = underline_color {
                            underline.push_str(&format!("{}", color));
                        } else {
                            underline.push_str(&format!("{}", AsciiColor::End));
                        }
                        last_color = underline_color;
                    }

                    underline.push(underline_char);
                }
                if !underline.trim().is_empty() {
                    writeln!(f, "{}{}", underline, AsciiColor::End)?;
                }

                line_start = line_end + 1;
            }
        }
        Ok(())
    }
}

impl From<String> for CuteString {
    fn from(s: String) -> Self {
        CuteString {
            string: Some(s),
            colored_ranges: Vec::new(),
            underlines: Vec::new(),
            default_color: None,
        }
    }
}

impl From<&str> for CuteString {
    fn from(s: &str) -> Self {
        CuteString {
            string: Some(s.to_owned()),
            colored_ranges: Vec::new(),
            underlines: Vec::new(),
            default_color: None,
        }
    }
}
