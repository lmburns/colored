// #![cfg(not(feature = "no-color"))]
#![allow(unused_imports)]

use ansi_term::Colour;
use colored::Colorize;

macro_rules! test_simple_color {
    ($string:expr, $colored_name:ident, $ansi_term_name:ident) => {
        #[test]
        fn $colored_name() {
            let s = format!("{} {}", $string, stringify!($colored_name));
            assert_eq!(
                s.$colored_name().to_string(),
                Colour::$ansi_term_name.paint(s).to_string()
            )
        }
    };
}

mod compat_colors {
    use super::{Colour, Colorize};

    test_simple_color!("test string", black, Black);
    test_simple_color!("test string", red, Red);
    test_simple_color!("test string", green, Green);
    test_simple_color!("test string", yellow, Yellow);
    test_simple_color!("test string", blue, Blue);
    test_simple_color!("test string", magenta, Purple);
    test_simple_color!("test string", cyan, Cyan);
    test_simple_color!("test string", white, White);
}

macro_rules! test_simple_style {
    ($string:expr, $colored_style:ident, $ansi_style:ident) => {
        #[test]
        fn $colored_style() {
            let s = format!("{} {}", $string, stringify!($style));
            assert_eq!(
                s.$colored_style().to_string(),
                ansi_term::Style::new().$ansi_style().paint(s).to_string()
            )
        }
    };
}

mod compat_styles {
    use super::{Colour, Colorize};

    test_simple_style!("test string", bold, bold);
    test_simple_style!("test string", dimmed, dimmed);
    test_simple_style!("test string", italic, italic);
    test_simple_style!("test string", underline, underline);
    test_simple_style!("test string", blink, blink);
    test_simple_style!("test string", reversed, reverse);
    test_simple_style!("test string", hidden, hidden);
}

macro_rules! test_simple_bgcolor {
    ($string:expr, $colored_name:ident, $ansi_term_name:ident) => {
        #[test]
        fn $colored_name() {
            let s = format!("{} {}", $string, stringify!($colored_name));
            assert_eq!(
                s.$colored_name().to_string(),
                ansi_term::Style::default()
                    .on(ansi_term::Colour::$ansi_term_name)
                    .paint(s)
                    .to_string()
            )
        }
    };
}

mod compat_bgcolors {
    use super::{Colour, Colorize};

    test_simple_bgcolor!("test string", on_black, Black);
    test_simple_bgcolor!("test string", on_red, Red);
    test_simple_bgcolor!("test string", on_green, Green);
    test_simple_bgcolor!("test string", on_yellow, Yellow);
    test_simple_bgcolor!("test string", on_blue, Blue);
    test_simple_bgcolor!("test string", on_magenta, Purple);
    test_simple_bgcolor!("test string", on_cyan, Cyan);
    test_simple_bgcolor!("test string", on_white, White);
}

mod compat_complex {
    use super::{Colour, Colorize};

    #[test]
    fn complex1() {
        let s = "test string";
        let ansi = Colour::Red.on(Colour::Black).bold().italic().paint(s);
        assert_eq!(
            ansi.to_string(),
            s.red().bold().italic().on_black().to_string()
        );
    }

    #[test]
    fn complex2() {
        let s = "test string";
        let ansi = Colour::Green.on(Colour::Yellow).underline().paint(s);
        assert_eq!(
            ansi.to_string(),
            s.green().on_yellow().underline().to_string()
        );
    }
}

mod compat_overrides {
    use super::{Colour, Colorize};

    #[test]
    fn overrides1() {
        let s = "test string";
        let ansi = Colour::Red.on(Colour::Black).on(Colour::Blue).paint(s);
        assert_eq!(ansi.to_string(), s.red().on_blue().to_string());
    }

    #[test]
    fn overrides2() {
        let s = "test string";
        let ansi = Colour::Green.on(Colour::Yellow).paint(s);
        assert_eq!(
            ansi.to_string(),
            s.green().on_yellow().green().on_yellow().to_string()
        );
    }
}
