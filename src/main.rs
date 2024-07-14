use cute_strings::{AsciiColor, CuteString};

fn main() {
    let mut printer = CuteString::new();

    printer
        .set_string("Th1s 1s an 3x4mple of 4 str1ng that w1ll b3 COLORED w1th 2 d1ff3rent colors.");

    #[cfg(feature = "regex")]
    do_that_with_regex(&mut printer);

    #[cfg(not(feature = "regex"))]
    do_that_without_regex(&mut printer);

    println!("{}", printer);
}

#[cfg(feature = "regex")]
fn do_that_with_regex(printer: &mut CuteString) {
    printer
        .color_regex(r"\d+", AsciiColor::Red)
        .unwrap()
        .color_range(0, 5, AsciiColor::Blue)
        .underline_regex(r"[A-Z]{2,}", AsciiColor::Purple, '^')
        .unwrap();
}

#[cfg(not(feature = "regex"))]
fn do_that_without_regex(printer: &mut CuteString) {
    printer.set_default_color(AsciiColor::Green).color_digits();
}
