# Documentation Sync Strategy

## Overview

We maintain documentation in **`.pyi` stub files as the single source of truth**. The `sync_docs.py` script automatically propagates these docs to Rust source files, eliminating duplication.

### File Organization

```
icedpygui/__init__.pyi       → Widget functions (add_button, add_text, etc.)
icedpygui/icedpygui.pyi      → Parameters, styles, helpers
src/py_api/*.rs              ← Auto-synced from .pyi files
```

## Workflow

### 1. Update the .pyi file (Source of Truth)

Edit `icedpygui/__init__.pyi` or `icedpygui/icedpygui.pyi` with improved documentation:

```python
def add_button(
    *,
    label: str = ...,
    on_press: Callable | None = None,
    # ... parameters ...
) -> int:
    """Adds a button widget.

    A clickable button used for GUI actions. The button must go into a
    container type (Container, Column, Row, etc.).

    Parameters
    ----------
    label : str, optional
        Sets the text label displayed on the button.
    on_press : callable, optional
        Sets the callback method to invoke when the button is pressed.
    # ... more parameters ...
    
    Returns
    -------
    int
        The numeric widget ID of the newly created button.
    """
```

### 2. Add to sync_docs.py

Update `scripts/sync_docs.py` - add the function mapping to `FUNCTIONS_TO_SYNC`:

```python
FUNCTIONS_TO_SYNC = [
    FunctionMapping(
        pyi_file="icedpygui/__init__.pyi",
        func_name="add_button",        # Name in .pyi file
        rust_file="src/py_api/button.rs",
        rust_func_name="add_button",   # Name in Rust (usually same)
    ),
    # NEW ENTRY: uncomment and update as you add more functions
    # FunctionMapping(
    #     pyi_file="icedpygui/__init__.pyi",
    #     func_name="add_text",
    #     rust_file="src/py_api/text.rs",
    #     rust_func_name="add_text",
    # ),
]
```

### 3. Preview Changes (Dry-Run)

```bash
python scripts/sync_docs.py
# or explicitly:
python scripts/sync_docs.py --dry-run
```

Output shows:
- Which function is being synced
- Where it's coming from and going to
- Preview of the new documentation
- **No files are modified**

### 4. Apply Changes

Once you're satisfied with the preview:

```bash
python scripts/sync_docs.py --update
```

This updates the Rust source files with the documentation from .pyi files.

### 5. Verify Changes

Run tests and build:

```bash
cargo build
maturin develop
```

## Adding More Functions

### Current Status

✅ **Synced:**
- `add_button()` (icedpygui/__init__.pyi → src/py_api/button.rs)

### To Add More Functions

1. Update the .pyi file with complete documentation
2. Uncomment/add a new `FunctionMapping` entry in `sync_docs.py`
3. Run dry-run to preview
4. Run `--update` to apply
5. Test and commit

### Example: Adding add_text()

```python
# 1. Make sure icedpygui/__init__.pyi::add_text has good docs
# 2. In sync_docs.py, uncomment the template:

FunctionMapping(
    pyi_file="icedpygui/__init__.pyi",
    func_name="add_text",
    rust_file="src/py_api/text.rs",
    rust_func_name="add_text",
),

# 3. Run:
python scripts/sync_docs.py --dry-run
python scripts/sync_docs.py --update

# 4. Verify:
cargo build
```

## Why This Approach?

✅ **Single source of truth** - Update once, everywhere stays in sync  
✅ **Python-first** - .pyi files are more readable for Python developers  
✅ **Automated** - No manual copying between files  
✅ **Incremental** - Add functions one at a time as docs improve  
✅ **Low risk** - Dry-run preview before applying changes  
✅ **Version control** - Track doc changes in git  

## Migration from generate_enum_stubs.py

The old `generate_enum_stubs.py` was designed to go from Rust → Python. 

**Our new approach reverses this:** Python (.pyi) → Rust

Once all commonly-used functions have been:
1. Added to `FUNCTIONS_TO_SYNC`
2. Synced and verified
3. All .pyi docs are current

...then `generate_enum_stubs.py` can be safely removed.

**Timeline:**
- Phase 1: Sync `add_button()`, verify it works ✅ (current)
- Phase 2: Add more widget functions (add_text, add_container, etc.)
- Phase 3: Add style functions (add_button_style, etc.)
- Phase 4: Remove `generate_enum_stubs.py`

## Tips

- **Keep .pyi docs concise** - Use Parameters/Returns sections consistently
- **Run dry-run first** - Always preview before applying
- **Commit frequently** - Sync one function at a time
- **Test after sync** - Run `cargo build` and examples to verify nothing broke
