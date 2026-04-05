/// Rust is most of the time pain in the ass to code with.
/// So this lib as some very convenient typing to ease the code.

/// For function returns:
pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
pub type Void = Res<()>;
