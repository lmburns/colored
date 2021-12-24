//! Standard colors for the command line and methods regarding them

use std::{borrow::Cow, io, str::FromStr};

// TODO: Add 256-ANSI support

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

/// The 8 standard colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

#[allow(missing_docs)]
impl Color {
    /// Convert a [`Color`] to a string used for foreground colors
    #[inline]
    #[must_use]
    pub fn to_fg_str(&self) -> Cow<'static, str> {
        match *self {
            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),
            Color::TrueColor { r, g, b } => format!("38;2;{};{};{}", r, g, b).into(),
        }
    }

    /// Convert a [`Color`] to a string used for background colors
    #[inline]
    #[must_use]
    pub fn to_bg_str(&self) -> Cow<'static, str> {
        match *self {
            Color::Black => "40".into(),
            Color::Red => "41".into(),
            Color::Green => "42".into(),
            Color::Yellow => "43".into(),
            Color::Blue => "44".into(),
            Color::Magenta => "45".into(),
            Color::Cyan => "46".into(),
            Color::White => "47".into(),
            Color::BrightBlack => "100".into(),
            Color::BrightRed => "101".into(),
            Color::BrightGreen => "102".into(),
            Color::BrightYellow => "103".into(),
            Color::BrightBlue => "104".into(),
            Color::BrightMagenta => "105".into(),
            Color::BrightCyan => "106".into(),
            Color::BrightWhite => "107".into(),
            Color::TrueColor { r, g, b } => format!("48;2;{};{};{}", r, g, b).into(),
        }
    }

    /// Parses a [`Color`] from an *ansi* foreground color string. _This does
    /// not parse hex notation_, instead use `Color::parse_hex`
    ///
    /// This is not to be used to parse a color word, instead use
    /// `Color::from_str`
    #[inline]
    #[must_use]
    pub fn from_fg_str(s: &str) -> Option<Self> {
        match s {
            "30" => Some(Self::Black),
            "31" => Some(Self::Red),
            "32" => Some(Self::Green),
            "33" => Some(Self::Yellow),
            "34" => Some(Self::Blue),
            "35" => Some(Self::Magenta),
            "36" => Some(Self::Cyan),
            "37" => Some(Self::White),
            "90" => Some(Self::BrightBlack),
            "91" => Some(Self::BrightRed),
            "92" => Some(Self::BrightGreen),
            "93" => Some(Self::BrightYellow),
            "94" => Some(Self::BrightBlue),
            "95" => Some(Self::BrightMagenta),
            "96" => Some(Self::BrightCyan),
            "97" => Some(Self::BrightWhite),
            color =>
                if color.starts_with("38;2;") {
                    let mut it = s.split(';');
                    it.next()?;
                    it.next()?;
                    Some(Self::TrueColor {
                        r: it.next()?.parse().ok()?,
                        g: it.next()?.parse().ok()?,
                        b: it.next()?.parse().ok()?,
                    })
                } else {
                    None
                },
        }
    }

    /// Convert a [`Color`] to a hex array
    ///
    /// Notes:
    ///   - These colors were found by searching Google for the names of the
    ///     color. They may not be exactly what you are looking for but this is
    ///     used to convert every color to a number or string which can be
    ///     further processed
    ///   - If the color did not have a bright variant, it is the same hex color
    ///     as the original
    #[inline]
    #[must_use]
    pub const fn to_hex_array(&self) -> [u8; 3] {
        match *self {
            Self::Black => [0x00, 0x00, 0x00],
            Self::Red => [0xFF, 0x00, 0x00],
            Self::Green => [0x00, 0xFF, 0x00],
            Self::Blue => [0x00, 0x00, 0xff],
            Self::Yellow | Self::BrightYellow => [0xFF, 0xFF, 0x00],
            Self::Magenta => [0xff, 0x00, 0xff],
            Self::Cyan => [0x00, 0xFF, 0xFF],
            Self::White | Self::BrightWhite => [0xFF, 0xFF, 0xFF],
            Self::BrightBlack => [0x22, 0x20, 0x24],
            Self::BrightRed => [0xFF, 0x16, 0x0C],
            Self::BrightGreen => [0xAA, 0xFF, 0x00],
            Self::BrightBlue => [0x00, 0x96, 0xFF],
            Self::BrightMagenta => [0xFF, 0x00, 0xCD],
            Self::BrightCyan => [0xE0, 0xFF, 0xFF],
            Self::TrueColor { r, g, b } => [r, g, b],
        }
    }

    /// Covnert a [`Color`] to one hex string
    #[inline]
    #[must_use]
    pub fn to_hex(&self) -> String {
        use std::fmt::Write;

        let arr = self.to_hex_array();
        let mut s = String::with_capacity(arr.len() * 2);
        for x in arr {
            write!(&mut s, "{:02x}", x).expect("failed to write character to hex");
        }
        s
    }

    /// Parses a string to a `Color::TrueColor` from *6 char notation*. If the
    /// provided string is only 6 digits (i.e., no prefix), or starts with
    /// `0x` or `#`, then the color is able to be parsed.
    ///
    /// Any colors like `0x1f1f1f` or `#ABBA12` or `121212` are valid.
    ///
    /// This is not to be used to parse a color word, instead use
    /// `Color::from_str`
    ///
    /// # Errors
    /// Will produce an error if the length of the `Color` is not 6 characters
    /// minus the hash (`#`) or hex (`0x`) prefix, or if it is not a valid hex
    /// sequence
    #[inline]
    pub fn from_hex<S: AsRef<str>>(color: S) -> Result<Self, io::Error> {
        let color = color.as_ref();
        /// Test whether the input is 6 characters long
        macro_rules! if_6 {
            ($c:ident) => {
                ($c.len() == 6).then(|| $c)
                // if $c.len() == 6 {
                //     Some($c)
                // } else {
                //     None
                // }
            };
        }

        let result = if let Some(c) = color.strip_prefix("0x") {
            if_6!(c)
        } else if let Some(c) = color.strip_prefix('#') {
            if_6!(c)
        } else {
            if_6!(color)
        };

        if let Some(color) = result {
            // hex
            if let Some((r, g, b)) = parse_hex(color) {
                return Ok(Self::TrueColor { r, g, b });
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "{} is an invalid color and is unable to be parsed",
                color.to_owned()
            ),
        ))
    }
}

impl From<&'_ str> for Color {
    #[inline]
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Self::White)
    }
}

impl From<String> for Color {
    #[inline]
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Self::White)
    }
}

impl FromStr for Color {
    type Err = ();

    #[inline]
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src.to_lowercase().trim() {
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" | "purple" => Ok(Self::Magenta),
            "cyan" => Ok(Self::Cyan),
            "white" => Ok(Self::White),
            "bright black" => Ok(Self::BrightBlack),
            "bright red" => Ok(Self::BrightRed),
            "bright green" => Ok(Self::BrightGreen),
            "bright yellow" => Ok(Self::BrightYellow),
            "bright blue" => Ok(Self::BrightBlue),
            "bright magenta" => Ok(Self::BrightMagenta),
            "bright cyan" => Ok(Self::BrightCyan),
            "bright white" => Ok(Self::BrightWhite),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "rusqlite-sql")]
pub mod sql {
    use rusqlite::types::{
        FromSql,
        FromSqlError,
        FromSqlResult,
        ToSql,
        ToSqlOutput,
        Value,
        ValueRef,
    };

    impl ToSql for Color {
        #[inline]
        fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
            Ok(ToSqlOutput::from(format!("0x{}", self.to_hex())))
        }
    }

    impl FromSql for Color {
        #[inline]
        fn column_result(val: ValueRef) -> FromSqlResult<Self> {
            match Self::from_hex(val.as_str().expect("failed to convert Color to `str`")) {
                Ok(v) => Ok(v),
                Err(err) => Err(FromSqlError::Other(Box::new(err))),
            }
        }
    }
    impl From<Color> for ToSqlOutput<'_> {
        #[inline]
        fn from(c: Color) -> Self {
            ToSqlOutput::Owned(Value::Text(c.to_hex()))
        }
    }
}


/// Convert a byte to a hex value
const fn hex_val(ch: u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - 48,
        b'A'..=b'F' => ch - 55,
        b'a'..=b'f' => ch - 87,
        _ => 0,
    }
}

/// Convert a tuple of hex chars to a `u8`
const fn hex_chars_to_u8(ch: (u8, u8)) -> u8 {
    let mut result = 0;
    result |= hex_val(ch.0);
    result <<= 4_i32;
    result |= hex_val(ch.1);
    result
}

/// Parse a 6-char-hex string into a 3 digit hex value
fn parse_hex(color: &str) -> Option<(u8, u8, u8)> {
    let mut bytes = color.as_bytes().chunks(2);

    Some((
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
        bytes.next().map(|arr| hex_chars_to_u8((arr[0], arr[1])))?,
    ))
}

#[cfg(test)]
mod tests {
    pub(crate) use super::*;

    mod from_str {
        pub(crate) use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let color : Color = $src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            purple: "purple" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod from_string {
        pub(crate) use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let src = String::from($src);
                        let color : Color = src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod fromstr {
        pub(crate) use super::*;

        #[test]
        fn parse() {
            let color: Result<Color, _> = "blue".parse();
            assert_eq!(Ok(Color::Blue), color);
        }

        #[test]
        fn error() {
            let color: Result<Color, ()> = "bloublou".parse();
            assert_eq!(Err(()), color);
        }
    }
}
