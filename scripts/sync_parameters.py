#!/usr/bin/env python3
"""
Validate parameters between Python type stubs and Rust source files.

This script extracts parameter/variant lists from Python .pyi files
and compares them with corresponding Rust implementations, reporting
any differences found.

Usage:
    python scripts/sync_parameters.py

Output:
    Only shows differences if parameters don't match.
    Exit code 0 if all match, 1 if any mismatches found.
"""

import re
import sys
from pathlib import Path
from dataclasses import dataclass
from typing import Optional


@dataclass
class ParameterMapping:
    """Maps a Python type to its Rust implementation."""
    pyi_file: str          # Path to .pyi file
    pyi_name: str          # Name in Python (enum/type)
    rust_file: str         # Path to Rust file
    rust_name: str         # Name in Rust


# Parameters to validate: (pyi source -> Rust target)
PARAMETERS_TO_CHECK = [
    ParameterMapping(
        pyi_file="icedpygui/__init__.pyi",
        pyi_name="ButtonParam",
        rust_file="src/widgets/ipg_button.rs",
        rust_name="ButtonParam",
    ),
    ParameterMapping(
        pyi_file="icedpygui/__init__.pyi",
        pyi_name="ButtonStyleParam",
        rust_file="src/widgets/ipg_button.rs",
        rust_name="ButtonStyleParam",
    ),
]


def extract_enum_variants_from_pyi(
    pyi_path: Path, enum_name: str
) -> Optional[list[str]]:
    """
    Extract enum/class variant names from Python .pyi file.

    Handles both:
    - Enum-style: name = ...
    - Type-style: name: type

    Returns:
        List of variant names, or None if not found.
    """
    with open(pyi_path, 'r', encoding="utf-8") as f:
        content = f.read()

    # Find class definition
    pattern = (
        rf'class\s+{re.escape(enum_name)}\s*(?:\([^)]*\))?\s*:'
    )
    match = re.search(pattern, content)
    if not match:
        return None

    # Find the class body
    start_pos = match.end()
    lines = content[start_pos:].split('\n')

    # First, skip past the docstring
    in_docstring = False
    docstring_end_idx = 0

    for idx, line in enumerate(lines):
        # Stop at next class or function definition at indent level 0
        if idx > 0 and line and not line[0].isspace():
            break

        stripped = line.strip()

        # Check for docstring start/end
        if '"""' in stripped:
            if not in_docstring:
                in_docstring = True
                # Check if docstring ends on same line
                if stripped.count('"""') >= 2:
                    in_docstring = False
                    docstring_end_idx = idx + 1
                    break
            else:
                in_docstring = False
                docstring_end_idx = idx + 1
                break
        elif "'''" in stripped:
            if not in_docstring:
                in_docstring = True
                # Check if docstring ends on same line
                if stripped.count("'''") >= 2:
                    in_docstring = False
                    docstring_end_idx = idx + 1
                    break
            else:
                in_docstring = False
                docstring_end_idx = idx + 1
                break

    # Now extract variants from lines after docstring
    variants = []
    for line in lines[docstring_end_idx:]:
        # Stop at next class or function definition at indent level 0
        if line and not line[0].isspace():
            break

        stripped = line.strip()
        if not stripped or stripped.startswith('#'):
            continue

        # Extract variant names
        # Pattern 1: name = ... (Enum-style)
        if '=' in stripped and ':' not in stripped.split('=')[0]:
            var_name = stripped.split('=')[0].strip()
            if var_name and var_name[0].isupper():
                variants.append(var_name)
        # Pattern 2: name: type (Type annotation style)
        elif ':' in stripped:
            var_name = stripped.split(':')[0].strip()
            if var_name and var_name[0].isupper():
                variants.append(var_name)

    return variants if variants else None


def extract_docstring_parameters(
    pyi_path: Path, enum_name: str
) -> Optional[list[str]]:
    """
    Extract parameter names from the docstring of a class.

    Parses the Parameters section of a docstring and extracts
    the parameter names (capitalized, before the colon).

    Returns:
        List of parameter names from docstring, or None if not found.
    """
    with open(pyi_path, 'r', encoding="utf-8") as f:
        content = f.read()

    # Find class definition and docstring
    pattern = (
        rf'class\s+{re.escape(enum_name)}\s*(?:\([^)]*\))?\s*:\s*"""'
    )
    match = re.search(pattern, content)
    if not match:
        return None

    # Extract docstring content
    start_pos = match.end()
    docstring_end = content.find('"""', start_pos)
    if docstring_end == -1:
        return None

    docstring = content[start_pos:docstring_end]

    # Find Parameters section
    params_match = re.search(r'Parameters\s*\n\s*-{2,}', docstring)
    if not params_match:
        return None

    params_start = params_match.end()
    params_section = docstring[params_start:]

    # Extract parameter names (capitalized word before colon)
    params = []
    section_headers = (
        'Returns', 'Examples', 'Raises', 'Notes', 'See', 'Attributes'
    )

    for line in params_section.split('\n'):
        stripped = line.strip()
        if not stripped:
            continue

        # Stop at next section (Returns, Examples, etc.)
        # Check if this is exactly a section header (word followed by newline,
        # no indentation continuation)
        if stripped and not line.startswith((' ', '\t')):
            # This is a non-indented line; check if it's a section header
            if stripped.split(':')[0].strip() in section_headers:
                break

        # Extract "ParamName:" pattern
        # Only extract if line starts with indentation (parameter line)
        if line.startswith((' ', '\t')) and ':' in stripped:
            param_name = stripped.split(':')[0].strip()
            if param_name and param_name[0].isupper():
                params.append(param_name)

    return params if params else None


def extract_enum_variants_from_rust(
    rust_file: Path, enum_name: str
) -> Optional[list[str]]:
    """
    Extract enum variant names from Rust source file.

    Looks for: pub enum EnumName { ... } patterns.

    Returns:
        List of variant names, or None if not found.
    """
    with open(rust_file, 'r', encoding="utf-8") as f:
        content = f.read()

    # Find enum/pub enum definition
    pattern = (
        rf'pub\s+(?:enum|struct)\s+{re.escape(enum_name)}\s*\{{'
    )
    match = re.search(pattern, content)
    if not match:
        return None

    # Extract variants between braces
    start_pos = match.end()
    brace_count = 1
    end_pos = start_pos

    while end_pos < len(content) and brace_count > 0:
        if content[end_pos] == '{':
            brace_count += 1
        elif content[end_pos] == '}':
            brace_count -= 1
        end_pos += 1

    enum_body = content[start_pos:end_pos - 1]
    lines = enum_body.split('\n')

    variants = []

    for line in lines:
        stripped = line.strip()
        # Skip empty lines and comments
        if not stripped or stripped.startswith('//'):
            continue

        # Extract variant names (before = or ,)
        for sep in [',', '=', '(', '{']:
            if sep in stripped:
                var_name = stripped.split(sep)[0].strip()
                if var_name and not any(
                    var_name.startswith(kw)
                    for kw in ['#[', '/*', '*']
                ):
                    variants.append(var_name)
                break

    return variants if variants else None


def compare_parameters(
    pyi_params: Optional[list[str]],
    rust_params: Optional[list[str]],
    name: str,
) -> bool:
    """
    Compare Python and Rust parameters, print differences.

    Returns:
        True if they match, False if mismatch.
    """
    if pyi_params is None:
        print(f"❌ {name}: Could not extract from Python")
        return False

    if rust_params is None:
        print(f"❌ {name}: Could not extract from Rust")
        return False

    if pyi_params == rust_params:
        print(f"✓ {name}: Parameters match")
        return True

    # Show differences
    pyi_set = set(pyi_params)
    rust_set = set(rust_params)

    only_in_python = pyi_set - rust_set
    only_in_rust = rust_set - pyi_set

    print(f"\n⚠️  {name}: Parameter mismatch")
    print(f"   Python: {', '.join(pyi_params)}")
    print(f"   Rust:   {', '.join(rust_params)}")

    if only_in_python:
        py_only = ', '.join(sorted(only_in_python))
        print(f"   Missing in Rust: {py_only}")
    if only_in_rust:
        rust_only = ', '.join(sorted(only_in_rust))
        print(f"   Extra in Rust:  {rust_only}")

    return False


def compare_docstring_to_enum(
    docstring_params: Optional[list[str]],
    enum_params: Optional[list[str]],
    name: str,
) -> bool:
    """
    Compare docstring parameters to actual enum values.

    Returns:
        True if they match, False if mismatch.
    """
    if docstring_params is None:
        print(f"❌ {name}: Could not extract docstring parameters")
        return False

    if enum_params is None:
        print(f"❌ {name}: Could not extract enum parameters")
        return False

    if docstring_params == enum_params:
        print(f"✓ {name}: Docstring matches enum values")
        return True

    # Show differences
    doc_set = set(docstring_params)
    enum_set = set(enum_params)

    only_in_doc = doc_set - enum_set
    only_in_enum = enum_set - doc_set

    print(f"\n⚠️  {name}: Docstring/Enum mismatch")
    print(f"   Docstring: {', '.join(docstring_params)}")
    print(f"   Enum:      {', '.join(enum_params)}")

    if only_in_doc:
        doc_only = ', '.join(sorted(only_in_doc))
        print(f"   In docstring but not enum: {doc_only}")
    if only_in_enum:
        enum_only = ', '.join(sorted(only_in_enum))
        print(f"   In enum but not docstring:  {enum_only}")

    return False


def main():
    """Main entry point."""
    repo_root = Path(__file__).parent.parent

    if not PARAMETERS_TO_CHECK:
        print("⚠️  No parameters configured to check!")
        return 1

    print(f"🔍 Checking {len(PARAMETERS_TO_CHECK)} parameter set(s)...\n")

    results = []
    for mapping in PARAMETERS_TO_CHECK:
        pyi_path = repo_root / mapping.pyi_file
        rust_path = repo_root / mapping.rust_file

        # Validate files exist
        if not pyi_path.exists():
            print(f"❌ .pyi file not found: {pyi_path}")
            results.append((mapping.pyi_name, False))
            continue

        if not rust_path.exists():
            print(f"❌ Rust file not found: {rust_path}")
            results.append((mapping.pyi_name, False))
            continue

        # VERTICAL VALIDATION: Docstring vs enum values in Python
        print(f"\n📋 {mapping.pyi_name}")
        print("─" * 50)

        docstring_params = extract_docstring_parameters(
            pyi_path, mapping.pyi_name
        )
        enum_params = extract_enum_variants_from_pyi(
            pyi_path, mapping.pyi_name
        )

        docstring_match = compare_docstring_to_enum(
            docstring_params, enum_params, mapping.pyi_name
        )

        # HORIZONTAL VALIDATION: Python vs Rust enum values
        pyi_params = extract_enum_variants_from_pyi(
            pyi_path, mapping.pyi_name
        )
        rust_params = extract_enum_variants_from_rust(
            rust_path, mapping.rust_name
        )

        horizontal_match = compare_parameters(
            pyi_params, rust_params, f"{mapping.pyi_name} (Python ↔ Rust)"
        )

        # Both must pass
        overall_match = docstring_match and horizontal_match
        results.append((mapping.pyi_name, overall_match))

    # Summary
    print("\n" + "=" * 60)
    successful = sum(1 for _, match in results if match)
    total = len(results)

    if successful == total:
        print(f"✅ All parameters match! ({successful}/{total})")
        return 0
    else:
        print(f"❌ Mismatches found: {total - successful} of {total}")
        return 1


if __name__ == '__main__':
    sys.exit(main())
