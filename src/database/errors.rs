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

impl FormatError {
    pub fn into_parse_error(self, line_no: usize, file_name: FileName) -> ParseError {
        match self {
            Self::MinusInStartOfName => ParseError::MinusInStartOfName(line_no, file_name),
            Self::NoCommand => ParseError::NoCommand(line_no, file_name),
            Self::NoName => ParseError::NoName(line_no, file_name),
            Self::SpaceInName => ParseError::SpaceInName(line_no, file_name),
            Self::UsedName => ParseError::UsedName(line_no, file_name),
        }
    }
}

/// The type representing a line number.
pub type LineNumber = usize;

/// The type representing a line number.
pub type FileName = &'static str;

/// The error type for parsing operations.
pub enum ParseError {
    MinusInStartOfName(LineNumber, FileName),
    NoName(LineNumber, FileName),
    NoCommand(LineNumber, FileName),
    SpaceInName(LineNumber, FileName),
    NoColon(LineNumber),
    UnexpectedComment(LineNumber),
    UsedName(LineNumber, FileName),
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            Self::MinusInStartOfName(line_no, file_name) => {
                format!(
                    "LINE {}: a name cannot start with a minus symbol in {}",
                    line_no, file_name
                )
            }
            Self::NoColon(line_no) => {
                format!(
                    "LINE {}: seperate name and script with a colon in {}",
                    line_no, "run.yaml"
                )
            }
            Self::NoCommand(line_no, file_name) => {
                format!("LINE {}: script is empty in {}", line_no, file_name)
            }
            Self::NoName(line_no, file_name) => {
                format!("LINE {}: name is empty in {}", line_no, file_name)
            }
            Self::SpaceInName(line_no, file_name) => {
                format!(
                    "LINE {}: a name cannot contain a space symbol in {}",
                    line_no, file_name
                )
            }
            Self::UnexpectedComment(line_no) => {
                format!("LINE {}: unexpected comment in {}", line_no, "run.yaml")
            }
            Self::UsedName(line_no, file_name) => format!(
                "LINE {}: same name is used before in {}",
                line_no, file_name
            ),
        }
    }
}
