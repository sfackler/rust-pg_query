extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::collections::HashMap;

use types::{Struct, Enum};

mod types;

fn main() {
    let dir = PathBuf::from(env::var_os("DEP_PG_QUERY_SRCDATA").unwrap());

    let struct_defs = File::open(dir.join("struct_defs.json")).unwrap();
    let struct_defs = BufReader::new(struct_defs);

    let struct_defs: HashMap<String, HashMap<String, Struct>> =
        serde_json::from_reader(struct_defs).unwrap();

    let enum_defs = File::open(dir.join("enum_defs.json")).unwrap();
    let enum_defs = BufReader::new(enum_defs);

    let enum_defs: HashMap<String, HashMap<String, Enum>> =
        serde_json::from_reader(enum_defs).unwrap();
}
