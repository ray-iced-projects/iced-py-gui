#!/usr/bin/env python3
"""
Palette Creator: Interactive tool to create and manage widget color palettes.

Workflow:
1. Load or create a palette file
2. Define base color using ColorPicker or text input
3. Create palette with status variants (Active, Hovered, Pressed, Disabled)
4. Get palette_id to use with widgets
"""

import json
from pathlib import Path
import os
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    Row,
    Scrollable,
    start_session,
    add_button,
    add_button_style,
    ButtonStyleParam,
    add_pick_list,
    PickListParam,
    add_checkbox,
    CheckboxParam,
    add_text,
    add_text_input,
    TextParam,
    custom_palette,
    PaletteKey,
    WidgetStatus,
    StateVariant,
    StylePart,
    add_file_system_dialog,
    FileSystemDialogCallbackType as FsdCallType,
    FileSystemDialogParam as FsdParam,
    update_widget,
    update_widget_params,
    get_widget_palette_part,
    get_widget_palette_list,
    get_color_palette,
)
from python_examples.py_palette.widget_matching import WidgetMatcher


# ============================================================================
# Palette storage and file I/O
# ============================================================================

PALETTES_DIR = Path.home() / ".icedpygui_palettes"
PALETTES_DIR.mkdir(exist_ok=True)

class PaletteManager:
    """Manages palette creation, storage, and retrieval."""

    def __init__(self):
        self.current_color = None
        self.current_palette_id = None
        self.palette_list = {}  # name -> palette_id
        self.load_palette_list()

    def load_palette_list(self):
        """Load palette metadata from storage."""
        list_file = PALETTES_DIR / "palette_list.json"
        if list_file.exists():
            try:
                with open(list_file, "r", encoding="utf-8") as f:
                    self.palette_list = json.load(f)
            except (json.JSONDecodeError, IOError):
                self.palette_list = {}

    def save_palette_list(self):
        """Save palette metadata to storage."""
        list_file = PALETTES_DIR / "palette_list.json"
        try:
            with open(list_file, "w", encoding="utf-8") as f:
                json.dump(self.palette_list, f, indent=2)
        except IOError as e:
            print(f"Error saving palette list: {e}")

    def save_palette(self, name: str, color: list[float], statuses: list = None):
        """Save a palette to file and create palette_id."""
        try:
            palette_file = PALETTES_DIR / f"{name}.json"
            palette_data = {
                "color": color,
                "statuses": statuses or self._default_statuses(),
            }
            with open(palette_file, "w", encoding="utf-8") as f:
                json.dump(palette_data, f, indent=2)

            # Create the palette in the application
            palette_id = custom_palette(rgba=color, statuses=statuses or self._default_statuses())
            self.palette_list[name] = {
                "id": palette_id,
                "file": str(palette_file),
            }
            self.save_palette_list()
            return palette_id
        except IOError as e:
            print(f"Error saving palette: {e}")
            return None

    def load_palette(self, name: str):
        """Load a palette from file."""
        palette_file = PALETTES_DIR / f"{name}.json"
        if not palette_file.exists():
            return None

        try:
            with open(palette_file, "r", encoding="utf-8") as f:
                data = json.load(f)
            self.current_color = data.get("color")
            statuses = data.get("statuses", self._default_statuses())
            palette_id = custom_palette(rgba=self.current_color, statuses=statuses)
            self.current_palette_id = palette_id
            return palette_id
        except (json.JSONDecodeError, IOError) as e:
            print(f"Error loading palette: {e}")
            return None

    @staticmethod
    def _default_statuses():
        """Create default status mappings for all button states."""
        return [
            # Active state
            ((WidgetStatus.Active, StateVariant.NoVariant), (
                (StylePart.Background, PaletteKey.Base, 1.0),
                (StylePart.Text, PaletteKey.BaseText, 1.0),
                (StylePart.Border, PaletteKey.Base, 1.0),
            )),
            # Hovered state
            ((WidgetStatus.Hovered, StateVariant.NoVariant), (
                (StylePart.Background, PaletteKey.Strong, 1.0),
                (StylePart.Text, PaletteKey.StrongText, 1.0),
                (StylePart.Border, PaletteKey.Strong, 1.0),
            )),
            # Pressed state
            ((WidgetStatus.Pressed, StateVariant.NoVariant), (
                (StylePart.Background, PaletteKey.Weakest, 1.0),
                (StylePart.Text, PaletteKey.WeakestText, 1.0),
                (StylePart.Border, PaletteKey.Base, 1.0),
            )),
            # Disabled state
            ((WidgetStatus.Disabled, StateVariant.NoVariant), (
                (StylePart.Background, PaletteKey.Base, 0.5),
                (StylePart.Text, PaletteKey.BaseText, 0.6),
                (StylePart.Border, PaletteKey.Base, 0.3),
            )),
        ]

    def parse_color_from_text_input(self, text: str) -> list[float] | None:
        """Parse color from text input like '[0.5, 0.3, 0.2, 1.0]'."""
        try:
            text = text.strip()
            if text.startswith("[") and text.endswith("]"):
                text = text[1:-1]
            values = [float(x.strip()) for x in text.split(",")]
            if len(values) != 4:
                return None
            return values
        except ValueError:
            return None



# ============================================================================
# Application State and UI Callbacks
# ============================================================================

pm = PaletteManager()
matcher = WidgetMatcher()

state = {
    "widget_list": [],
    "widget_active_id": None,
    "widget_active_style_id": None,
    "widget_hovered_id": None,
    "widget_hovered_style_id": None,
    "widget_pressed_id": None,
    "widget_pressed_style_id": None,
    "widget_disabled_id": None,
    "widget_disabled_style_id": None,
    "widget_name": "",
    "widget_parts": {},
    "widget_ids": [],
    "parts_file": "",
    "parts_file_name": None,
    "current_color": None,
    "palette": {},
    "palette_name": "",
    "color_preview": "",
    "palette_ids": {},  # palette_name -> palette_id
    "palette_row_ids": [],  # list of row IDs for palette display
    "empty_ids": [], # The empty text ids
    "row_ids": [], # rows containing the widget and checkboxes
    "checkbox_grid": [],  # 2D grid: [row][col] for matrix selection
}


def on_palette_name_input(_ti_id: int, text: str):
    """Handle palette name input."""
    matcher.palette_name = text.strip()


# def on_create_palette(_btn_id: int):
#     """Create a palette with the current color and save it."""
#     if not state["current_color"]:
#         state["status_log"] += "ERROR: No color selected\n"
#         print("Error: Select a color first")
#         return

#     if not state["palette_name"]:
#         state["status_log"] += "ERROR: No palette name entered\n"
#         print("Error: Enter a palette name")
#         return

#     try:
#         palette_id = pm.save_palette(state["palette_name"], state["current_color"])
#         if palette_id is not None:
#             state["palette_ids"][state["palette_name"]] = palette_id
#             print(f"Palette created: {state['palette_name']} (ID: {palette_id})")
#         else:
#             state["status_log"] += "ERROR: Failed to create palette\n"
#     except Exception as e:
#         state["status_log"] += f"ERROR: {str(e)}\n"
#         print(f"Error: {e}")




# Set the file types
file_types = ["yml", "yaml"]

# Default directory
cwd = os.getcwd()
DEFAULT_DIRECTORY = os.path.join(cwd, "python_examples", "py_palette")
BUTTON_WIDTH = 150


def place_widgets(widget: str):
    """Add the selected widget with status"""
    match widget:
        case "button":
            state["widget_active_style_id"] = add_button_style()
            state["widget_active_id"] = add_button(
                label="Status=Active",
                parent_id=new_widget_row_id,
                active=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=state["widget_active_style_id"]
                )

            state["widget_hovered_style_id"] = add_button_style()
            state["widget_hovered_id"] = add_button(
                label="Status=Hovered",
                parent_id=new_widget_row_id,
                hovered=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=state["widget_hovered_style_id"]
                )

            state["widget_pressed_style_id"] = add_button_style()
            state["widget_pressed_id"] = add_button(
                label="Status=Pressed",
                parent_id=new_widget_row_id,
                pressed=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=state["widget_pressed_style_id"]
                )

            state["widget_disabled_style_id"] = add_button_style()
            state["widget_disabled_id"] = add_button(
                label="Status=Disabled",
                parent_id=new_widget_row_id,
                disabled=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=state["widget_disabled_style_id"]
                )


def load_parts_file(_btn_id: int):
    """Opens the FSDr"""
    update_widget(fsd_id, FsdParam.LoadFile, True)


def on_parts_loaded(_fsd_id: int, results: tuple[FsdCallType, str, any]):
    """Callback results for the FSD"""
    # store parts file
    (_fsd_type, file_name, data) = results
    state["parts_file"] = data
    state['parts_file_name'] = file_name
    state["widget_list"] = get_widget_palette_list(data)

    update_widget(widget_pl_id, PickListParam.Options, state["widget_list"])
    update_widget(widget_pl_id, PickListParam.Placeholder, "Select a widget")
    update_widget(parts_file_status_txt_id, TextParam.Content,
                      f"Parts file selected = {file_name}")


def parse_parts_selection(_pl_id: int, selection: str):
    """Parsing the parts file for the selected widget"""
    state["widget_parts"] = get_widget_palette_part(selection, state["parts_file"])
    state["widget_name"] = selection
    place_widgets(selection)



# Setup the FSD
fsd_id = add_file_system_dialog(
    results_callback=on_parts_loaded,
    filters=file_types,
    default_directory=DEFAULT_DIRECTORY,
)

def on_color_picked(_cp_id: int, color: list[float]):
    """Handle color selection from ColorPicker."""
    state["current_color"] = color
    formatted = [round(c, 2) for c in color]
    update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {formatted}")
    populate_widgets_checkboxes(color)


def on_color_text_input(_ti_id: int, text: str):
    """Handle color input from TextInput (format: [r, g, b, a])."""
    color = pm.parse_color_from_text_input(text)
    if color:
        state["current_color"] = color
        update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {color}")
        populate_widgets_checkboxes(color)
    else:
        update_widget(selected_color_txt_id, TextParam.Content,
                      "Invalid color format, use float values")


def on_status_checked(cb_id: int, is_checked: bool):
    """
    Enforce matrix selection: only one checked per row AND per column.
    On check, clear the rest of the target's row and column, then set it True.
    """
    grid = state["checkbox_grid"]  # list[list[tuple[int, bool]]]

    # Locate the target id in the grid
    target_row = target_col = None
    for r, row in enumerate(grid):
        for c, (wid, _state) in enumerate(row):
            if wid == cb_id:
                target_row, target_col = r, c
                break
        if target_row is not None:
            break

    if target_row is None:
        return  # id not found

    # Clear every True in the target's row (except the target)
    for c, (wid, checked) in enumerate(grid[target_row]):
        if c != target_col and checked:
            update_widget(wid, CheckboxParam.IsChecked, False)
            grid[target_row][c] = (wid, False)

    # Clear every True in the target's column (except the target)
    for r, row in enumerate(grid):
        if r != target_row:
            wid, checked = row[target_col]
            if checked:
                update_widget(wid, CheckboxParam.IsChecked, False)
                row[target_col] = (wid, False)

    # Set the target to its incoming state
    update_widget(cb_id, CheckboxParam.IsChecked, is_checked)
    grid[target_row][target_col] = (cb_id, is_checked)
    set_widget_status(target_row, target_col)


def populate_widgets_checkboxes(color: list):
    """Updating the widget and checkboxes for palette matrix selection"""
    state["palette"] = get_color_palette(rgba=color)
    statuses = state["widget_parts"].get("statuses")
    pal_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in state["palette"].items()}

    # Initialize the checkbox grid (8 rows x 4 columns)
    state["checkbox_grid"] = [[] for _ in range(8)]

    # add the palette color buttons (one per status/column)
    idx = 0
    for (k, pal_color) in pal_by_name.items():
        if "Text" not in k:
            text_color = pal_by_name.get(k + "Text")
            btn_style_id = add_button_style(bkg_rgba=pal_color, text_rgba=text_color)
            widget_id = add_button(
                            parent_id=state["row_ids"][idx],
                            label=k,
                            width=100,
                            padding=[10],
                            style_id=btn_style_id)
            state["widget_ids"].append(widget_id)
            idx += 1

    # Create checkboxes in a 8x4 matrix (8 rows x 4 statuses/columns)
    for c, (idx, status) in enumerate(enumerate(statuses)):
        for r in range(8):
            cb_id = add_checkbox(
                        parent_id=state["row_ids"][r],
                        label=status,
                        on_toggle=on_status_checked,
                        )
            state["checkbox_grid"][r].append((cb_id, False))


# the widget and palette is found by row=status_index, col=pal_idx of the matrix
def set_widget_status(pal_idx: int, status_index: int):
    """Radio select for status"""
    statuses = state["widget_parts"].get("statuses")
    _pal, bkg_rgba = list(state["palette"].items())[pal_idx*2]
    _text_pal, bkg_text_color = list(state["palette"].items())[pal_idx*2 + 1]
    status = statuses[status_index]
    print(status)
    # update the widget
    match state["widget_name"]:
        case "button":
            match status:
                case "Active":
                    update_widget_params(
                        state["widget_active_style_id"], {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Hovered":
                    update_widget_params(
                        state["widget_hovered_style_id"], {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Pressed":
                    update_widget_params(
                        state["widget_pressed_style_id"], {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Disabled":
                    update_widget_params(
                        state["widget_disabled_style_id"], {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })


def load_demo(_btn_id: int):
    """Method to load the demo"""
    file_path = os.path.join(cwd, "python_examples", "py_palette", "widget_palette_parts.yml")
    try:
        with open(file_path, "r", encoding='utf-8') as file:
            state["parts_file"] = file.read()
    except FileNotFoundError:
        print(f"*********The file does not exist using {file_path}.*******")

    # Parts
    state['parts_file_name'] = file_path
    state["widget_list"] = get_widget_palette_list(state["parts_file"])
    state["widget_parts"] = get_widget_palette_part("button", state["parts_file"])
    state["widget_name"] = "button"
    place_widgets("button")
    # color
    color = [0.32, 0.2, 0.13, 1.0]
    state["current_color"] = color
    update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {color}")
    populate_widgets_checkboxes(color)

# ============================================================================
# GUI
# ============================================================================

with Window(title="Palette Creator - Interactive Workflow", center=True, size=(1000, 800)):

    with Container(padding=[20]):
        add_button(label="Load Demo", padding=[10], on_press=load_demo)

    with Container(width_fill=True, padding=[20]):
        with Column(spacing=15) as col:
            # Instructions
            add_text(content=
                "Palette Creation Workflow\n")

            add_text(content="***Step 1: Load Palettes Part file.")
            with Row(spacing=20):
                add_button(label="Select Palette Parts file", on_press=load_parts_file)
                parts_file_status_txt_id = add_text(content="Parts file selected = None")


            add_text(content="***Step 2: Select the widget to create the palette for")
            widget_pl_id = add_pick_list(options=state["widget_list"],
                                    placeholder="Empty until parts file selected",
                                    on_select=parse_parts_selection)

            add_text(content=("***Step 3: Select Color for new palette using "
                                "either ColorPicker(press submit) or "
                                "text_input(press enter to submit)."),
                     width=600, wrapping_word_glyph=True)

            with Row(spacing=10, width_fill=True, height=30):
                with ColorPicker(on_submit=on_color_picked):
                    add_button(label="Select a Palette Color")
                add_text_input(
                    placeholder="Or type/paste: [r, g, b, a] float format",
                    on_submit=on_color_text_input,
                    width=300
                )

            selected_color_txt_id = add_text(content="Selected color = []")

            widget_selected_txt_id = add_text(content="***Selected Widget")

            with Row(spacing=10) as new_widget_row_id:
            # The selected widget should be placed here
                pass

            with Scrollable(width=600, height=275):
                with Column(spacing=10):
                    for row in range(8):
                        with Row(spacing=10) as row_id:
                            # initialize some ids
                            state["row_ids"].append(row_id)


start_session()
