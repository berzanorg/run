use std::fmt::Display;

/// All the availible colors.
pub mod colors {
    pub const GREEN: u8 = 32;
    pub const RED: u8 = 31;
    pub const YELLOW: u8 = 33;
}

/// The struct that enables having beautiful text.
pub struct Beautiful<'a, T>
where
    T: 'a + Display,
{
    content: &'a T,
    color: Option<u8>,
}

// Implement `Display` trait for `Beautiful`.
impl<'a, T> Display for Beautiful<'a, T>
where
    T: 'a + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Some(color) => write!(f, "\x1B[1;{}m{}\x1B[0m", color, self.content),

            None => self.content.fmt(f),
        }
    }
}

/// The trait that makes converting displayable things to `Beautiful` easy.
pub trait Beautify<'a>
where
    Self: 'a + Display + Sized,
{
    fn green(&'a self) -> Beautiful<'a, Self>;
    fn red(&'a self) -> Beautiful<'a, Self>;
    fn yellow(&'a self) -> Beautiful<'a, Self>;
}

// Implement `Beautify` trait for all the displayable things.
impl<'a, T> Beautify<'a> for T
where
    T: 'a + Display,
{
    /// Makes the text green.
    fn green(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            color: Some(colors::GREEN),
        }
    }

    /// Makes the text red.
    fn red(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            color: Some(colors::RED),
        }
    }

    /// Makes the text yellow.
    fn yellow(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            color: Some(colors::YELLOW),
        }
    }
}
