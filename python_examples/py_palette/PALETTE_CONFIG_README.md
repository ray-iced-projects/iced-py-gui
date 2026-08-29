# Palette Configuration System

A complete system for creating and managing custom color palettes for iced-py-gui widgets.

## Quick Start

### 1. Define Your Palette in YAML

Create a `.yml` config file (e.g., `my_button_palette.yml`):

```yaml
widget: button

metadata:
  description: "Custom button palette"
  palette_enabled: true

statuses:
  - Active
  - Hovered
  - Pressed
  - Disabled

state_variants:
  - NoVariant

parts:
  - Background
  - Text
  - Border

palette_mappings:
  - status: Active
    variant: NoVariant
    parts:
      - part: Background
        key: Base
        alpha: 1.0
      - part: Text
        key: BaseText
        alpha: 1.0
      - part: Border
        key: Stronger
        alpha: 1.0

  - status: Hovered
    variant: NoVariant
    parts:
      - part: Background
        key: Strong
        alpha: 1.0
      - part: Text
        key: StrongText
        alpha: 1.0
      - part: Border
        key: Strongest
        alpha: 1.0

  - status: Pressed
    variant: NoVariant
    parts:
      - part: Background
        key: Base
        alpha: 1.0
      - part: Text
        key: BaseText
        alpha: 1.0
      - part: Border
        key: Stronger
        alpha: 1.0

  - status: Disabled
    variant: NoVariant
    parts:
      - part: Background
        key: Base
        alpha: 0.5
      - part: Text
        key: BaseText
        alpha: 0.5
      - part: Border
        key: Stronger
        alpha: 0.5
```

### 2. Load the Config in Python

```python
from icedpygui import Window, add_button, custom_palette, start_session
from palette_config_loader import PaletteConfigLoader

# Load the palette definition from YAML
statuses = PaletteConfigLoader.load("my_button_palette.yml")

# Create a palette with your base color
palette_id = custom_palette(
    rgba=[0.32, 0.2, 0.13, 1.0],  # Your base color
    statuses=statuses
)

# Use it with widgets
with Window(title="My App"):
    add_button(label="Styled Button", palette_id=palette_id)

start_session()
```

### 3. Using the Interactive Palette Creator

```bash
python python_examples/py_palette/py_palette_create.py
```

Features:
- **ColorPicker**: Visually select colors
- **Text Input**: Paste color as `[r, g, b, a]`
- **Preview**: See all palette tiers before saving
- **Auto-Save**: Palettes stored in `~/.icedpygui_palettes/`

## Palette Key Tiers

Available palette keys for styling:

| Tier | Color Keys | Text Keys |
|------|-----------|-----------|
| Weakest | `Weakest` | `WeakestText` |
| Weak | `Weak` | `WeakText` |
| Base | `Base` | `BaseText` |
| Strong | `Strong` | `StrongText` |
| Stronger | `Stronger` | `StrongerText` |
| Strongest | `Strongest` | `StrongestText` |

Each tier is automatically generated from the base color using WCAG-compliant contrast calculations.

## Widget Status Types

| Status | Usage |
|--------|-------|
| `Active` | Default button state |
| `Hovered` | Mouse hovering over button |
| `Pressed` | Button actively clicked |
| `Disabled` | Button is inactive |

## Style Parts

| Part | Purpose |
|------|---------|
| `Background` | Button background color |
| `Text` | Button text color |
| `Border` | Button border color |

## State Variants

| Variant | Usage |
|---------|-------|
| `NoVariant` | Default state |
| `Checked` | Checkbox/toggle is checked |
| `Unchecked` | Checkbox/toggle is unchecked |

## Examples

### Example 1: Brown Button Palette

```yaml
widget: button

palette_mappings:
  - status: Active
    variant: NoVariant
    parts:
      - part: Background
        key: Base
        alpha: 1.0
      - part: Text
        key: BaseText
        alpha: 1.0
      - part: Border
        key: Base
        alpha: 1.0

  - status: Hovered
    variant: NoVariant
    parts:
      - part: Background
        key: Strong
        alpha: 1.0
      - part: Text
        key: StrongText
        alpha: 1.0
      - part: Border
        key: Strong
        alpha: 1.0
```

Base color: `[0.32, 0.2, 0.13, 1.0]` (brown)

### Example 2: Subtle Palette with Transparency

```yaml
palette_mappings:
  - status: Active
    variant: NoVariant
    parts:
      - part: Background
        key: Base
        alpha: 0.8
      - part: Text
        key: BaseText
        alpha: 0.9
      - part: Border
        key: Base
        alpha: 0.6
```

Use `alpha: 0.0 - 1.0` to create transparent effects.

## File Structure

```
~/.icedpygui_palettes/
├── palette_list.json          # Index of all saved palettes
├── MyBrown.json              # Individual palette (RGBA + statuses)
└── DarkTheme.json

python_examples/py_palette/
├── button_palette_config.yml    # Example config (copy & customize)
├── widget_palette_parts.yml     # Widget capabilities reference
├── palette_config_loader.py     # YAML parser & loader
└── py_palette_create.py         # Interactive palette creator
```

## API Reference

### PaletteConfigLoader

Load palette definitions from YAML config files and manage widget parts:

#### 1. Load from file path

```python
from palette_config_loader import PaletteConfigLoader
from icedpygui import custom_palette

# Absolute path
statuses = PaletteConfigLoader.load("/path/to/button_palette_config.yml")

# Relative path with base directory
from pathlib import Path
base = Path(__file__).parent / "py_palette"
statuses = PaletteConfigLoader.load("button_palette_config.yml", base_dir=base)

# Create palette
pal_id = custom_palette(rgba=[0.5, 0.3, 0.2, 1.0], statuses=statuses)
```

#### 2. Load from file dialog

Opens a file selection dialog for the user to choose a palette config:

```python
from palette_config_loader import PaletteConfigLoader
from icedpygui import custom_palette, Window, start_session, add_button

def on_load_palette_btn(btn_id: int):
    """Button callback to load palette from dialog."""
    statuses = PaletteConfigLoader.load_from_file_dialog()
    if statuses:
        pal_id = custom_palette(rgba=[0.5, 0.3, 0.2, 1.0], statuses=statuses)
        # Use palette_id with your widgets
        print(f"Palette created with ID: {pal_id}")

with Window(title="My App"):
    add_button(label="Load Palette", on_press=on_load_palette_btn)

start_session()
```

**Parameters:**
- `file_types` (optional): List of file extensions to filter (default: `['yml', 'yaml']`)

**Returns:** 
- Parsed palette definitions (list), or None if cancelled/error

#### 3. Load widget parts

Get the available statuses and parts for a specific widget:

```python
from palette_config_loader import PaletteConfigLoader

# Load parts for button widget
button_config = PaletteConfigLoader.load_widget_parts('button')

if button_config:
    print(f"Statuses: {button_config['statuses']}")
    # Output: ['Active', 'Pressed', 'Hovered', 'Disabled']
    
    print(f"Parts: {button_config['parts']}")
    # Output: ['Background', 'Text', 'Border']
    
    print(f"Palette Enabled: {button_config['palette_enabled']}")
    # Output: True
```

**Parameters:**
- `widget_name` (str): Name of the widget (case-insensitive, e.g., 'button', 'checkbox')
- `parts_file` (optional): Path to widget_palette_parts.yml file. Auto-discovered if not provided.

**Returns:**
- Dictionary with widget config (statuses, parts, variants, palette_enabled), or None if not found

**Supported widgets:** button, checkbox, radio, container, text_input, slider, progress_bar, toggler, pick_list, combobox, card, menu, sash, scrollable, table, text_editor, tool_tip

#### 4. Print config info

Print human-readable configuration details:

```python
from palette_config_loader import PaletteConfigLoader

# Print info for absolute path
PaletteConfigLoader.print_config_info("/path/to/button_palette_config.yml")

# Print info with base directory
from pathlib import Path
base = Path(__file__).parent / "py_palette"
PaletteConfigLoader.print_config_info("button_palette_config.yml", base_dir=base)
```

**Output example:**
```
======================================================================
Palette Config: button_palette_config.yml
======================================================================

Widget: button
Palette Enabled: true

Statuses:
  - Active
  - Hovered
  - Pressed
  - Disabled

State Variants:
  - NoVariant

Style Parts:
  - Background
  - Text
  - Border

Palette Mappings:

  Active (NoVariant):
    - Background  → Base            (alpha: 1.0) Base color for button background
    - Text        → BaseText        (alpha: 0.8) Readable text on base background
    - Border      → Stronger        (alpha: 1.0) Subtle border using stronger palette tier
    ...
======================================================================
```

### PaletteManager

Manage palette creation and storage (used by py_palette_create.py):

```python
from py_palette_create import PaletteManager

pm = PaletteManager()

# Save a palette
pal_id = pm.save_palette(
    name="MyBrown",
    color=[0.5, 0.3, 0.2, 1.0],
    statuses=[...]  # optional, uses defaults if not provided
)

# Load a saved palette
pal_id = pm.load_palette("MyBrown")

# Parse color from text
color = pm.parse_color_input("[0.5, 0.3, 0.2, 1.0]")

# Load statuses from config file
statuses = pm.load_statuses_from_config("button_palette_config.yml")
```

## Requirements

- **PyYAML** (optional, for config file loading):
  ```bash
  pip install PyYAML
  ```
  
  Without it, use default statuses or define inline Python lists.

## Tips & Tricks

1. **Start with button_palette_config.yml**: Copy and modify it for other widgets
2. **Use alpha for depth**: Different alpha values create layered effects
3. **Test with ColorPicker**: See results before saving to disk
4. **Save multiple variants**: Create Active, Hovered, Pressed configs separately
5. **Document your palettes**: Add `description` field to YAML for future reference

## Troubleshooting

**"Config file not found"**
- Check file path is relative to script directory
- Use absolute paths: `Path(__file__).parent / "config.yml"`

**"YAML support not available"**
- Install PyYAML: `pip install PyYAML`
- Or use Python list definitions instead

**"Unknown palette key"**
- Check key name matches available tiers (Base, Strong, Strongest, etc.)
- Use `print_config_info()` to debug

## See Also

- [py_palette_create.py](py_palette_create.py) - Interactive palette creator
- [button_palette_config.yml](button_palette_config.yml) - Example button config
- [palette_config_loader.py](palette_config_loader.py) - YAML loader implementation
- [py_button_new_palette.py](../py_button/py_button_new_palette.py) - Button example
