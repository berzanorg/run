/// The error type for database operations.
pub enum DatabaseError<'a> {
    NoAlias(char),
    MultiAlias(char),
    NoName(&'a str),
}

impl<'a> ToString for DatabaseError<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::MultiAlias(alias) => {
                format!("there are multiple names starting with '{}'", alias)
            }
            Self::NoAlias(alias) => format!("there is no name starting with '{}'", alias),
            Self::NoName(name) => format!("there isn't a script called '{}'", name),
        }
    }
}

/// The error type for script format.
pub enum FormatError {
    MinusInStartOfName,
    NoName,
    NoCommand,
    SpaceInName,
    UsedName,
}

/// The type representing a line number.
pub type LineNumber = usize;

/// The error type for parsing operations.
pub enum ParseError {
    NoName(LineNumber),
    NoCommand(LineNumber),
    SpaceInName(LineNumber),
    MinusInStartOfName(LineNumber),
    NoColon(LineNumber),
    UnexpectedComment(LineNumber),
    UsedName(LineNumber),
}

impl ParseError {
    pub fn from_parse_error(err: FormatError, line_no: usize) -> ParseError {
        match err {
            FormatError::MinusInStartOfName => ParseError::MinusInStartOfName(line_no),
            FormatError::NoCommand => ParseError::NoCommand(line_no),
            FormatError::NoName => ParseError::NoName(line_no),
            FormatError::SpaceInName => ParseError::SpaceInName(line_no),
            FormatError::UsedName => ParseError::UsedName(line_no),
        }
    }
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            Self::MinusInStartOfName(line_no) => {
                format!("LINE {}: a name cannot start with a minus symbol", line_no)
            }
            Self::NoColon(line_no) => {
                format!("LINE {}: seperate name and script with a colon", line_no)
            }
            Self::NoCommand(line_no) => format!("LINE {}: script is empty", line_no),
            Self::NoName(line_no) => format!("LINE {}: name is empty", line_no),
            Self::SpaceInName(line_no) => {
                format!("LINE {}: a name cannot contain a space symbol", line_no)
            }
            Self::UnexpectedComment(line_no) => format!("LINE {}: unexpected comment", line_no),
            Self::UsedName(line_no) => format!("LINE {}: same name is used before", line_no),
        }
    }
}
