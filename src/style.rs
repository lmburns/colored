const CLEARV: u8 = 0b0000_0000;
const BOLD: u8 = 0b0000_0001;
const UNDERLINE: u8 = 0b0000_0010;
const REVERSED: u8 = 0b0000_0100;
const ITALIC: u8 = 0b0000_1000;
const BLINK: u8 = 0b0001_0000;
const HIDDEN: u8 = 0b0010_0000;
const DIMMED: u8 = 0b0100_0000;
const STRIKETHROUGH: u8 = 0b1000_0000;

static STYLES: [(u8, Styles); 8] = [
    (BOLD, Styles::Bold),
    (DIMMED, Styles::Dimmed),
    (UNDERLINE, Styles::Underline),
    (REVERSED, Styles::Reversed),
    (ITALIC, Styles::Italic),
    (BLINK, Styles::Blink),
    (HIDDEN, Styles::Hidden),
    (STRIKETHROUGH, Styles::Strikethrough),
];

pub(crate) static CLEAR: Style = Style(CLEARV);

/// A combinatorial style such as bold, italics, dimmed, etc.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Style(u8);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum Styles {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}

impl Styles {
    const fn to_str<'a>(self) -> &'a str {
        match self {
            Self::Clear => "", // unreachable, but we don't want to panic
            Self::Bold => "1",
            Self::Dimmed => "2",
            Self::Italic => "3",
            Self::Underline => "4",
            Self::Blink => "5",
            Self::Reversed => "7",
            Self::Hidden => "8",
            Self::Strikethrough => "9",
        }
    }

    const fn to_u8(self) -> u8 {
        match self {
            Self::Clear => CLEARV,
            Self::Bold => BOLD,
            Self::Dimmed => DIMMED,
            Self::Italic => ITALIC,
            Self::Underline => UNDERLINE,
            Self::Blink => BLINK,
            Self::Reversed => REVERSED,
            Self::Hidden => HIDDEN,
            Self::Strikethrough => STRIKETHROUGH,
        }
    }

    fn from_u8(u: u8) -> Option<Vec<Self>> {
        if u == CLEARV {
            return None;
        }

        let res: Vec<Self> = STYLES
            .iter()
            .filter(|(mask, _)| (0 != (u & mask)))
            .map(|&(_, value)| value)
            .collect();
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

impl Style {
    /// Check if the current style has one of [`Styles`](Styles) switched on.
    ///
    /// ```rust
    /// # use colored::*;
    /// let colored = "".bold().italic();
    /// assert_eq!(colored.style().contains(Styles::Bold), true);
    /// assert_eq!(colored.style().contains(Styles::Italic), true);
    /// assert_eq!(colored.style().contains(Styles::Dimmed), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn contains(self, style: Styles) -> bool {
        let s = style.to_u8();
        self.0 & s == s
    }

    /// Convert style to string
    pub(crate) fn to_str(self) -> String {
        let styles = Styles::from_u8(self.0).unwrap_or_default();
        styles
            .iter()
            .map(|s| s.to_str())
            .collect::<Vec<&str>>()
            .join(";")
    }

    /// Combine styles
    pub(crate) fn add(&mut self, two: Styles) {
        self.0 |= two.to_u8();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod u8_to_styles_invalid_is_none {
        use super::super::{Styles, CLEARV};

        #[test]
        fn empty_is_none() {
            assert_eq!(None, Styles::from_u8(CLEARV));
        }
    }

    mod u8_to_styles_isomorphism {
        use super::super::{
            Styles,
            BLINK,
            BOLD,
            DIMMED,
            HIDDEN,
            ITALIC,
            REVERSED,
            STRIKETHROUGH,
            UNDERLINE,
        };

        macro_rules! value_isomorph {
            ($name:ident, $value:expr) => {
                #[test]
                fn $name() {
                    let u = Styles::from_u8($value);
                    assert!(
                        u.is_some(),
                        "{}: Styles::from_u8 -> None",
                        stringify!($value)
                    );
                    let u = u.unwrap();
                    assert!(
                        u.len() == 1,
                        "{}: Styles::from_u8 found {} styles (expected 1)",
                        stringify!($value),
                        u.len()
                    );
                    assert!(
                        u[0].to_u8() == $value,
                        "{}: to_u8() doesn't match its const value",
                        stringify!($value)
                    );
                }
            };
        }

        value_isomorph!(bold, BOLD);
        value_isomorph!(underline, UNDERLINE);
        value_isomorph!(reversed, REVERSED);
        value_isomorph!(italic, ITALIC);
        value_isomorph!(blink, BLINK);
        value_isomorph!(hidden, HIDDEN);
        value_isomorph!(dimmed, DIMMED);
        value_isomorph!(strikethrough, STRIKETHROUGH);
    }

    mod styles_combine_complex {
        use super::super::{Style, Styles, Styles::*};

        fn style_from_multiples(styles: &[Styles]) -> Style {
            let mut res = Style(styles[0].to_u8());
            for s in &styles[1..] {
                res = Style(res.0 | s.to_u8());
            }
            res
        }

        macro_rules! test_aggreg {
            ($styles:expr, $expect:expr) => {{
                let v = style_from_multiples($styles);
                let r = Styles::from_u8(v.0).expect("should find styles");
                let e: &[Styles] = &$expect;
                assert_eq!(e, &r[..])
            }};
        }

        #[test]
        fn aggreg1() {
            let styles: &[Styles] = &[Bold, Bold, Bold];
            test_aggreg!(styles, [Bold])
        }

        #[test]
        fn aggreg2() {
            let styles: &[Styles] = &[Italic, Italic, Bold, Bold];
            test_aggreg!(styles, [Bold, Italic])
        }

        #[test]
        fn aggreg3() {
            let styles: &[Styles; 3] = &[Bold, Italic, Bold];
            test_aggreg!(styles, [Bold, Italic])
        }

        macro_rules! test_combine {
            ($styles:expr) => {{
                let v = style_from_multiples($styles);
                let r = Styles::from_u8(v.0).expect("should find styles");
                assert_eq!($styles, &r[..])
            }};
        }

        #[test]
        fn two1() {
            let s: &[Styles] = &[Bold, Underline];
            test_combine!(s)
        }

        #[test]
        fn two2() {
            let s: &[Styles] = &[Underline, Italic];
            test_combine!(s)
        }

        #[test]
        fn two3() {
            let s: &[Styles] = &[Bold, Italic];
            test_combine!(s)
        }

        #[test]
        fn three1() {
            let s: &[Styles] = &[Bold, Underline, Italic];
            test_combine!(s)
        }

        #[test]
        fn three2() {
            let s: &[Styles] = &[Dimmed, Underline, Italic];
            test_combine!(s)
        }

        #[test]
        fn four() {
            let s: &[Styles] = &[Dimmed, Underline, Italic, Hidden];
            test_combine!(s)
        }

        #[test]
        fn five() {
            let s: &[Styles] = &[Dimmed, Underline, Italic, Blink, Hidden];
            test_combine!(s)
        }

        #[test]
        fn six() {
            let s: &[Styles] = &[Bold, Dimmed, Underline, Italic, Blink, Hidden];
            test_combine!(s)
        }

        #[test]
        fn all() {
            let s: &[Styles] = &[
                Bold,
                Dimmed,
                Underline,
                Reversed,
                Italic,
                Blink,
                Hidden,
                Strikethrough,
            ];
            test_combine!(s)
        }
    }

    #[test]
    fn test_style_contains() {
        let mut style = Style(Styles::Bold.to_u8());
        style.add(Styles::Italic);

        assert_eq!(style.contains(Styles::Bold), true);
        assert_eq!(style.contains(Styles::Italic), true);
        assert_eq!(style.contains(Styles::Dimmed), false);
    }
}
