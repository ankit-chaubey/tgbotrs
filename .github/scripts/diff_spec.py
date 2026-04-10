#!/usr/bin/env python3
"""
diff_spec.py - Compute a semantic diff between two Telegram Bot API spec files.

Usage: python3 diff_spec.py old_api.json new_api.json output_report.json
"""

import json
import sys
from pathlib import Path


def load(path):
    try:
        return json.load(open(path))
    except (FileNotFoundError, json.JSONDecodeError):
        return {"types": {}, "methods": {}, "version": "unknown", "release_date": ""}


def fields_by_name(fields):
    return {f["name"]: f for f in (fields or [])}


def diff_fields(old_fields, new_fields, context_name):
    """Compare two lists of fields and return human-readable changes."""
    old = fields_by_name(old_fields)
    new = fields_by_name(new_fields)
    changes = {}

    for name in set(list(old.keys()) + list(new.keys())):
        if name not in old:
            changes[name] = f"**Added** field `{name}` ({', '.join(new[name]['types'])})"
        elif name not in new:
            changes[name] = f"**Removed** field `{name}`"
        else:
            o, n = old[name], new[name]
            sub = []
            if o["types"] != n["types"]:
                sub.append(f"types changed: `{o['types']}` → `{n['types']}`")
            if o["required"] != n["required"]:
                req_str = "required" if n["required"] else "optional"
                sub.append(f"now **{req_str}**")
            if o.get("description") != n.get("description"):
                sub.append("description updated")
            if sub:
                changes[name] = "; ".join(sub)

    return changes


def diff_specs(old, new):
    old_types = old.get("types", {})
    new_types = new.get("types", {})
    old_methods = old.get("methods", {})
    new_methods = new.get("methods", {})

    all_type_names = set(list(old_types.keys()) + list(new_types.keys()))
    all_method_names = set(list(old_methods.keys()) + list(new_methods.keys()))

    added_types = sorted([n for n in all_type_names if n not in old_types])
    removed_types = sorted([n for n in all_type_names if n not in new_types])
    added_methods = sorted([n for n in all_method_names if n not in old_methods])
    removed_methods = sorted([n for n in all_method_names if n not in new_methods])

    # Detailed field-level diffs for existing types
    changed_types = {}
    for name in all_type_names:
        if name not in old_types or name not in new_types:
            continue
        old_t = old_types[name]
        new_t = new_types[name]

        changes = diff_fields(
            old_t.get("fields", []),
            new_t.get("fields", []),
            name,
        )

        # Check for subtype changes (union type membership)
        old_subs = sorted(old_t.get("subtypes", []))
        new_subs = sorted(new_t.get("subtypes", []))
        if old_subs != new_subs:
            added_variants = sorted(set(new_subs) - set(old_subs))
            removed_variants = sorted(set(old_subs) - set(new_subs))
            if added_variants:
                changes["[variants]"] = f"Added subtypes: {added_variants}"
            if removed_variants:
                changes["[variants]"] = changes.get("[variants]", "") + f" Removed subtypes: {removed_variants}"

        # Description changes
        if old_t.get("description") != new_t.get("description"):
            changes["[description]"] = "Description updated"

        if changes:
            changed_types[name] = changes

    # Detailed field-level diffs for existing methods
    changed_methods = {}
    for name in all_method_names:
        if name not in old_methods or name not in new_methods:
            continue
        old_m = old_methods[name]
        new_m = new_methods[name]

        changes = diff_fields(
            old_m.get("fields", []),
            new_m.get("fields", []),
            name,
        )

        # Check return type changes
        if old_m.get("returns") != new_m.get("returns"):
            changes["[returns]"] = f"Return type changed: `{old_m.get('returns')}` → `{new_m.get('returns')}`"

        if changes:
            changed_methods[name] = changes

    return {
        "old_version": old.get("version", "unknown"),
        "new_version": new.get("version", "unknown"),
        "new_date": new.get("release_date", ""),
        "old_date": old.get("release_date", ""),

        "added_types": added_types,
        "removed_types": removed_types,
        "changed_types": changed_types,

        "added_methods": added_methods,
        "removed_methods": removed_methods,
        "changed_methods": changed_methods,

        "stats": {
            "old_types": len(old_types),
            "new_types": len(new_types),
            "old_methods": len(old_methods),
            "new_methods": len(new_methods),
        },
    }


def main():
    if len(sys.argv) < 4:
        print("Usage: diff_spec.py <old.json> <new.json> <report.json>")
        sys.exit(1)

    old = load(sys.argv[1])
    new = load(sys.argv[2])
    report_path = sys.argv[3]

    report = diff_specs(old, new)

    with open(report_path, "w") as f:
        json.dump(report, f, indent=2)

    # Print summary to stdout
    print(f"📦 Spec diff: {report['old_version']} → {report['new_version']}")
    print(f"  ✅ Added types:    {len(report['added_types'])}")
    print(f"  ❌ Removed types:  {len(report['removed_types'])}")
    print(f"  🔄 Changed types:  {len(report['changed_types'])}")
    print(f"  ✅ Added methods:  {len(report['added_methods'])}")
    print(f"  ❌ Removed methods:{len(report['removed_methods'])}")
    print(f"  🔄 Changed methods:{len(report['changed_methods'])}")
    print(f"  Report → {report_path}")


if __name__ == "__main__":
    main()
