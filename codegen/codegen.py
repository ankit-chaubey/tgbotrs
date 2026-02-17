#!/usr/bin/env python3
"""
tgbotrs — Code Generator
========================
Generates Rust source files from the Telegram Bot API spec (api.json).

Spec source: https://github.com/ankit-chaubey/api-spec
Project:     https://github.com/ankit-chaubey/tgbotrs
Author:      Ankit Chaubey <ankitchaubey.dev@gmail.com>
License:     MIT

Usage:
    python3 codegen.py <api.json> <output_directory>

Example:
    python3 codegen/codegen.py api.json tgbotrs/src/

Generates:
    gen_types.rs   — All 285 Telegram Bot API types
    gen_methods.rs — All 165 Telegram Bot API methods

No external dependencies required. Pure Python 3.6+.
"""

import json
import re
import sys
import os
from pathlib import Path

# Types that are hand-crafted in the library and must NOT be generated.
# Keep this in sync with HAND_CRAFTED_TYPES in .github/scripts/validate_generated.py
SKIP_TYPES = {
    "InputFile",   # rich enum in tgbotrs/src/input_file.rs
    "InputMedia",  # ergonomic wrapper enum in tgbotrs/src/lib.rs
}

# ─────────────────────────────────────────────────
# Load spec
# ─────────────────────────────────────────────────

def load_spec(path):
    with open(path) as f:
        return json.load(f)

# ─────────────────────────────────────────────────
# Naming helpers
# ─────────────────────────────────────────────────

def snake_case(name):
    """Convert camelCase or PascalCase to snake_case"""
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s)
    return s.lower()

def safe_field_name(name):
    """Return a Rust-safe field name"""
    keywords = {'type', 'self', 'move', 'use', 'in', 'fn', 'let', 'mut', 'ref', 'where', 'loop', 'if', 'else', 'match', 'return'}
    if name in keywords:
        return 'r#' + name
    return snake_case(name)

def method_fn_name(name):
    return snake_case(name)

def method_params_struct(name):
    return name[0].upper() + name[1:] + 'Params'

# ─────────────────────────────────────────────────
# Type mapping
# ─────────────────────────────────────────────────

BASE_TYPE_MAP = {
    'Integer': 'i64',
    'Float': 'f64',
    'Boolean': 'bool',
    'String': 'String',
    'InputFile': 'InputFile',
}

def is_array(t):
    return t.startswith('Array of ')

def strip_array(t):
    return t[len('Array of '):]

def tg_to_rust(t, optional, types_map):
    """Convert a single TG type string to Rust."""
    if is_array(t):
        inner = strip_array(t)
        inner_rust = tg_to_rust(inner, False, types_map)
        rust = f'Vec<{inner_rust}>'
        return f'Option<{rust}>' if optional else rust

    base = BASE_TYPE_MAP.get(t, t)

    if optional:
        # Check if this is a struct or enum that needs boxing to break potential cycles
        tg = types_map.get(t)
        if tg and t not in BASE_TYPE_MAP:
            return f'Option<Box<{base}>>'
        return f'Option<{base}>'
    return base

def field_rust_type(field, types_map):
    """Determine the full Rust type for a field."""
    types = field['types']
    required = field['required']
    name = field['name']

    if len(types) == 0:
        return 'serde_json::Value'

    if len(types) == 1:
        return tg_to_rust(types[0], not required, types_map)

    # Multi-type field handling
    sorted_types = sorted(types)
    if sorted_types == ['Integer', 'String']:
        return 'ChatId' if required else 'Option<ChatId>'

    if 'InputFile' in types and 'String' in types and len(types) == 2:
        return 'InputFileOrString' if required else 'Option<InputFileOrString>'

    if name == 'reply_markup' and len(types) >= 2:
        return 'ReplyMarkup' if required else 'Option<ReplyMarkup>'

    # media field (InputMedia* types)
    if name == 'media':
        if any('InputMedia' in t for t in types) or any('InputPaidMedia' in t for t in types):
            return 'InputMedia' if required else 'Option<InputMedia>'

    # Default: use first type
    return tg_to_rust(types[0], not required, types_map)

def opt_wrap(rust_type, optional):
    """Ensure a type is wrapped in Option if optional."""
    if optional and not rust_type.startswith('Option<'):
        return f'Option<{rust_type}>'
    return rust_type

def return_rust_type(returns, types_map):
    """Get Rust return type from a list of TG return types."""
    if not returns:
        return 'bool'
    if len(returns) == 1:
        t = returns[0]
        return tg_to_rust(t, False, types_map)
    # Multiple returns: serde_json::Value
    return 'serde_json::Value'

# ─────────────────────────────────────────────────
# Docs helpers
# ─────────────────────────────────────────────────

def doc_comment(lines, indent=''):
    return '\n'.join(f'{indent}/// {line}' for line in lines)

# ─────────────────────────────────────────────────
# Generate types
# ─────────────────────────────────────────────────

def generate_types(spec):
    types_map = spec['types']
    version = spec['version']
    lines = []

    lines.append(f'// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append(f'// Generated from Telegram Bot API {version}')
    lines.append(f'// Spec:    https://github.com/ankit-chaubey/api-spec')
    lines.append(f'// Project: https://github.com/ankit-chaubey/tgbotrs')
    lines.append(f'// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>')
    lines.append(f'// License: MIT')
    lines.append(f'// See:     https://core.telegram.org/bots/api')
    lines.append(f'')
    lines.append(f'#![allow(clippy::all, dead_code, unused_imports)]')
    lines.append(f'')
    lines.append(f'use serde::{{Deserialize, Serialize}};')
    lines.append(f'#[rustfmt::skip]')
    lines.append(f'use crate::{{ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia}};')
    lines.append(f'')

    for type_name in sorted(types_map.keys()):
        # Skip types that are hand-crafted in the library (not auto-generated).
        # Add new hand-crafted types to SKIP_TYPES above AND to HAND_CRAFTED_TYPES
        # in .github/scripts/validate_generated.py.
        if type_name in SKIP_TYPES:
            continue
        tg_type = types_map[type_name]
        docs = tg_type.get('description', [])
        href = tg_type.get('href', '')
        subtypes = tg_type.get('subtypes', [])
        fields = tg_type.get('fields', [])

        lines.append(doc_comment(docs))
        lines.append(f'/// {href}')

        if subtypes:
            # Union / enum type
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]')
            lines.append('#[serde(untagged)]')
            lines.append(f'pub enum {type_name} {{')
            for variant in subtypes:
                lines.append(f'    {variant}({variant}),')
            lines.append('}')
            lines.append('')
        elif not fields:
            # Empty marker struct
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]')
            lines.append(f'pub struct {type_name} {{}}')
            lines.append('')
        else:
            # Regular struct
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]')
            lines.append(f'pub struct {type_name} {{')
            for field in fields:
                fname = safe_field_name(field['name'])
                fdesc = field['description'].replace('\n', ' ')
                ftype = field_rust_type(field, types_map)
                lines.append(f'    /// {fdesc}')
                # serde rename if the field name differs or is a keyword
                if fname != field['name']:
                    rename_attr = '#[serde(rename = "' + field['name'] + '")]'
                    lines.append(f'    {rename_attr}')
                if ftype.startswith('Option<'):
                    lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
                    lines.append(f'    pub {fname}: {ftype},')
                else:
                    lines.append(f'    pub {fname}: {ftype},')
            lines.append('}')
            lines.append('')

    return '\n'.join(lines)

# ─────────────────────────────────────────────────
# Generate methods
# ─────────────────────────────────────────────────

def generate_methods(spec):
    types_map = spec['types']
    methods_map = spec['methods']
    version = spec['version']
    lines = []

    lines.append(f'// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append(f'// Generated from Telegram Bot API {version}')
    lines.append(f'// Spec:    https://github.com/ankit-chaubey/api-spec')
    lines.append(f'// Project: https://github.com/ankit-chaubey/tgbotrs')
    lines.append(f'// Author:  Ankit Chaubey <ankitchaubey.dev@gmail.com>')
    lines.append(f'// License: MIT')
    lines.append(f'// See:     https://core.telegram.org/bots/api')
    lines.append(f'')
    lines.append(f'#![allow(clippy::all, dead_code, unused_imports, unused_mut)]')
    lines.append(f'')
    lines.append(f'use serde::{{Deserialize, Serialize}};')
    lines.append(f'use crate::types::*;')
    lines.append(f'#[rustfmt::skip]')
    lines.append(f'use crate::{{Bot, BotError, ChatId, InputFile, InputFileOrString, ReplyMarkup, InputMedia}};')
    lines.append(f'')

    for method_name in sorted(methods_map.keys()):
        method = methods_map[method_name]
        fn_name = method_fn_name(method_name)
        params_name = method_params_struct(method_name)
        docs = method.get('description', [])
        href = method.get('href', '')
        all_fields = method.get('fields', [])
        returns = method.get('returns', [])

        required_fields = [f for f in all_fields if f['required']]
        optional_fields = [f for f in all_fields if not f['required']]

        # Params struct for optional fields
        if optional_fields:
            lines.append(f'/// Optional parameters for [`Bot::{fn_name}`]')
            lines.append('#[derive(Debug, Clone, Serialize, Deserialize, Default)]')
            lines.append(f'pub struct {params_name} {{')
            for field in optional_fields:
                fname = safe_field_name(field['name'])
                fdesc = field['description'].replace('\n', ' ')
                ftype = field_rust_type(field, types_map)
                # Ensure it's wrapped in Option
                ftype = opt_wrap(ftype, True)
                lines.append(f'    /// {fdesc}')
                if fname != field['name']:
                    rename_attr2 = '#[serde(rename = "' + field['name'] + '")]'
                    lines.append(f'    {rename_attr2}')
                lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
                lines.append(f'    pub {fname}: {ftype},')
            lines.append('}')
            lines.append('')

            # Builder pattern for params
            lines.append(f'impl {params_name} {{')
            lines.append(f'    pub fn new() -> Self {{ Self::default() }}')
            for field in optional_fields:
                fname = safe_field_name(field['name'])
                ftype = field_rust_type(field, types_map)
                ftype = opt_wrap(ftype, True)
                inner_type = ftype[len('Option<'):-1] if ftype.startswith('Option<') else ftype
                lines.append(f'    pub fn {fname}(mut self, v: impl Into<{inner_type}>) -> Self {{ self.{fname} = Some(v.into()); self }}')
            lines.append('}')
            lines.append('')

        # Return type
        ret = return_rust_type(returns, types_map)

        # Signature args
        sig_parts = []
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            # Flexible Into<> for common types
            if ftype == 'String':
                sig_parts.append(f'{fname}: impl Into<String>')
            elif ftype == 'ChatId':
                sig_parts.append(f'{fname}: impl Into<ChatId>')
            elif ftype == 'InputFileOrString':
                sig_parts.append(f'{fname}: impl Into<InputFileOrString>')
            elif ftype == 'InputMedia':
                sig_parts.append(f'{fname}: impl Into<InputMedia>')
            else:
                sig_parts.append(f'{fname}: {ftype}')

        has_opts = bool(optional_fields)
        if has_opts:
            sig_parts.append(f'params: Option<{params_name}>')

        sig = ', '.join(sig_parts)

        lines.append(f'impl Bot {{')
        lines.append(doc_comment(docs, '    '))
        lines.append(f'    /// See: {href}')
        args = f'&self, {sig}' if sig else '&self'
        lines.append(f'    pub async fn {fn_name}({args}) -> Result<{ret}, BotError> {{')

        # Build body
        lines.append(f'        let mut req = serde_json::Map::new();')
        for field in required_fields:
            fname = safe_field_name(field['name'])
            ftype = field_rust_type(field, types_map)
            expr = f'{fname}.into()' if ftype in ('String', 'ChatId', 'InputFileOrString', 'InputMedia') else fname
            lines.append(f'        req.insert("{field["name"]}".into(), serde_json::to_value({expr}).unwrap_or_default());')

        if has_opts:
            lines.append(f'        if let Some(p) = params {{')
            lines.append(f'            let extra = serde_json::to_value(&p).unwrap_or_default();')
            lines.append(f'            if let serde_json::Value::Object(m) = extra {{')
            lines.append(f'                for (k, v) in m {{ if !v.is_null() {{ req.insert(k, v); }} }}')
            lines.append(f'            }}')
            lines.append(f'        }}')

        lines.append(f'        self.call_api("{method_name}", &serde_json::Value::Object(req)).await')
        lines.append(f'    }}')
        lines.append(f'}}')
        lines.append(f'')

    return '\n'.join(lines)

# ─────────────────────────────────────────────────
# Generate constants (string literals from spec)
# ─────────────────────────────────────────────────

def generate_consts(spec):
    types_map = spec['types']
    lines = []
    lines.append('// THIS FILE IS AUTO-GENERATED. DO NOT EDIT.')
    lines.append('// Telegram Bot API constant field values (e.g. type discriminators)')
    lines.append('')
    lines.append('#![allow(dead_code)]')
    lines.append('')
    # Extract constant "type" field values from union type variants
    for tname in sorted(types_map.keys()):
        tg = types_map[tname]
        if not tg.get('subtype_of'):
            continue
        for field in tg.get('fields', []):
            if field['name'] == 'type' and field['types'] == ['String']:
                # The constant value is derived from the type name
                variant_name = tname
                # Guess value from description
                desc = field.get('description', '')
                # extract quoted values
                quoted = re.findall(r'"([^"]+)"', desc)
                if quoted:
                    const_name = f'{tname.upper()}_TYPE'
                    lines.append(f'/// Type discriminator for {tname}')
                    lines.append(f'pub const {const_name}: &str = "{quoted[0]}";')
    lines.append('')
    return '\n'.join(lines)

# ─────────────────────────────────────────────────
# Main
# ─────────────────────────────────────────────────

def main():
    spec_path = sys.argv[1] if len(sys.argv) > 1 else 'api.json'
    out_dir = sys.argv[2] if len(sys.argv) > 2 else '../tgbotrs/src'

    print(f'Reading spec: {spec_path}')
    spec = load_spec(spec_path)
    print(f"Telegram Bot API {spec['version']} ({spec['release_date']})")
    print(f"Types: {len(spec['types'])}, Methods: {len(spec['methods'])}")

    Path(out_dir).mkdir(parents=True, exist_ok=True)

    # gen_types.rs
    types_code = generate_types(spec)
    with open(f'{out_dir}/gen_types.rs', 'w') as f:
        f.write(types_code)
    print(f'Written: {out_dir}/gen_types.rs')

    # gen_methods.rs
    methods_code = generate_methods(spec)
    with open(f'{out_dir}/gen_methods.rs', 'w') as f:
        f.write(methods_code)
    print(f'Written: {out_dir}/gen_methods.rs')

    # Format generated files so output is always consistent with cargo fmt.
    # This ensures the validate-generated-code CI check never diffs on formatting.
    import subprocess
    for fname in ['gen_types.rs', 'gen_methods.rs']:
        subprocess.run(['rustfmt', f'{out_dir}/{fname}'], check=True)

    print('Done ✅')

if __name__ == '__main__':
    main()
