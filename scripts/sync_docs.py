#!/usr/bin/env python3
"""
Sync documentation from .pyi files (source of truth) to Rust source files.

This script extracts docstrings from icedpygui Python type stub files
and updates corresponding Rust rustdoc comments, keeping documentation
in a single place: the .pyi files.

Usage:
    python scripts/sync_docs.py --dry-run  # Preview without modifying
    python scripts/sync_docs.py --update   # Apply changes to Rust

To add more functions to sync, add entries to FUNCTIONS_TO_SYNC list below.
"""

import re
import sys
from pathlib import Path
from dataclasses import dataclass
from typing import Optional


@dataclass
class FunctionMapping:
    """Maps a Python function to its Rust implementation."""
    pyi_file: str          # Path to .pyi file
    func_name: str         # Function name in .pyi
    rust_file: str         # Path to Rust file
    rust_func_name: str    # Function name in Rust


# Functions to sync: (pyi source -> Rust target)
# Add more entries here as .pyi files are updated
FUNCTIONS_TO_SYNC = [
    FunctionMapping(
        pyi_file="icedpygui/__init__.pyi",
        func_name="add_button",
        rust_file="src/py_api/button.rs",
        rust_func_name="add_button",
    ),
    FunctionMapping(
            pyi_file="icedpygui/__init__.pyi",
            func_name="add_button_style",
            rust_file="src/py_api/button.rs",
            rust_func_name="add_button_style",
        ),
    # Template for adding more:
    # FunctionMapping(
    #     pyi_file="icedpygui/__init__.pyi",
    #     func_name="add_text",
    #     rust_file="src/py_api/text.rs",
    #     rust_func_name="add_text",
    # ),
    # FunctionMapping(
    #     pyi_file="icedpygui/icedpygui.pyi",
    #     func_name="add_button_style",
    #     rust_file="src/py_api/button.rs",
    #     rust_func_name="add_button_style",
    # ),
]


def extract_docstring_from_pyi(
    pyi_path: Path, func_name: str
) -> Optional[str]:
    """
    Extract docstring from Python function in .pyi file.

    Handles multi-line function signatures.

    Returns:
        The docstring content, or None if not found.
    """
    with open(pyi_path, 'r', encoding="utf-8") as f:
        lines = f.readlines()

    # Find function definition
    func_pattern = rf'def {re.escape(func_name)}\s*\('
    start_line = None

    for i, line in enumerate(lines):
        if re.search(func_pattern, line):
            start_line = i
            break

    if start_line is None:
        return None

    # Find the matching closing paren and the docstring after it
    paren_count = 0
    found_open = False
    i = start_line

    while i < len(lines):
        line = lines[i]

        for char in line:
            if char == '(':
                paren_count += 1
                found_open = True
            elif char == ')':
                paren_count -= 1
                if found_open and paren_count == 0:
                    # Found the closing paren of function signature
                    # Now look for the docstring
                    rest_of_line = line[line.index(char)+1:]

                    # Check if docstring starts on this line
                    if '"""' in rest_of_line:
                        # Find the docstring on this line and subsequent lines
                        doc_start = rest_of_line.index('"""') + 3
                        doc_text = rest_of_line[doc_start:]

                        # Get docstring end
                        if '"""' in doc_text:
                            doc_end = doc_text.index('"""')
                            return doc_text[:doc_end].strip()
                        else:
                            # Docstring spans multiple lines
                            doc_lines = [doc_text]
                            for j in range(i + 1, len(lines)):
                                line_text = lines[j]
                                if '"""' in line_text:
                                    end_idx = line_text.index('"""')
                                    doc_lines.append(line_text[:end_idx])
                                    return '\n'.join(doc_lines).strip()
                                else:
                                    doc_lines.append(line_text.rstrip())
                    else:
                        # Docstring is on next line
                        for j in range(i + 1, len(lines)):
                            next_line = lines[j].strip()
                            if next_line.startswith('"""'):
                                # Found docstring
                                doc_text = next_line[3:]
                                if '"""' in doc_text:
                                    doc_end = doc_text.index('"""')
                                    return doc_text[:doc_end].strip()
                                else:
                                    # Multi-line docstring
                                    doc_lines = [doc_text]
                                    for k in range(j + 1, len(lines)):
                                        line_k = lines[k]
                                        if '"""' in line_k:
                                            end_idx = line_k.index('"""')
                                            doc_lines.append(line_k[:end_idx])
                                            return '\n'.join(doc_lines).strip()
                                        else:
                                            doc_lines.append(line_k.rstrip())
                            elif next_line and not next_line.startswith('#'):
                                break
                    return None
        i += 1

    return None


def format_as_rustdoc(docstring: str) -> str:
    """
    Convert Python docstring to Rust rustdoc format.

    - Prepends '/// ' to each line
    - Handles multi-line docstrings
    """
    lines = docstring.split('\n')
    rustdoc_lines = [
        f'/// {line}' if line.strip() else '///'
        for line in lines
    ]
    return '\n'.join(rustdoc_lines)


def extract_parameters_from_pyi(
    pyi_path: Path, func_name: str
) -> Optional[list[str]]:
    """
    Extract parameter names from Python function in .pyi file.

    Returns:
        List of parameter names, or None if not found.
    """
    with open(pyi_path, 'r', encoding="utf-8") as f:
        lines = f.readlines()

    func_pattern = rf'def {re.escape(func_name)}\s*\('
    start_line = None

    for i, line in enumerate(lines):
        if re.search(func_pattern, line):
            start_line = i
            break

    if start_line is None:
        return None

    # Extract parameters from function signature
    paren_count = 0
    found_open = False
    param_text = []
    i = start_line

    while i < len(lines):
        line = lines[i]

        for char in line:
            if char == '(':
                paren_count += 1
                found_open = True
            elif char == ')':
                paren_count -= 1
                if found_open and paren_count == 0:
                    # Found closing paren - extract params using colon parsing
                    param_str = ''.join(param_text)
                    params = []

                    # Parse by colons (which uniquely mark param: type)
                    for colon_idx, char in enumerate(param_str):
                        if char == ':':
                            # Find param name: go back to last comma
                            name_start = colon_idx - 1
                            while (name_start >= 0 and
                                   param_str[name_start] not in (',', '(')):
                                name_start -= 1
                            name_start += 1

                            param_name = (
                                param_str[name_start:colon_idx].strip()
                            )
                            if param_name and param_name != '*':
                                params.append(param_name)

                    return params if params else None
            elif found_open:
                param_text.append(char)
        i += 1

    return None


def extract_parameters_from_rust(
    rust_file: Path, func_name: str
) -> Optional[list[str]]:
    """
    Extract parameters from Rust function's PyO3 signature.

    Looks for #[pyo3(signature = (...))] decorator.

    Returns:
        List of parameter names, or None if not found.
    """
    with open(rust_file, 'r', encoding="utf-8") as f:
        lines = f.readlines()

    func_pattern = rf'pub fn {re.escape(func_name)}\s*\('
    func_line = None

    for i, line in enumerate(lines):
        if re.search(func_pattern, line):
            func_line = i
            break

    if func_line is None:
        return None

    # Search backwards from the function line for #[pyo3(signature = (...))]
    for i in range(func_line - 1, -1, -1):
        if '#[pyo3(signature' in lines[i]:
            # Found the signature decorator - extract parameters from it
            paren_count = 0
            found_open = False
            param_text = []

            # Look for the opening paren of signature tuple
            start_idx = -1
            for idx, char in enumerate(lines[i]):
                if char == '(' and idx > lines[i].find('signature'):
                    start_idx = idx
                    break

            if start_idx == -1:
                continue

            # Extract parameters across lines
            j = i
            while j < len(lines):
                line_to_process = lines[j][start_idx:] if j == i else lines[j]

                for char in line_to_process:
                    if char == '(':
                        paren_count += 1
                        found_open = True
                    elif char == ')':
                        paren_count -= 1
                        if found_open and paren_count == 0:
                            # Found closing paren
                            param_str = ''.join(param_text)
                            params = []
                            for p in param_str.split(','):
                                p = p.strip()
                                if p:  # Skip empty strings
                                    param_name = p.split('=')[0].strip()
                                    if param_name and param_name != '*':
                                        params.append(param_name)
                            return params if params else None
                    elif found_open:
                        param_text.append(char)

                j += 1

            return None

    return None


def validate_parameters(
    pyi_params: Optional[list[str]],
    rust_params: Optional[list[str]],
) -> bool:
    """
    Validate Python and Rust signatures have compatible parameters.

    Returns:
        True if parameters match or can't determine, False if mismatch.
    """
    # Both extraction failed - can't determine, allow sync
    if pyi_params is None and rust_params is None:
        print("   ⚠️  Could not extract parameters from either source")
        return True

    # One extraction failed - real problem
    if pyi_params is None:
        print("   ❌ Could not extract parameters from Python function")
        return False

    if rust_params is None:
        print("   ❌ Could not extract parameters from Rust signature")
        return False

    # Both extracted - compare them
    if pyi_params == rust_params:
        params_display = ', '.join(pyi_params) if pyi_params else (
            '(no parameters)'
        )
        print(f"   ✓ Parameters match: {params_display}")
        return True
    else:
        print("   ⚠️  Parameter mismatch:")
        pyi_set = set(pyi_params) if pyi_params else set()
        rust_set = set(rust_params) if rust_params else set()

        only_in_python = pyi_set - rust_set
        only_in_rust = rust_set - pyi_set

        # Show full lists for context
        py_list = ', '.join(pyi_params) if pyi_params else '(none)'
        rust_list = ', '.join(rust_params) if rust_params else '(none)'
        print(f"       Python: {py_list}")
        print(f"       Rust:   {rust_list}")

        # Show what's different
        if only_in_python or only_in_rust:
            print("\n       Differences Found:")
        if only_in_python:
            py_only = ', '.join(sorted(only_in_python))
            print(f"       Missing in Rust: {py_only}")
        if only_in_rust:
            rust_only = ', '.join(sorted(only_in_rust))
            print(f"       Extra in Rust:  {rust_only}")

        return False


def find_rust_function(
    rust_file: Path, func_name: str
) -> Optional[tuple[int, int, int]]:
    """
    Find documentation and function definition lines.

    Returns:
        Tuple of (doc_start, doc_end, func_def) or None.
        doc_start: first /// comment line
        doc_end: last /// comment line
        func_def: line with 'pub fn'
    """
    with open(rust_file, 'r', encoding="utf-8") as f:
        lines = f.readlines()

    # Find pub fn definition
    func_pattern = rf'pub fn {re.escape(func_name)}\s*\('

    for i, line in enumerate(lines):
        if re.search(func_pattern, line):
            # Found function definition at line i
            func_def_line = i

            # Search backwards for the last /// line
            # (end of documentation block)
            doc_end = i - 1
            while doc_end >= 0 and not lines[doc_end].strip().\
                    startswith('///'):
                doc_end -= 1

            # Now doc_end is at the last /// line, or -1 if not found
            if doc_end < 0:
                return None

            # Find first /// line
            doc_start = doc_end
            while doc_start > 0 and lines[doc_start - 1].strip().\
                    startswith('///'):
                doc_start -= 1

            return (doc_start, doc_end, func_def_line)

    return None


def replace_rustdoc(
    rust_file: Path,
    doc_start_line: int,
    doc_end_line: int,
    new_rustdoc: str,
) -> str:
    """
    Replace rustdoc block in Rust file.

    Args:
        rust_file: Path to Rust file
        doc_start_line: First /// line
        doc_end_line: Last /// line (inclusive)
        new_rustdoc: New rustdoc string

    Returns:
        Modified file content.
    """
    with open(rust_file, 'r', encoding="utf-8") as f:
        lines = f.readlines()

    # Replace doc lines with new rustdoc
    new_lines = (
        lines[:doc_start_line] +
        [new_rustdoc + '\n'] +
        lines[doc_end_line + 1:]
    )

    return ''.join(new_lines)


def sync_function(
    mapping: FunctionMapping,
    repo_root: Path,
    dry_run: bool = True,
) -> bool:
    """
    Sync documentation for a single function.

    Returns:
        True if successful, False otherwise.
    """
    pyi_path = repo_root / mapping.pyi_file
    rust_path = repo_root / mapping.rust_file

    # Validate files exist
    if not pyi_path.exists():
        print(f"❌ .pyi file not found: {pyi_path}")
        return False

    if not rust_path.exists():
        print(f"❌ Rust file not found: {rust_path}")
        return False

    # Extract docstring from .pyi
    print(f"\n📄 Syncing: {mapping.func_name}")
    print(f"   From: {mapping.pyi_file}")
    print(f"   To:   {mapping.rust_file}")

    docstring = extract_docstring_from_pyi(
        pyi_path, mapping.func_name
    )
    if not docstring:
        print(
            f"❌ Could not extract docstring for {mapping.func_name}"
        )
        return False

    # Convert to Rust format
    rustdoc = format_as_rustdoc(docstring)

    # Find function in Rust file
    func_lines = find_rust_function(
        rust_path, mapping.rust_func_name
    )
    if not func_lines:
        print(
            f"❌ Could not find function {mapping.rust_func_name} "
            f"in {rust_path}"
        )
        return False

    doc_start, doc_end, func_def = func_lines

    # Preview the change
    print(
        f"   ✓ Found function at lines {doc_start+1}-{doc_end+1} "
        f"(doc) and {func_def+1} (function)"
    )

    # Validate parameters
    print("\n   Validating parameters...")
    pyi_params = extract_parameters_from_pyi(pyi_path, mapping.func_name)
    rust_params = extract_parameters_from_rust(
        rust_path, mapping.rust_func_name
    )

    valid_params = validate_parameters(
        pyi_params, rust_params
    )

    if not valid_params:
        print("   ⚠️  Skipping sync due to parameter mismatch")
        return False

    print("\n   New documentation (first 5 lines):")
    for line in rustdoc.split('\n')[:5]:
        print(f"   {line}")
    if rustdoc.count('\n') > 5:
        lines_count = rustdoc.count(chr(10)) - 5
        print(f"   ... ({lines_count} more lines)")

    # Apply changes if not dry-run
    if not dry_run:
        modified_content = replace_rustdoc(
            rust_path, doc_start, doc_end, rustdoc
        )
        with open(rust_path, 'w', encoding="utf-8") as f:
            f.write(modified_content)
        print(f"   ✅ Updated {rust_path}")
    else:
        print("   (dry-run: no changes applied)")

    return True


def main():
    """Main entry point."""
    dry_run = "--update" not in sys.argv

    if dry_run:
        print("🔍 DRY RUN MODE - No files will be modified")
        print(
            "   Use: python scripts/sync_docs.py --update to apply changes"
        )
    else:
        print("⚠️  UPDATING FILES - Changes will be written to Rust source")

    repo_root = Path(__file__).parent.parent

    if not FUNCTIONS_TO_SYNC:
        print("⚠️  No functions configured to sync!")
        print("   Add entries to FUNCTIONS_TO_SYNC")
        return 1

    print(f"\n📦 Found {len(FUNCTIONS_TO_SYNC)} function(s) to sync")

    results = []
    for mapping in FUNCTIONS_TO_SYNC:
        success = sync_function(mapping, repo_root, dry_run=dry_run)
        results.append((mapping.func_name, success))

    # Summary
    print("\n" + "=" * 60)
    successful = sum(1 for _, success in results if success)
    total = len(results)
    print(f"Summary: {successful}/{total} functions synced successfully")

    for func_name, success in results:
        status = "✅" if success else "❌"
        print(f"  {status} {func_name}")

    if dry_run:
        print("\n💡 To apply these changes, run:")
        print("   python scripts/sync_docs.py --update")

    return 0 if successful == len(results) else 1


if __name__ == '__main__':
    sys.exit(main())
