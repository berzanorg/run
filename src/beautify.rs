use std::fmt::Display;

/// All the availible fonts.
pub mod fonts {
    pub const BOLD: u8 = 1;
    pub const TRANSPARENT: u8 = 2;
    pub const ITALIC: u8 = 3;
}

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
    font: Option<u8>,
    color: Option<u8>,
}

// Implement functions to modify `Beautiful` values.
impl<'a, T> Beautiful<'a, T>
where
    T: 'a + Display,
{
    /// Makes the text bold.
    pub fn bold(self) -> Self {
        Beautiful {
            font: Some(fonts::BOLD),
            ..self
        }
    }

    /// Makes the text italic.
    pub fn italic(self) -> Self {
        Beautiful {
            font: Some(fonts::ITALIC),
            ..self
        }
    }

    /// Makes the text transparent.
    pub fn transparent(self) -> Self {
        Beautiful {
            font: Some(fonts::TRANSPARENT),
            ..self
        }
    }

    /// Makes the text green.
    pub fn green(self) -> Self {
        Beautiful {
            color: Some(colors::GREEN),
            ..self
        }
    }

    /// Makes the text red.
    pub fn red(self) -> Self {
        Beautiful {
            color: Some(colors::RED),
            ..self
        }
    }

    /// Makes the text yellow.
    pub fn yellow(self) -> Self {
        Beautiful {
            color: Some(colors::YELLOW),
            ..self
        }
    }
}

/// The trait that makes converting displayable things to `Beautiful` easy.
pub trait Beautify<'a>
where
    Self: 'a + Display + Sized,
{
    fn bold(&'a self) -> Beautiful<'a, Self>;
    fn italic(&'a self) -> Beautiful<'a, Self>;
    fn transparent(&'a self) -> Beautiful<'a, Self>;
    fn green(&'a self) -> Beautiful<'a, Self>;
    fn red(&'a self) -> Beautiful<'a, Self>;
    fn yellow(&'a self) -> Beautiful<'a, Self>;
}

// Implement `Beautify` trait for all the displayable things.
impl<'a, T> Beautify<'a> for T
where
    T: 'a + Display,
{
    /// Makes the text bold.
    fn bold(&'a self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: Some(fonts::BOLD),
            color: None,
        }
    }

    /// Makes the text italic.
    fn italic(&'a self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: Some(fonts::ITALIC),
            color: None,
        }
    }

    /// Makes the text transparent.
    fn transparent(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: Some(fonts::TRANSPARENT),
            color: None,
        }
    }

    /// Makes the text green.
    fn green(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: None,
            color: Some(colors::GREEN),
        }
    }

    /// Makes the text red.
    fn red(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: None,
            color: Some(colors::RED),
        }
    }

    /// Makes the text yellow.
    fn yellow(&self) -> Beautiful<T> {
        Beautiful {
            content: &self,
            font: None,
            color: Some(colors::YELLOW),
        }
    }
}

// Implement `Display` trait for `Beautiful`.
impl<'a, T> Display for Beautiful<'a, T>
where
    T: 'a + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Some(color) => match self.font {
                Some(font) => write!(f, "\x1B[{};{}m{}\x1B[0m", color, font, self.content),
                None => write!(f, "\x1B[{}m{}\x1B[0m", color, self.content),
            },
            None => match self.font {
                Some(font) => write!(f, "\x1B[{}m{}\x1B[0m", font, self.content),
                None => self.content.fmt(f),
            },
        }
    }
}
