use std::collections::HashMap;

use huff_utils::prelude::{Argument, Contract, MacroDefinition, OPCODES};

pub enum HuffCompletionItem {
    // Opcode: name
    Opcode(String),
    // Macro: name, args
    Macro(String, Vec<Argument>),
    // Builtin Function Invocation
    BuiltinFunction(String),
}

const BUILTIN_FUNCTIONS: [&str; 8] = [
    "__tablesize",
    "__codesize",
    "__tablestart",
    "__FUNC_SIG",
    "__EVENT_HASH",
    "__ERROR",
    "__RIGHTPAD",
    "__CODECOPY_DYN_ARG",
];

// TODO: include completions from related files
pub fn completion(ast: &Contract, ident_offset: usize) -> HashMap<String, HuffCompletionItem> {
    let mut map = HashMap::new();

    // Just check against opcodes for now
    // Check if the current pointer sits in a macro def span
    if inside_macro_context(&ast.macros, ident_offset) {
        // Cache this response
        for opcode in OPCODES {
            map.insert(
                opcode.to_string(),
                HuffCompletionItem::Opcode(opcode.to_string()),
            );
        }

        for builtin in BUILTIN_FUNCTIONS {
            map.insert(
                builtin.to_string(),
                HuffCompletionItem::BuiltinFunction(builtin.to_string()),
            );
        }

        // Other macro names - TODO: this is already iterated over - do this in one operation
        for mac in &ast.macros {
            map.insert(
                mac.name.clone(),
                HuffCompletionItem::Macro(mac.name.clone(), mac.parameters.clone()),
            );
        }
    }

    map
}

fn inside_macro_context(macro_defs: &Vec<MacroDefinition>, ident_offset: usize) -> bool {
    for macro_def in macro_defs {
        let first_macro_statement = macro_def.statements.first();
        let last_macro_statement = macro_def.statements.last();

        if first_macro_statement.is_some() {
            let first_span = first_macro_statement.unwrap().span.0.first();
            let last_span = last_macro_statement.unwrap().span.0.last();
            if first_span.is_some() {
                if first_span.unwrap().start < ident_offset && last_span.unwrap().end > ident_offset
                {
                    return true;
                }
            }
        }
    }
    return false;
}
