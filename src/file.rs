use crate::beautify::Beautify;

/// The type that represents file names.
type FileName = &'static str;

#[derive(Debug)]
/// The error type file system operations.
pub enum FileError {
    /// The file is not able to be read.
    CannotBeRead(FileName),
    /// The file is not able to be written.
    CannotBeWritten(FileName),
    /// The file is not found.
    NotFound(FileName),
    /// The file already exists.
    Exists(FileName),
}

// Implement `ToString` trait for `FileError`.
impl ToString for FileError {
    fn to_string(&self) -> String {
        match self {
            Self::CannotBeRead(file_name) => {
                format!("{} {}", "can't read".red(), file_name.green())
            }

            Self::CannotBeWritten(file_name) => {
                format!("{} {}", "can't write to".red(), file_name.green())
            }

            Self::Exists(file_name) => format!("{} {}", file_name.green(), "already exists".red(),),

            Self::NotFound(file_name) => format!(
                "{} {}\n\n{}",
                file_name.green(),
                "isn't found".red(),
                "type `run -i` to generate one".yellow()
            ),
        }
    }
}

/// The type based on `Result<T, FileError>`.
type Result<T> = core::result::Result<T, FileError>;

/// Reads the given file in the current directory.
///
pub fn read(file_name: FileName) -> Result<String> {
    match std::fs::read_to_string(file_name) {
        Ok(content) => Ok(content),
        Err(err) => Err(match err.kind() {
            std::io::ErrorKind::NotFound => FileError::NotFound(file_name),
            _ => FileError::CannotBeRead(file_name),
        }),
    }
}

/// Writes the given file in the current directory.
///
/// If the file doesn't exists, creates a new one.
///
pub fn write(file_name: FileName, content: &str) -> Result<()> {
    match std::fs::write(file_name, content) {
        Ok(()) => Ok(()),
        Err(_) => Err(FileError::CannotBeWritten(file_name)),
    }
}

/// Writes the given file in the current directory.
///
/// Only writes, if the file doesn't exists.
///
pub fn create(file_name: FileName, content: &str) -> Result<()> {
    if exists(file_name) {
        return Err(FileError::Exists(file_name));
    } else {
        write(file_name, content)
    }
}

/// Returns `true` if the given file exists in the current directory.
/// Otherwise returns `false`.
pub fn exists(file_name: FileName) -> bool {
    std::path::Path::new(file_name).exists()
}
