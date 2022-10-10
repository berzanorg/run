/// The trait that enables getting the value inside a `Result`.
/// 
pub trait Exit<T> {
    fn exit(self) -> T;
}


// Implement `Exit<T>` for `Result<T, E>`.
impl<T, E: ToString> Exit<T> for Result<T, E> {
    /// If it's `Ok(T)`, returns `T` inside `Result<T, E>`.
    /// 
    /// If it's `Err(E)`, prints `E.to_string()` to standard error,
    /// and terminates the process.
    /// 
    fn exit(self) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{}", err.to_string());
                std::process::exit(1);
            }
        }
    }
}

