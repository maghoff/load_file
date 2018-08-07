// TODO Documentation

#[doc(hidden)]
pub fn load_file_bytes(base: &str, rel: &str) -> Result<&'static [u8], &'static str> {
    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;

    let path = Path::new(base).parent().ok_or("invalid source file path")?.join(rel);

    let mut f = File::open(path).map_err(|_| "file not found")?;

    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .map_err(|_| "unable to read the file")?;

    let contents = contents.into_boxed_slice();
    Ok(Box::leak(contents))
}

#[doc(hidden)]
pub fn load_file_str(base: &str, rel: &str) -> Result<&'static str, &'static str> {
    Ok(std::str::from_utf8(load_file_bytes(base, rel)?).map_err(|_| "invalid utf8")?)
}

#[macro_export]
macro_rules! load_bytes {
    ($name:expr) => {
        match $crate::load_file_bytes(file!(), $name) {
            Ok(x) => x,
            Err(msg) => {
                panic!(format!("{msg} in load_bytes!({name:?})",
                    msg=msg, name=$name));
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
                panic!(format!("{msg} in load_str!({name:?})",
                    msg=msg, name=$name));
            }
        }
    };
}
