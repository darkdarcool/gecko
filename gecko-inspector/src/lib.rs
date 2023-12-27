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
        Type::Iden(i) => format!("Iden({})", i),
        Type::Void => String::from("Void"),
        Type::Unknown => String::from("Unknown"),
    }
}

fn expr_to_json(expr: Expr) -> JsonValue {
    match expr {
        Expr::Literal(lit) => {
            let mut literal: HashMap<String, JsonValue> = HashMap::new();
            literal.insert(String::from("value"), JsonValue::String(type_to_string(lit.value)));
            let json = JsonValue::Object(literal);
            add_name_to_json(String::from("Literal"), json)
        },
        Expr::Binary(expr) => {
            let mut binary: HashMap<String, JsonValue> = HashMap::new();
            binary.insert(String::from("left"), expr_to_json(expr.left.as_ref().clone()));
            binary.insert(String::from("operator"), JsonValue::String(expr.operator.lexeme));
            binary.insert(String::from("right"), expr_to_json(expr.right.as_ref().clone()));
            let json = JsonValue::Object(binary);
            add_name_to_json(String::from("Binary"), json)
        },
        // Expr::Variable(var) => {
        //     let mut variable: HashMap<String, JsonValue> = HashMap::new();
        //     variable.insert(String::from("name"), JsonValue::String(var.lexeme));
        //     let json = JsonValue::Object(variable);
        //     add_name_to_json(String::from("Variable"), json)
        // },
        _ => JsonValue::Null,
    }
}

#[allow(unreachable_patterns)]
fn stmt_to_json(stmt: Stmt) -> JsonValue {
    match stmt {
        Stmt::VarDecl(var) => {
            let mut var_decl: HashMap<String, JsonValue> = HashMap::new();
            var_decl.insert(String::from("name"), JsonValue::String(var.name));
            var_decl.insert(String::from("initializer"), expr_to_json(var.initializer.unwrap()));
            let json = JsonValue::Object(var_decl);
            add_name_to_json(String::from("VarDeclStmt"), json)
        },
        Stmt::ExprStmt(expr) => {
            let mut expr_stmt: HashMap<String, JsonValue> = HashMap::new();
            expr_stmt.insert(String::from("expression"), expr_to_json(expr));
            let json = JsonValue::Object(expr_stmt);
            add_name_to_json(String::from("ExprStmt"), json)
        },
        Stmt::FnDecl(func) => {
            let mut fn_decl: HashMap<String, JsonValue> = HashMap::new();
            fn_decl.insert(String::from("name"), JsonValue::String(func.name));
            fn_decl.insert(String::from("params"), JsonValue::Array(func.params.into_iter().map(|p| JsonValue::String(p.name)).collect()));
            fn_decl.insert(String::from("body"), JsonValue::Array(func.body.into_iter().map(|s| stmt_to_json(s)).collect()));
            if let Some(t) = func.return_type {
                fn_decl.insert(String::from("rtype"), JsonValue::String(t.lexeme));
            } else {
                fn_decl.insert(String::from("rtype"), JsonValue::Null);
            }
            let json = JsonValue::Object(fn_decl);
            add_name_to_json(String::from("FnDecl"), json)
        },
        Stmt::Return(expr) => {
            let mut return_stmt: HashMap<String, JsonValue> = HashMap::new();
            return_stmt.insert(String::from("expression"), expr_to_json(expr.unwrap()));
            let json = JsonValue::Object(return_stmt);
            add_name_to_json(String::from("ReturnStmt"), json)
        },
        Stmt::FileImport(path) => {
            let mut file_import: HashMap<String, JsonValue> = HashMap::new();
            file_import.insert(String::from("path"), JsonValue::String(path));
            let json = JsonValue::Object(file_import);
            add_name_to_json(String::from("FileImportStmt"), json)
        },
        Stmt::LangImport(langs) => {
            let mut lang_import: HashMap<String, JsonValue> = HashMap::new();
            lang_import.insert(String::from("modules"), JsonValue::Array(langs.into_iter().map(|l| JsonValue::String(l)).collect()));
            let json = JsonValue::Object(lang_import);
            add_name_to_json(String::from("LangImportStmt"), json)
        },
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
