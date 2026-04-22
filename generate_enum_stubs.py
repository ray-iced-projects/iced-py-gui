"""
Generate .pyi stub entries for specific PyO3 enums by introspecting
the compiled module. Run after `maturin develop`.

Usage:
    python generate_enum_stubs.py

Prints the .pyi class text to stdout. Copy/paste into icedpygui.pyi,
or use --patch to auto-replace existing entries in the .pyi file.
"""
import sys
import argparse

# The enums you want to auto-generate stubs for
ENUMS_TO_GENERATE = [
    "Arrow",
    "ButtonParam",
    "Color",
    "Icon",
    "Image"
]

PYI_PATH = "icedpygui/icedpygui.pyi"


def get_enum_members(cls):
    """Get enum variant names from a PyO3 enum class."""
    members = []
    for name in dir(cls):
        if name.startswith('_'):
            continue
        members.append(name)
    return members


def generate_stub(cls_name, members):
    """Generate .pyi class text for an enum."""
    lines = [f"class {cls_name}:"]
    for m in members:
        lines.append(f"    {m} = ...")
    lines.append("")
    return "\n".join(lines)


def patch_pyi(pyi_path, cls_name, new_stub):
    """Replace an existing class block in the .pyi file."""
    with open(pyi_path, "r") as f:
        content = f.read()

    # Find the class block
    marker = f"class {cls_name}:"
    start = content.find(marker)
    if start == -1:
        # Append to end of file
        if not content.endswith("\n"):
            content += "\n"
        content += "\n" + new_stub + "\n"
        with open(pyi_path, "w") as f:
            f.write(content)
        print(f"  Appended '{cls_name}' to {pyi_path}")
        return True

    # Find the end of the class block (next class or end of file)
    # Look for next top-level class definition or end of file
    next_class = content.find("\nclass ", start + len(marker))
    if next_class == -1:
        end = len(content)
    else:
        end = next_class + 1  # include the newline before next class

    # Preserve any docstring from the original
    old_block = content[start:end]
    doc_lines = []
    in_docstring = False
    for line in old_block.split("\n")[1:]:  # skip the "class X:" line
        stripped = line.strip()
        if stripped.startswith('"""') and not in_docstring:
            in_docstring = True
            doc_lines.append(line)
            if stripped.endswith('"""') and len(stripped) > 3:
                in_docstring = False
            continue
        if in_docstring:
            doc_lines.append(line)
            if stripped.endswith('"""'):
                in_docstring = False
            continue
        if not in_docstring and doc_lines:
            break
        if not stripped and not doc_lines:
            continue
        break

    # Build new block with preserved docstring
    if doc_lines:
        new_lines = [f"class {cls_name}:"]
        new_lines.extend(doc_lines)
        # Add the members from new_stub (skip the "class X:" line)
        for line in new_stub.split("\n")[1:]:
            if line.strip():
                new_lines.append(line)
        new_block = "\n".join(new_lines) + "\n\n"
    else:
        new_block = new_stub + "\n"

    content = content[:start] + new_block + content[end:]

    with open(pyi_path, "w") as f:
        f.write(content)

    print(f"  Patched '{cls_name}' in {pyi_path}")
    return True


def main():
    parser = argparse.ArgumentParser(description="Generate .pyi stubs for PyO3 enums")
    parser.add_argument("--patch", action="store_true",
                        help="Auto-replace entries in the .pyi file")
    args = parser.parse_args()

    import icedpygui

    for enum_name in ENUMS_TO_GENERATE:
        cls = getattr(icedpygui, enum_name, None)
        if cls is None:
            print(f"Warning: '{enum_name}' not found in icedpygui module", file=sys.stderr)
            continue

        members = get_enum_members(cls)
        stub = generate_stub(enum_name, members)

        if args.patch:
            patch_pyi(PYI_PATH, enum_name, stub)
        else:
            print(stub)
            print()


if __name__ == "__main__":
    main()
