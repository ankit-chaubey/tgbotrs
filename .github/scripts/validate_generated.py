#!/usr/bin/env python3
"""
validate_generated.py — Validates that all types and methods from api.json
are present across the generated AND hand-crafted Rust source files.

Usage:
    python3 validate_generated.py api.json gen_types.rs gen_methods.rs

How it works:
    - Generated types    → must appear in gen_types.rs
    - Hand-crafted types → defined in HAND_CRAFTED_TYPES below; searched in
                           tgbotrs/src/ instead of gen_types.rs
    - All methods        → must appear in gen_methods.rs

Adding a new hand-crafted type:
    1. Implement it in the appropriate tgbotrs/src/*.rs file
    2. Add its name to HAND_CRAFTED_TYPES below (relative path from repo root)
"""

import glob
import json
import os
import re
import sys

# ─────────────────────────────────────────────────────────────────────────────
# Single source of truth for types implemented manually outside gen_types.rs.
# Key   = exact Telegram API type name
# Value = list of source file globs (relative to repo root) where it lives
# ─────────────────────────────────────────────────────────────────────────────
HAND_CRAFTED_TYPES = {
    # Rich enum with FileId / Url / Memory variants — in input_file.rs
    "InputFile":  ["tgbotrs/src/input_file.rs"],
    # Ergonomic wrapper enum with From<> impls — in lib.rs
    "InputMedia": ["tgbotrs/src/lib.rs"],
}
# ─────────────────────────────────────────────────────────────────────────────


def snake_case(name):
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s)
    return s.lower()


def read(path):
    """Read a file relative to cwd (repo root) or as absolute path."""
    for p in [path, os.path.join(os.getcwd(), path)]:
        if os.path.exists(p):
            with open(p) as f:
                return f.read()
    return ""


def find_type_in_sources(type_name, src_globs):
    """Return True if `pub struct/enum TypeName` appears in any of the given files."""
    patterns = [f"pub struct {type_name}", f"pub enum {type_name}"]
    for pattern in src_globs:
        for filepath in glob.glob(pattern) or [pattern]:
            src = read(filepath)
            if any(p in src for p in patterns):
                return True
    return False


def main():
    if len(sys.argv) < 4:
        print("Usage: validate_generated.py <api.json> <gen_types.rs> <gen_methods.rs>")
        sys.exit(1)

    spec         = json.load(open(sys.argv[1]))
    types_src    = open(sys.argv[2]).read()
    methods_src  = open(sys.argv[3]).read()
    all_types    = spec["types"]
    all_methods  = spec["methods"]

    errors   = []
    warnings = []

    # ── 1. Types ─────────────────────────────────────────────────────────────
    print(f"\n=== Validating {len(all_types)} types ===")
    print(f"    ({len(HAND_CRAFTED_TYPES)} hand-crafted, exempt from gen_types.rs check: "
          f"{list(HAND_CRAFTED_TYPES.keys())})")

    missing_types = []
    for type_name, type_info in all_types.items():
        is_union = bool(type_info.get("subtypes"))
        gen_pattern = f"pub enum {type_name}" if is_union else f"pub struct {type_name}"

        if type_name in HAND_CRAFTED_TYPES:
            # Verify it actually exists in its declared hand-crafted source
            if not find_type_in_sources(type_name, HAND_CRAFTED_TYPES[type_name]):
                errors.append(
                    f"❌ Hand-crafted type '{type_name}' not found in "
                    f"{HAND_CRAFTED_TYPES[type_name]} — did you forget to implement it?"
                )
        else:
            if gen_pattern not in types_src:
                missing_types.append(type_name)
                errors.append(f"❌ Missing generated type: {type_name} (expected '{gen_pattern}')")
            elif not is_union and type_info.get("fields"):
                for field in type_info["fields"]:
                    if field["name"] not in types_src:
                        warnings.append(f"⚠️  Field '{field['name']}' may be missing from {type_name}")

    if missing_types:
        print(f"❌ Missing types ({len(missing_types)}): {missing_types}")
    else:
        print(f"✅ All {len(all_types)} types are present "
              f"({len(all_types) - len(HAND_CRAFTED_TYPES)} generated + "
              f"{len(HAND_CRAFTED_TYPES)} hand-crafted)")

    # ── 2. Methods ───────────────────────────────────────────────────────────
    print(f"\n=== Validating {len(all_methods)} methods ===")

    gen_fns = set(re.findall(r'pub async fn (\w+)', methods_src))
    missing_methods = []
    for method_name in all_methods:
        fn_name = snake_case(method_name)
        if fn_name not in gen_fns:
            missing_methods.append(method_name)
            errors.append(f"❌ Missing method: {method_name} (expected fn '{fn_name}')")

    if missing_methods:
        print(f"❌ Missing methods ({len(missing_methods)}): {missing_methods}")
    else:
        print(f"✅ All {len(all_methods)} methods are present")

    # ── 3. Union variants ────────────────────────────────────────────────────
    print(f"\n=== Validating union type variants ===")

    union_errors = 0
    for type_name, type_info in all_types.items():
        subtypes = type_info.get("subtypes", [])
        if not subtypes:
            continue

        if type_name in HAND_CRAFTED_TYPES:
            # For hand-crafted union types, just warn if a variant type is
            # not referenced anywhere in the declared source file(s)
            combined = "".join(read(p) for srcs in [HAND_CRAFTED_TYPES[type_name]] for p in srcs)
            for variant in subtypes:
                if variant not in combined:
                    warnings.append(
                        f"⚠️  Hand-crafted {type_name}: variant '{variant}' not referenced "
                        f"in {HAND_CRAFTED_TYPES[type_name]}"
                    )
        else:
            for variant in subtypes:
                if f"{variant}({variant})" not in types_src:
                    errors.append(f"❌ Missing union variant: {type_name}::{variant}")
                    union_errors += 1

    if union_errors == 0:
        print(f"✅ All union type variants are present")
    else:
        print(f"❌ {union_errors} union variants missing")

    # ── 4. Report ────────────────────────────────────────────────────────────
    print(f"\n{'='*50}")
    print(f"📊 Validation Summary")
    print(f"  Types:   {len(all_types) - len(missing_types)}/{len(all_types)} ✅")
    print(f"  Methods: {len(all_methods) - len(missing_methods)}/{len(all_methods)} ✅")

    if warnings:
        print(f"\n⚠️  Warnings ({len(warnings)}):")
        for w in warnings[:10]:
            print(f"  {w}")

    if errors:
        print(f"\n❌ Errors ({len(errors)}):")
        for e in errors[:20]:
            print(f"  {e}")
        print(f"\nValidation FAILED with {len(errors)} error(s)")
        sys.exit(1)
    else:
        print(f"\n✅ Validation passed!")
        sys.exit(0)


if __name__ == "__main__":
    main()
