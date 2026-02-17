use heck::{ToPascalCase, ToSnakeCase};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;

// ─────────────────────────────────────────────────
// API Spec structures
// ─────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
struct ApiSpec {
    version: String,
    release_date: String,
    methods: HashMap<String, Method>,
    types: HashMap<String, TgType>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct TgType {
    name: String,
    description: Vec<String>,
    href: String,
    #[serde(default)]
    fields: Vec<Field>,
    /// If non-empty, this is a union/interface type; the list is its variants.
    #[serde(default)]
    subtypes: Vec<String>,
    /// Which union types this type belongs to
    #[serde(default)]
    subtype_of: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Method {
    name: String,
    description: Vec<String>,
    href: String,
    #[serde(default)]
    fields: Vec<Field>,
    returns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Field {
    name: String,
    /// Multiple means the field can be one of several types (union field).
    types: Vec<String>,
    required: bool,
    description: String,
}

// ─────────────────────────────────────────────────
// Type mapping helpers
// ─────────────────────────────────────────────────

fn is_tg_array(s: &str) -> bool {
    s.starts_with("Array of ")
}

fn strip_array(s: &str) -> &str {
    s.trim_start_matches("Array of ")
}

/// Convert a Telegram API type name to a Rust type string.
/// `optional` wraps struct types in `Option<Box<...>>`.
fn tg_to_rust(s: &str, optional: bool, types_map: &HashMap<String, TgType>) -> String {
    if is_tg_array(s) {
        let inner = strip_array(s);
        let inner_rust = tg_to_rust(inner, false, types_map);
        if optional {
            return format!("Option<Vec<{}>>", inner_rust);
        }
        return format!("Vec<{}>", inner_rust);
    }
    let base = match s {
        "Integer" => "i64".to_string(),
        "Float" => "f64".to_string(),
        "Boolean" => "bool".to_string(),
        "String" => "String".to_string(),
        "InputFile" => "InputFile".to_string(),
        _ => s.to_string(),
    };
    // If it's a complex TG type (struct) and optional, box it
    if optional {
        let is_tg_struct = types_map
            .get(s)
            .map(|t| t.subtypes.is_empty() && !t.fields.is_empty())
            .unwrap_or(false);
        let is_tg_union = types_map
            .get(s)
            .map(|t| !t.subtypes.is_empty())
            .unwrap_or(false);
        if is_tg_struct || is_tg_union {
            return format!("Option<Box<{}>>", base);
        }
        if matches!(s, "Integer" | "Float" | "Boolean" | "String") {
            return format!("Option<{}>", base);
        }
        format!("Option<{}>", base)
    } else {
        base
    }
}

/// Determine the Rust type for a field (handling union fields, optional, etc.)
fn field_rust_type(field: &Field, types_map: &HashMap<String, TgType>) -> String {
    match field.types.len() {
        0 => "serde_json::Value".to_string(),
        1 => tg_to_rust(&field.types[0], !field.required, types_map),
        _ => {
            // Multi-type field
            let types = &field.types;
            // chat_id: Integer | String => ChatId enum
            if types == &["Integer", "String"] || types == &["String", "Integer"] {
                if field.required {
                    return "ChatId".to_string();
                } else {
                    return "Option<ChatId>".to_string();
                }
            }
            // InputFile | String => InputFileOrString
            if (types.contains(&"InputFile".to_string()) && types.contains(&"String".to_string()))
                && types.len() == 2
            {
                if field.required {
                    return "InputFileOrString".to_string();
                } else {
                    return "Option<InputFileOrString>".to_string();
                }
            }
            // reply_markup: InlineKeyboardMarkup | ReplyKeyboardMarkup | ReplyKeyboardRemove | ForceReply
            if field.name == "reply_markup" {
                if field.required {
                    return "ReplyMarkup".to_string();
                } else {
                    return "Option<ReplyMarkup>".to_string();
                }
            }
            // Default: use first type
            tg_to_rust(&types[0], !field.required, types_map)
        }
    }
}

fn rust_field_name(name: &str) -> String {
    // Some names conflict with Rust keywords
    match name {
        "type" => "r#type".to_string(),
        "self" => "r#self".to_string(),
        "move" => "r#move".to_string(),
        _ => name.to_snake_case(),
    }
}

fn method_fn_name(name: &str) -> String {
    name.to_snake_case()
}

fn method_params_name(name: &str) -> String {
    format!("{}Params", name.to_pascal_case())
}

// Types implemented manually in the library — skip generating them to avoid duplicates.
// Keep in sync with SKIP_TYPES in codegen/codegen.py and HAND_CRAFTED_TYPES in
// .github/scripts/validate_generated.py
const SKIP_TYPES: &[&str] = &["InputFile", "InputMedia"];

fn sorted_keys<V>(map: &HashMap<String, V>) -> Vec<String> {
    let mut keys: Vec<_> = map.keys().cloned().collect();
    keys.sort();
    keys
}

// ─────────────────────────────────────────────────
// Code generators
// ─────────────────────────────────────────────────

fn generate_types(spec: &ApiSpec) -> String {
    let mut out = String::new();
    writeln!(out, "// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.").unwrap();
    writeln!(out, "// Generated from Telegram Bot API {}", spec.version).unwrap();
    writeln!(out, "// https://core.telegram.org/bots/api").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(
        out,
        "use crate::{{ChatId, InputFile, InputFileOrString, ReplyMarkup}};"
    )
    .unwrap();
    writeln!(out).unwrap();

    let types_map = &spec.types;

    for type_name_str in sorted_keys(types_map) {
        if SKIP_TYPES.contains(&type_name_str.as_str()) {
            continue;
        }
        let tg_type = &types_map[&type_name_str];
        let docs = tg_type.description.join("\n/// ");

        if !tg_type.subtypes.is_empty() {
            // Union type → Rust enum
            writeln!(out, "/// {}", docs).unwrap();
            writeln!(out, "/// See: {}", tg_type.href).unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
            )
            .unwrap();
            writeln!(out, "#[serde(untagged)]").unwrap();
            writeln!(out, "pub enum {} {{", type_name_str).unwrap();
            for variant in &tg_type.subtypes {
                writeln!(out, "    {}({}),", variant, variant).unwrap();
            }
            writeln!(out, "}}").unwrap();
            writeln!(out).unwrap();
        } else if tg_type.fields.is_empty() {
            // Empty struct (marker type)
            writeln!(out, "/// {}", docs).unwrap();
            writeln!(out, "/// See: {}", tg_type.href).unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]"
            )
            .unwrap();
            writeln!(out, "pub struct {} {{}}", type_name_str).unwrap();
            writeln!(out).unwrap();
        } else {
            // Regular struct
            writeln!(out, "/// {}", docs).unwrap();
            writeln!(out, "/// See: {}", tg_type.href).unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]"
            )
            .unwrap();
            writeln!(out, "pub struct {} {{", type_name_str).unwrap();

            for field in &tg_type.fields {
                let rust_type = field_rust_type(field, types_map);
                let fname = rust_field_name(&field.name);
                let desc = field.description.replace('\n', " ");
                writeln!(out, "    /// {}", desc).unwrap();
                // serde rename if needed (snake_case is already the default for serde)
                if fname.starts_with("r#") || fname != field.name {
                    writeln!(out, "    #[serde(rename = \"{}\")]", field.name).unwrap();
                }
                // skip_serializing_if for optional fields
                if rust_type.starts_with("Option<") {
                    writeln!(
                        out,
                        "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                    )
                    .unwrap();
                }
                writeln!(out, "    pub {}: {},", fname, rust_type).unwrap();
            }

            writeln!(out, "}}").unwrap();
            writeln!(out).unwrap();
        }
    }

    out
}

fn generate_methods(spec: &ApiSpec) -> String {
    let mut out = String::new();
    writeln!(out, "// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.").unwrap();
    writeln!(out, "// Generated from Telegram Bot API {}", spec.version).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "use serde::{{Deserialize, Serialize}};").unwrap();
    writeln!(
        out,
        "use crate::{{Bot, BotError, ChatId, InputFile, InputFileOrString, ReplyMarkup}};"
    )
    .unwrap();
    writeln!(out, "use crate::types::*;").unwrap();
    writeln!(out).unwrap();

    let types_map = &spec.types;

    for method_name_str in sorted_keys(&spec.methods) {
        let method = &spec.methods[&method_name_str];
        let fn_name = method_fn_name(&method.name);
        let params_name = method_params_name(&method.name);
        let docs = method.description.join("\n    /// ");

        let required_fields: Vec<&Field> = method.fields.iter().filter(|f| f.required).collect();
        let optional_fields: Vec<&Field> = method.fields.iter().filter(|f| !f.required).collect();

        // Generate the params struct (optional fields)
        if !optional_fields.is_empty() {
            writeln!(out, "/// Optional parameters for [`Bot::{}`]", fn_name).unwrap();
            writeln!(
                out,
                "#[derive(Debug, Clone, Serialize, Deserialize, Default)]"
            )
            .unwrap();
            writeln!(out, "pub struct {} {{", params_name).unwrap();
            for field in &optional_fields {
                let rust_type = field_rust_type(field, types_map);
                let fname = rust_field_name(&field.name);
                let desc = field.description.replace('\n', " ");
                writeln!(out, "    /// {}", desc).unwrap();
                if fname.starts_with("r#") || fname != field.name {
                    writeln!(out, "    #[serde(rename = \"{}\")]", field.name).unwrap();
                }
                writeln!(
                    out,
                    "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                )
                .unwrap();
                // Optional fields in params struct are always Option
                let opt_type = if rust_type.starts_with("Option<") {
                    rust_type.clone()
                } else {
                    format!("Option<{}>", rust_type)
                };
                writeln!(out, "    pub {}: {},", fname, opt_type).unwrap();
            }
            writeln!(out, "}}").unwrap();
            writeln!(out).unwrap();
        }

        // Return type
        let ret_type = build_return_type(&method.returns, types_map);

        // Function signature
        let req_args: Vec<String> = required_fields
            .iter()
            .map(|f| {
                let rust_type = field_rust_type(f, types_map);
                let fname = rust_field_name(&f.name);
                // Use into() for flexible types
                let param_type = if rust_type == "String" {
                    "impl Into<String>".to_string()
                } else if rust_type == "ChatId" {
                    "impl Into<ChatId>".to_string()
                } else if rust_type == "InputFileOrString" {
                    "impl Into<InputFileOrString>".to_string()
                } else {
                    rust_type
                };
                format!("{}: {}", fname, param_type)
            })
            .collect();

        let has_opts = !optional_fields.is_empty();

        writeln!(out, "impl Bot {{").unwrap();
        writeln!(out, "    /// {}", docs).unwrap();
        writeln!(out, "    /// See: {}", method.href).unwrap();

        let mut sig_args = req_args.join(", ");
        if has_opts {
            if !sig_args.is_empty() {
                sig_args.push_str(", ");
            }
            sig_args.push_str(&format!("params: Option<&{}>", params_name));
        }

        writeln!(
            out,
            "    pub async fn {}(&self, {}) -> Result<{}, BotError> {{",
            fn_name, sig_args, ret_type
        )
        .unwrap();

        // Build request body
        writeln!(out, "        let mut form = serde_json::json!({{}});").unwrap();

        // Add required fields
        for field in &required_fields {
            let fname = rust_field_name(&field.name);
            let rust_type = field_rust_type(field, types_map);
            // .into() for flexible types
            let expr = if rust_type == "String"
                || rust_type == "ChatId"
                || rust_type == "InputFileOrString"
            {
                format!("{}.into()", fname)
            } else {
                fname.to_string()
            };
            writeln!(
                out,
                "        form[\"{}\"] = serde_json::to_value({}).unwrap_or_default();",
                field.name, expr
            )
            .unwrap();
        }

        // Add optional fields from params
        if has_opts {
            writeln!(out, "        if let Some(p) = params {{").unwrap();
            for field in &optional_fields {
                let fname = rust_field_name(&field.name);
                writeln!(
                    out,
                    "            if let Some(ref v) = p.{} {{ form[\"{}\"] = serde_json::to_value(v).unwrap_or_default(); }}",
                    fname, field.name
                )
                .unwrap();
            }
            writeln!(out, "        }}").unwrap();
        }

        writeln!(
            out,
            "        self.call_api(\"{}\", &form).await",
            method.name
        )
        .unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
    }

    out
}

fn build_return_type(returns: &[String], types_map: &HashMap<String, TgType>) -> String {
    if returns.is_empty() {
        return "serde_json::Value".to_string();
    }
    if returns.len() == 1 {
        return tg_to_rust(&returns[0], false, types_map);
    }
    // Multiple: use serde_json::Value as fallback
    "serde_json::Value".to_string()
}

// ─────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let spec_path = args.get(1).map(|s| s.as_str()).unwrap_or("api.json");
    let out_dir = args.get(2).map(|s| s.as_str()).unwrap_or("../tgbotrs/src");

    println!("Reading spec from: {}", spec_path);
    let json = std::fs::read_to_string(spec_path).expect("Could not read api.json");
    let spec: ApiSpec = serde_json::from_str(&json).expect("Could not parse api.json");

    println!("Telegram Bot API {} ({})", spec.version, spec.release_date);
    println!(
        "Found {} types and {} methods",
        spec.types.len(),
        spec.methods.len()
    );

    let types_code = generate_types(&spec);
    let methods_code = generate_methods(&spec);

    let types_out = format!("{}/gen_types.rs", out_dir);
    let methods_out = format!("{}/gen_methods.rs", out_dir);

    std::fs::write(&types_out, &types_code).expect("Failed to write gen_types.rs");
    std::fs::write(&methods_out, &methods_code).expect("Failed to write gen_methods.rs");

    println!("Generated: {}", types_out);
    println!("Generated: {}", methods_out);
    println!("Done! ✅");
}
