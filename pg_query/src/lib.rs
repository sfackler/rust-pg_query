#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate pg_query_sys;

use std::ffi::{CString, CStr};
use std::fmt;
use std::str;
use std::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct Error {
    pub message: String,
    pub file: String,
    pub line: u32,
    pub index: usize,
    _p: (),
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Error")
            .field("message", &self.message)
            .field("file", &self.file)
            .field("line", &self.line)
            .field("index", &self.index)
            .finish()
    }
}

impl Error {
    unsafe fn from_raw(raw: *mut pg_query_sys::PgQueryError) -> Error {
        Error {
            message: str::from_utf8(CStr::from_ptr((*raw).message).to_bytes()).unwrap().to_owned(),
            file: str::from_utf8(CStr::from_ptr((*raw).filename).to_bytes()).unwrap().to_owned(),
            line: (*raw).lineno as u32,
            index: (*raw).cursorpos as usize,
            _p: (),
        }
    }
}

fn init() -> MutexGuard<'static, ()> {
    lazy_static! {
        static ref LOCK: Mutex<()> = {
            unsafe {
                pg_query_sys::pg_query_init();
            }

            Mutex::new(())
        };
    }

    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

pub fn parse_to_json(query: &str) -> Result<String, Error> {
    let _lock = init();

    let query = CString::new(query).expect("interior null");
    unsafe {
        let raw_result = pg_query_sys::pg_query_parse(query.as_ptr() as *mut _);
        let result = if raw_result.error.is_null() {
            Ok(str::from_utf8(CStr::from_ptr(raw_result.parse_tree).to_bytes()).unwrap().to_owned())
        } else {
            Err(Error::from_raw(raw_result.error))
        };
        pg_query_sys::pg_query_free_parse_result(raw_result);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        for _ in 0..1000 {
            parse_to_json("SELECT foo FROM bar WHERE baz = $1").unwrap();
        }
    }

    #[test]
    fn err() {
        for _ in 0..1000 {
            let err = parse_to_json("SELECT foo FORM bar WEHRE baz = $1").unwrap_err();
            assert_eq!(err.message, "syntax error at or near \"bar\"");
        }
    }
}
