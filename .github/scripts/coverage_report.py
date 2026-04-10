#!/usr/bin/env python3

import json
import re
import sys
from pathlib import Path

# -------------------------------------------------
# Telegram abstract / conceptual types
# -------------------------------------------------
IGNORED_TYPES = {
    "InputFile",
    "InputMedia",
}

# -------------------------------------------------
# Helpers
# -------------------------------------------------
def read(path: str) -> str:
    return Path(path).read_text(encoding="utf-8")


def snake_case(name: str) -> str:
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s)
    return s.lower()


# -------------------------------------------------
# Args  - accepts an optional --markdown flag
# -------------------------------------------------
args = sys.argv[1:]
markdown = "--markdown" in args
args = [a for a in args if a != "--markdown"]

if len(args) != 3:
    print("Usage: coverage_report.py api.json gen_types.rs gen_methods.rs [--markdown]")
    sys.exit(2)

api_path, types_path, methods_path = args

api        = json.loads(read(api_path))
types_src  = read(types_path)
methods_src = read(methods_path)

all_types   = api.get("types", {})
all_methods = api.get("methods", {})

implemented_types  = []
missing_types      = []
implemented_methods = []
missing_methods    = []

# -------------------------------------------------
# Type coverage
# -------------------------------------------------
for name, info in all_types.items():
    if name in IGNORED_TYPES:
        implemented_types.append(name)
        continue

    is_union = bool(info.get("subtypes"))
    pattern  = f"pub enum {name}" if is_union else f"pub struct {name}"

    if pattern in types_src:
        implemented_types.append(name)
    else:
        missing_types.append(name)

# -------------------------------------------------
# Method coverage
# Methods are generated as:  pub async fn snake_name
# e.g. sendMessage -> pub async fn send_message
# -------------------------------------------------
gen_fns = set(re.findall(r'pub async fn (\w+)', methods_src))

for name in all_methods:
    fn_name = snake_case(name)
    if fn_name in gen_fns:
        implemented_methods.append(name)
    else:
        missing_methods.append(name)

# -------------------------------------------------
# Compute stats
# -------------------------------------------------
total_types   = len(all_types)
total_methods = len(all_methods)

types_covered   = len(implemented_types)
methods_covered = len(implemented_methods)

types_pct   = int((types_covered   / total_types)   * 100) if total_types   else 100
methods_pct = int((methods_covered / total_methods) * 100) if total_methods else 100

# -------------------------------------------------
# Output - plain text or Markdown
# -------------------------------------------------
if markdown:
    print("## 📊 tgbotrs API Coverage - Telegram Bot API\n")
    print("| Category | Covered | Total | % |")
    print("|----------|--------:|------:|--:|")
    print(f"| Types    | {types_covered} | {total_types} | {types_pct}% |")
    print(f"| Methods  | {methods_covered} | {total_methods} | {methods_pct}% |")

    if missing_types:
        print("\n### ⚠️ Missing Types\n")
        print("```")
        for t in missing_types:
            print(f"  {t}")
        print("```")

    if missing_methods:
        print("\n### ⚠️ Missing Methods\n")
        print("```")
        for m in missing_methods:
            print(f"  {m}")
        print("```")

    if not missing_types and not missing_methods:
        print("\n✅ **Full coverage - all types and methods implemented.**")
else:
    print("=" * 60)
    print("📊 tgbotrs API Coverage - Telegram Bot API")
    print("=" * 60)
    print(f"  Types:   {types_covered}/{total_types}  ({types_pct}%)")
    print(f"  Methods: {methods_covered}/{total_methods}  ({methods_pct}%)")

    if missing_types:
        print("\n⚠️  Missing Types:")
        print(" ", missing_types)

    if missing_methods:
        print("\n⚠️  Missing Methods:")
        print(" ", missing_methods)

# -------------------------------------------------
# Strict CI enforcement (both modes must fail hard)
# -------------------------------------------------
if missing_types or missing_methods:
    sys.exit(1)
