use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SizeUnit {
    Zero,
    Px(i32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Auto,
}

impl Display for SizeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SizeUnit::Zero => write!(f, "0px"),
            SizeUnit::Px(pixels) => write!(f, "{}px", pixels),
            SizeUnit::Em(em) => write!(f, "{}em", em),
            SizeUnit::Rem(rem) => write!(f, "{}rem", rem),
            SizeUnit::Percent(percent) => write!(f, "{}%", percent),
            SizeUnit::Auto => write!(f, "auto"),
        }
    }
}
