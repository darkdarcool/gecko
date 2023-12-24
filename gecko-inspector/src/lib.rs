use std::collections::HashMap;

use gecko_parser::nodes::{stmt::Stmt, expr::Type};
use gecko_parser::nodes::expr::Expr;
use tinyjson::{JsonGenerator, JsonValue};

fn add_name_to_json(name: String, json: JsonValue) -> JsonValue {
    let mut map = HashMap::new();
    map.insert(name, json);
    JsonValue::Object(map)
}

fn type_to_string(t: Type) -> String {
    match t {
        Type::Int(i) => format!("Int({})", i),
        Type::Float(f) => format!("Float({})", f),
        Type::String(s) => format!("String({})", s),
        Type::Bool(b) => format!("Bool({})", b),
        Type::Void => String::from("Void"),
        Type::Unknown => String::from("Unknown"),
    }
}

fn expr_to_json(expr: Expr) -> JsonValue {
    match expr {
        Expr::Literal(lit) => {
            let mut literal: HashMap<String, JsonValue> = HashMap::new();
            literal.insert(String::from("value"), JsonValue::String(type_to_string(lit)));
            let json = JsonValue::Object(literal);
            add_name_to_json(String::from("Literal"), json)
        },
        Expr::Binary(left, op, right) => {
            let mut binary: HashMap<String, JsonValue> = HashMap::new();
            binary.insert(String::from("left"), expr_to_json(left.as_ref().clone()));
            binary.insert(String::from("operator"), JsonValue::String(op.lexeme));
            binary.insert(String::from("right"), expr_to_json(right.as_ref().clone()));
            let json = JsonValue::Object(binary);
            add_name_to_json(String::from("Binary"), json)
        },
        _ => JsonValue::Null,
    }
}

fn stmt_to_json(stmt: Stmt) -> JsonValue {
    match stmt {
        Stmt::VarDecl(var) => {
            let mut var_decl: HashMap<String, JsonValue> = HashMap::new();
            var_decl.insert(String::from("name"), JsonValue::String(var.name));
            var_decl.insert(String::from("initializer"), expr_to_json(var.initializer.unwrap()));
            let json = JsonValue::Object(var_decl);
            add_name_to_json(String::from("VarDecl"), json)
        }
        _ => JsonValue::Null,
    }
}

pub fn inspect(tree: Vec<Stmt>) -> String {
    let mut buf = vec![];
    let mut gen = JsonGenerator::new(&mut buf).indent(" ");

    let mut root: Vec<JsonValue> = vec![];

    for stmt in tree {
        root.push(stmt_to_json(stmt));
    }

    let json_root = JsonValue::Array(root);
    gen.generate(&json_root).unwrap();

    String::from_utf8(buf).unwrap()
}
