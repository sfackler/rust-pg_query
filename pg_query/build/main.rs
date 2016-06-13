extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
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

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out = File::create(out_dir.join("types.rs")).unwrap();
    let mut out = BufWriter::new(out);

    make_node(&struct_defs, &mut out);
}

fn make_node(struct_defs: &HashMap<String, HashMap<String, Struct>>, out: &mut BufWriter<File>) {
    write!(out,
"pub enum Node {{
").unwrap();
    for (name, def) in &struct_defs["nodes/parsenodes"] {
        write!(out,
"    {} {{
", name).unwrap();

        for field in &def.fields {
            let (name, c_type) = match (&field.name, &field.c_type) {
                (&Some(ref name), &Some(ref c_type)) => (name, c_type),
                _ => continue,
            };

            if name == "type" {
                continue;
            }

            let rust_type = c_to_rust_type(c_type);

            write!(out,
"        {}: {},
", name, rust_type).unwrap();
        }

        write!(out,
"    }},
").unwrap();
    }

    write!(out,
"}}
").unwrap();
}

fn c_to_rust_type(c_type: &str) -> &'static str {
    match c_type {
        "CmdType" => "CmdType",
        "QuerySource" => "QuerySource",
        "uint32" => "u32",
        "bool" => "bool",
        "Node*" => "Box<Node>",
        "int" => "i32",
        "List*" => "Vec<Node>",
        "FromExpr*" => "FromExpr",
        "OnConflictExpr*" => "OnConflictExpr",
        "CreateStmt" => "Box<Node>",
        "Oid" => "Oid",
        "int32" => "i32",
        "RangeVar*" => "RangeVar",
        "char*" => "String",
        "TypeName*" => "TypeName",
        "GrantTargetType" => "GrantTargetType",
        "GrantObjectType" => "GrantObjectType",
        "WindowDef*" => "WindowDef",
        "ObjectType" => "ObjectType",
        "WithClause*" => "WithClause",
        "RoleSpecType" => "RoleSpecType",
        "XmlOptionType" => "XmlOptionType",
        "DropBehavior" => "DropBehavior",
        "OnCommitAction" => "OnCommitAction",
        "Expr*" => "Expr",
        "Alias*" => "Alias",
        "DefElemAction" => "DefElemAction",
        "int16" => "i16",
        "char" => "u8",
        "RoleStmtType" => "RoleStmtType",
        "FunctionParameterMode" => "FunctionParameterMode",
        "Index" => "Index",
        "A_Expr_Kind" => "A_Expr_Kind",
        "SortByDir" => "SortByDir",
        "VariableSetStmt*" => "VariableSetStmt",
        "IntoClause*" => "IntoClause",
        "AlterTSConfigType" => "AlterTSConfigType",
        "SortByNulls" => "SortByNulls",
        "DiscardMode" => "DiscardMode",
        "GrantStmt*" => "GrantStmt",
        "LockClauseStrength" => "LockClauseStrength",
        "RTEKind" => "RTEKind",
        "SetOperation" => "SetOperation",
        "LockWaitPolicy" => "LockWaitPolicy",
        "VariableSetKind" => "VariableSetKind",
        "Value" => "Value",
        "OnConflictClause*" => "OnConflictClause",
        "CollateClause*" => "CollateClause",
        "SelectStmt*" => "Box<Node>",
        t => panic!("unknown C type {}", t),
    }
}
