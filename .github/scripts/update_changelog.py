#!/usr/bin/env python3
"""
update_changelog.py - Prepends a new entry to CHANGELOG.md.

Usage: python3 update_changelog.py CHANGELOG.md <crate_version> <api_version> <date>
"""

import sys
import json
from pathlib import Path


HEADER = """# Changelog

All notable changes to **tgbotrs** are documented here.

Format: [Semantic Versioning](https://semver.org/)  
Auto-generated API updates use the [Telegram Bot API spec](https://github.com/PaulSonOfLars/telegram-bot-api-spec).

---
"""


def load_diff(path="/tmp/diff_report.json"):
    try:
        return json.load(open(path))
    except Exception:
        return None


def main():
    if len(sys.argv) < 5:
        print("Usage: update_changelog.py CHANGELOG.md <crate_ver> <api_ver> <date>")
        sys.exit(1)

    changelog_path = Path(sys.argv[1])
    crate_ver = sys.argv[2]
    api_ver = sys.argv[3]
    date = sys.argv[4]

    diff = load_diff()

    # Build new entry
    entry_lines = []
    entry_lines.append(f"## [{crate_ver}] - {date}")
    entry_lines.append(f"")
    entry_lines.append(f"### Telegram Bot API: `{api_ver}`")
    entry_lines.append(f"")

    if diff:
        added_t = diff.get("added_types", [])
        removed_t = diff.get("removed_types", [])
        added_m = diff.get("added_methods", [])
        removed_m = diff.get("removed_methods", [])
        changed_t = diff.get("changed_types", {})
        changed_m = diff.get("changed_methods", {})

        if added_t:
            entry_lines.append("**New Types:**")
            for t in added_t:
                entry_lines.append(f"- `{t}`")
            entry_lines.append("")

        if removed_t:
            entry_lines.append("**Removed Types:**")
            for t in removed_t:
                entry_lines.append(f"- ~~`{t}`~~")
            entry_lines.append("")

        if added_m:
            entry_lines.append("**New Methods:**")
            for m in added_m:
                entry_lines.append(f"- `{m}`")
            entry_lines.append("")

        if removed_m:
            entry_lines.append("**Removed Methods:**")
            for m in removed_m:
                entry_lines.append(f"- ~~`{m}`~~")
            entry_lines.append("")

        if changed_t:
            entry_lines.append(f"**Changed Types** ({len(changed_t)}):")
            for type_name, fields in sorted(changed_t.items()):
                entry_lines.append(f"- `{type_name}`")
                for field_name, desc in sorted(fields.items()):
                    entry_lines.append(f"  - `{field_name}`: {desc}")
            entry_lines.append("")

        if changed_m:
            entry_lines.append(f"**Changed Methods** ({len(changed_m)}):")
            for method_name, fields in sorted(changed_m.items()):
                entry_lines.append(f"- `{method_name}`")
                for field_name, desc in sorted(fields.items()):
                    entry_lines.append(f"  - `{field_name}`: {desc}")
            entry_lines.append("")
    else:
        entry_lines.append("Auto-generated from latest Telegram Bot API spec.")
        entry_lines.append("")

    entry_lines.append("---")
    entry_lines.append("")

    new_entry = "\n".join(entry_lines)

    # Read or initialize changelog
    if changelog_path.exists():
        existing = changelog_path.read_text()
        # Insert after the header separator
        if "---\n" in existing:
            idx = existing.index("---\n") + 4
            updated = existing[:idx] + "\n" + new_entry + existing[idx:]
        else:
            updated = HEADER + "\n" + new_entry + existing
    else:
        updated = HEADER + "\n" + new_entry

    changelog_path.write_text(updated)

    # Also write release notes for GitHub release
    with open("/tmp/release_notes.md", "w") as f:
        f.write(f"## tgbotrs v{crate_ver}\n\n")
        f.write(f"Auto-generated from **Telegram Bot API {api_ver}**.\n\n")
        f.write(new_entry)

    print(f"✅ CHANGELOG.md updated for v{crate_ver}")


if __name__ == "__main__":
    main()
