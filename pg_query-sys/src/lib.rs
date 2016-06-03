extern crate libc;

use libc::{c_char, c_int};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgQueryError {
    pub message: *mut c_char,
    pub funcname: *mut c_char,
    pub filename: *mut c_char,
    pub lineno: c_int,
    pub cursorpos: c_int,
    pub context: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgQueryParseResult {
    pub parse_tree: *mut c_char,
    pub stderr_buffer: *mut c_char,
    pub error: *mut PgQueryError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgQueryPlpgsqlParseResult {
    pub plpgsql_func: *mut c_char,
    pub stderr_buffer: *mut c_char,
    pub error: *mut PgQueryError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgQueryFingerprintResult {
    pub hexdigest: *mut c_char,
    pub stderr_buffer: *mut c_char,
    pub error: *mut PgQueryError,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgQueryNormalizeResult {
    pub normalized_query: *mut c_char,
    pub error: *mut PgQueryError,
}

extern "C" {
    pub fn pg_query_init();
    pub fn pg_query_normalize(input: *const c_char) -> PgQueryNormalizeResult;
    pub fn pg_query_parse(input: *const c_char) -> PgQueryParseResult;
    pub fn pg_query_parse_plpgsql(input: *const c_char) -> PgQueryPlpgsqlParseResult;

    pub fn pg_query_fingerprint(input: *const c_char) -> PgQueryFingerprintResult;

    pub fn pg_query_free_normalize(result: PgQueryNormalizeResult);
    pub fn pg_query_free_parse_result(result: PgQueryParseResult);
    pub fn pg_query_free_plpgsql_parse_result(result: PgQueryPlpgsqlParseResult);
    pub fn pg_query_free_fingerprint_result(result: PgQueryFingerprintResult);
}
