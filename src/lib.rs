//! This crate provides macros to help conveniently load the contents of
//! files during development.
//!
//! `load_str!` and `load_bytes!` are modeled after `include_str!` and
//! `include_bytes!` from the standard library. The standard library macros
//! are useful in many situations, one of which is quick-and-dirty loading of
//! assets during a prototyping phase. (Examples of such assets are static web
//! assets such as CSS or GLSL shaders for a game.) The `load_*` macros aim to
//! offer a convenient way of loading the assets dynamically at run-time
//! instead. This gets rid of the need to compile or even restart for every
//! change while iterating on the assets.
//!
//! # Example
//! Before:
//!
//! ```
//! fn main() {
//!     println!("{}", include_str!("greeting.txt"));
//! }
//! ```
//!
//! After:
//!
//! ```
//! #[macro_use]
//! extern crate load_file;
//!
//! fn main() {
//!     println!("{}", load_str!("greeting.txt"));
//! }
//! ```

use std::{path::{Path, PathBuf}, fs::File, io::Read, str};

#[doc(hidden)]
pub fn resolve_path(base: &str, rel: &str) -> Result<PathBuf, &'static str> {
    Ok(Path::new(base)
        .parent().ok_or("invalid source file path")?
        .join(rel))
}

#[doc(hidden)]
pub fn load_file_bytes(path: &Path) -> Result<&'static [u8], &'static str> {
    let mut f = File::open(path).map_err(|_| "file not found")?;

    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .map_err(|_| "unable to read the file")?;

    let contents = contents.into_boxed_slice();
    Ok(Box::leak(contents))
}

#[doc(hidden)]
pub fn load_file_str(path: &Path) -> Result<&'static str, &'static str> {
    let bytes = load_file_bytes(path)?;
    let s = str::from_utf8(bytes).map_err(|_| "invalid utf8")?;
    Ok(s)
}

/// Load a file as a reference to a byte array at run-time.
///
/// The file is located relative to the current source file, and the binary
/// must be run with the crate root as the working directory.
///
/// The resulting value is a `&'static [u8]` with the contents of the file.
///
/// This macro can often be a drop-in replacement for `include_bytes!`,
/// switching it to be a run-time rather than compile-time operation.
///
/// Each time the macro is reached, the file is read into memory in its
/// entirety and the memory is leaked, keeping the memory valid for the
/// remainder of the program execution.
///
/// # Compatibility with `include_bytes!`
/// Apart from the semantic differences between `include_bytes!` and
/// `load_bytes!` there are also some technical differences:
///
///  * With `include_bytes!`, the length of the array is statically known, and
///    is included in the type: `&'static [u8; N]`, vs `&'static [u8]` for
///    `load_bytes!`
///  * `include_bytes!` can appear in static contexts in the source code,
///    while `load_bytes!` can not. It is possible to use the `lazy_static`
///    crate to work around this.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate load_file;
///
/// fn main() {
///     let greeting: &[u8] = load_bytes!("greeting.txt");
///     println!("{:?}", greeting);
/// }
/// ```
///
/// # Panics
/// To facilitate using `load_bytes!` as a drop-in replacement for
/// `include_bytes!`, all error situations cause panics:
///
///  * File not found
///  * Read errors
#[macro_export]
macro_rules! load_bytes {
    ($name:expr) => {{
        let path = match $crate::resolve_path(file!(), $name) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_bytes!({:?})", msg, $name));
            }
        };
        match $crate::load_file_bytes(&path) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_bytes!({:?}) (resolved to: {:?})",
                    msg, $name, path));
            }
        }
    }};
}

/// Load a utf8-encoded file as a string at run-time.
///
/// The file is located relative to the current source file, and the binary
/// must be run with the crate root as the working directory.
///
/// The resulting value is a `&'static str` with the contents of the file.
///
/// This macro can often be a drop-in replacement for `include_str!`,
/// switching it to be a run-time rather than compile-time operation.
///
/// Each time the macro is reached, the file is read into memory in its
/// entirety and the memory is leaked, keeping the memory valid for the
/// remainder of the program execution.
///
/// # Compatibility with `include_str!`
/// Apart from the semantic differences between `include_str!` and `load_str!`
/// there are also a technical difference:
///
/// `include_str!` can appear in static contexts in the source code, while
/// `load_str!` can not. It is possible to use the `lazy_static` crate to work
/// around this.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate load_file;
///
/// fn main() {
///     let greeting: &str = load_str!("greeting.txt");
///     println!("{}", greeting);
/// }
/// ```
///
/// # Panics
/// To facilitate using `load_str!` as a drop-in replacement for
/// `include_str!`, all error situations cause panics:
///
///  * File not found
///  * Read errors
///  * UTF-8 validation errors
#[macro_export]
macro_rules! load_str {
    ($name:expr) => {{
        let path = match $crate::resolve_path(file!(), $name) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_bytes!({:?})", msg, $name));
            }
        };
        match $crate::load_file_str(&path) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_str!({:?}) (resolved to: {:?})",
                    msg, $name, path));
            }
        }
    }};
}
