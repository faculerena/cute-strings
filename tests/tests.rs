#[cfg(test)]
mod tests {

    use cute_strings::{AsciiColor, CuteString};

    const SINGLE_LINE_MSG: &str =
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam nec purus.";

    const MULTI_LINE_MSG: &str =
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nNullam nec purus.\n123 456 789.";

    const ESCAPE_SEQUENCE_RESET: &str = "\u{1b}[0m\n";
    #[test]
    fn initialize_works() {
        let printer = CuteString::new();
        assert_eq!(printer.to_string(), "");
    }

    #[test]
    fn set_string_without_color() {
        let mut printer = CuteString::new();
        let printer = printer.set_string(SINGLE_LINE_MSG);
        assert_eq!(
            printer.to_string(),
            SINGLE_LINE_MSG.to_owned() + ESCAPE_SEQUENCE_RESET
        );
    }

    #[test]
    fn set_string_with_one_color() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(SINGLE_LINE_MSG)
            .set_default_color(AsciiColor::Red);
        assert_eq!(
            printer.to_string(),
            AsciiColor::Red.to_string() + SINGLE_LINE_MSG + ESCAPE_SEQUENCE_RESET
        );
    }

    #[test]
    fn set_string_with_overlapping_colors() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(SINGLE_LINE_MSG)
            .set_default_color(AsciiColor::Red)
            .color_range(0, 20, AsciiColor::Blue)
            .color_range(10, 30, AsciiColor::Green);

        assert_eq!(
            printer.to_string(),
            "\u{1b}[31m\u{1b}[34mLorem ipsum dolor si\u{1b}[32mt amet, co\u{1b}[31mnsectetur adipiscing elit. Nullam nec purus.\u{1b}[0m\n"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn set_regex_coloring() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(SINGLE_LINE_MSG)
            .color_regex(r"\w{5}", AsciiColor::Red)
            .unwrap();

        assert_eq!(
            printer.to_string(),
            "\u{1b}[31mLorem\u{1b}[0m \u{1b}[31mipsum\u{1b}[0m \u{1b}[31mdolor\u{1b}[0m sit amet, \u{1b}[31mconsectetu\u{1b}[0mr \u{1b}[31madipiscing\u{1b}[0m elit. \u{1b}[31mNulla\u{1b}[0mm nec \u{1b}[31mpurus\u{1b}[0m.\u{1b}[0m\n"
        );
    }
    #[cfg(feature = "regex")]
    #[test]
    fn set_regex_underline() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(SINGLE_LINE_MSG)
            .underline_regex(r"[a-zA-Z]{2,}", AsciiColor::Purple, '^')
            .unwrap();

        assert_eq!(
            printer.to_string(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam nec purus.\u{1b}[0m\n\u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^\u{1b}[0m \u{1b}[35m^^^^\u{1b}[0m  \u{1b}[35m^^^^^^^^^^^\u{1b}[0m \u{1b}[35m^^^^^^^^^^\u{1b}[0m \u{1b}[35m^^^^\u{1b}[0m  \u{1b}[35m^^^^^^\u{1b}[0m \u{1b}[35m^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[0m\n"
        );
    }

    #[cfg(feature = "regex")]
    #[test]
    fn set_regex_underline_multiline() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(MULTI_LINE_MSG)
            .underline_regex(r"[a-zA-Z]{2,}", AsciiColor::Purple, '^')
            .unwrap();

        assert_eq!(
            printer.to_string(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\u{1b}[0m\n\u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[35m^^^\u{1b}[0m \u{1b}[35m^^^^\u{1b}[0m  \u{1b}[35m^^^^^^^^^^^\u{1b}[0m \u{1b}[35m^^^^^^^^^^\u{1b}[0m \u{1b}[35m^^^^\u{1b}[0m \u{1b}[0m\nNullam nec purus.\u{1b}[0m\n\u{1b}[35m^^^^^^\u{1b}[0m \u{1b}[35m^^^\u{1b}[0m \u{1b}[35m^^^^^\u{1b}[0m \u{1b}[0m\n123 456 789.\u{1b}[0m\n"
        );
    }

    #[test]
    fn color_char() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(SINGLE_LINE_MSG)
            .color_char('a', AsciiColor::Red);
        assert_eq!(printer.to_string(), "Lorem ipsum dolor sit \u{1b}[31ma\u{1b}[0mmet, consectetur \u{1b}[31ma\u{1b}[0mdipiscing elit. Null\u{1b}[31ma\u{1b}[0mm nec purus.\u{1b}[0m\n");
    }

    #[test]
    fn color_digits() {
        let mut printer = CuteString::new();
        let printer = printer.set_string(MULTI_LINE_MSG).color_digits();
        assert_eq!(printer.to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\u{1b}[0m\nNullam nec purus.\u{1b}[0m\n\u{1b}[31m1\u{1b}[33m2\u{1b}[32m3\u{1b}[0m \u{1b}[34m4\u{1b}[35m5\u{1b}[36m6\u{1b}[0m \u{1b}[38;5;208m7\u{1b}[38;5;205m8\u{1b}[38;5;51m9\u{1b}[0m.\u{1b}[0m\n");
    }

    #[test]
    fn color_line() {
        let mut printer = CuteString::new();
        let printer = printer
            .set_string(MULTI_LINE_MSG)
            .color_line(1, AsciiColor::Red);
        assert_eq!(printer.to_string(), "\u{1b}[31mLorem ipsum dolor sit amet, consectetur adipiscing elit.\u{1b}[0m\nNullam nec purus.\u{1b}[0m\n123 456 789.\u{1b}[0m\n");
    }
}
