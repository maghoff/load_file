// TODO Documentation

use std::{path::Path, fs::File, io::Read, str};

#[doc(hidden)]
pub fn load_file_bytes(base: &str, rel: &str) -> Result<&'static [u8], &'static str> {
    let path = Path::new(base)
        .parent().ok_or("invalid source file path")?
        .join(rel);

    let mut f = File::open(path).map_err(|_| "file not found")?;

    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .map_err(|_| "unable to read the file")?;

    let contents = contents.into_boxed_slice();
    Ok(Box::leak(contents))
}

#[doc(hidden)]
pub fn load_file_str(base: &str, rel: &str) -> Result<&'static str, &'static str> {
    let bytes = load_file_bytes(base, rel)?;
    let s = str::from_utf8(bytes).map_err(|_| "invalid utf8")?;
    Ok(s)
}

#[macro_export]
macro_rules! load_bytes {
    ($name:expr) => {
        match $crate::load_file_bytes(file!(), $name) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_bytes!({:?})", msg, $name));
            }
        }
    };
}

#[macro_export]
macro_rules! load_str {
    ($name:expr) => {
        match $crate::load_file_str(file!(), $name) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{} in load_str!({:?})", msg, $name));
            }
        }
    };
}
