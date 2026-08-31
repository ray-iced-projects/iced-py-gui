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
    ContainerStyleStd,
    Grid,
    Row,
    Scrollable,
    start_session,
    add_button,
    add_button_style,
    add_checkbox,
    add_pick_list,
    PickListParam,
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
    get_widget_palette_part,
    get_widget_palette_list,
    get_color_palette,
)


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
state = {
    "widget_list": [],
    "widget_active_id": None,
    "widget_hovered_id": None,
    "widget_pressed_id": None,
    "widget_disabled_id": None,
    "widget_parts": {},
    "parts_file": None,
    "parts_file_name": None,
    "current_color": None,
    "palette": {},
    "palette_name": "",
    "color_preview": "",
    "palette_ids": {},  # palette_name -> palette_id
    "palette_row_ids": [],  # list of row IDs for palette display
}




def on_palette_name_input(_ti_id: int, text: str):
    """Handle palette name input."""
    state["palette_name"] = text.strip()


def on_create_palette(_btn_id: int):
    """Create a palette with the current color and save it."""
    if not state["current_color"]:
        state["status_log"] += "ERROR: No color selected\n"
        print("Error: Select a color first")
        return

    if not state["palette_name"]:
        state["status_log"] += "ERROR: No palette name entered\n"
        print("Error: Enter a palette name")
        return

    try:
        palette_id = pm.save_palette(state["palette_name"], state["current_color"])
        if palette_id is not None:
            state["palette_ids"][state["palette_name"]] = palette_id
            print(f"Palette created: {state['palette_name']} (ID: {palette_id})")
        else:
            state["status_log"] += "ERROR: Failed to create palette\n"
    except Exception as e:
        state["status_log"] += f"ERROR: {str(e)}\n"
        print(f"Error: {e}")




# Set the file types
file_types = ["yml", "yaml"]

# Default directory
cwd = os.getcwd()
FILE_PATH = os.path.join(cwd, "python_examples", "py_palette")

def place_widgets(widget: str):
    """Add the selected widget with status"""
    match widget:
        case "button":
            state["widget_active_id"] = add_button(
                label="Status=Active",
                parent_id=new_widget_row_id,
                active=True,
                padding=[10],
                )
            state["widget_hovered_id"] = add_button(
                label="Status=Hovered",
                parent_id=new_widget_row_id,
                hovered=True,
                padding=[10],
                )
            state["widget_pressed_id"] = add_button(
                label="Status=Pressed",
                parent_id=new_widget_row_id,
                pressed=True,
                padding=[10],
                )
            state["widget_disabled_id"] = add_button(
                label="Status=Disabled",
                parent_id=new_widget_row_id,
                disabled=True,
                padding=[10],
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
    place_widgets(selection)
    for part in state["widget_parts"].get("parts"):
        add_text(parent_id=parts_row_id, content=part)


# Setup the FSD
fsd_id = add_file_system_dialog(
    results_callback=on_parts_loaded,
    filters=file_types,
    default_directory=FILE_PATH,
)

def on_color_picked(_cp_id: int, color: list[float]):
    """Handle color selection from ColorPicker."""
    state["current_color"] = color
    formatted = [round(c, 2) for c in color]
    update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {formatted}")
    state["palette"] = get_color_palette(rgba=color)
    statuses = state["widget_parts"].get("statuses")
    for (idx, status) in enumerate(statuses):
        pal_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in state["palette"].items()}
        add_text(parent_id=state["palette_row_ids"][idx], content=status)
        for (k, pal_color) in pal_by_name.items():
            if "Text" not in k:
                text_color = pal_by_name.get(k + "Text")
                btn_style_id = add_button_style(bkg_rgba=pal_color, text_rgba=text_color)
                add_button(
                    parent_id=state["palette_row_ids"][idx],
                    label=k,
                    width=100,
                    padding=[10],
                    style_id=btn_style_id)
                # add_checkbox(parent_id=state["palette_row_ids"][idx],
                #              label="Select")


def on_color_text_input(_ti_id: int, text: str):
    """Handle color input from TextInput (format: [r, g, b, a])."""
    color = pm.parse_color_from_text_input(text)
    if color:
        state["current_color"] = color
        update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {color}")
    else:
        update_widget(selected_color_txt_id, TextParam.Content,
                      "Invalid color format, use float values")


# ============================================================================
# GUI
# ============================================================================

with Window(title="Palette Creator - Interactive Workflow", center=True, size=(1000, 700)):
    with Scrollable(height=650):
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

                add_text(content=("***Step 3: Select Color for new palette using\n"
                                  "either ColorPicker(press submit) or \n"
                                  "text_input(press enter to submit)."))

                with Row(spacing=10, width_fill=True):
                    with ColorPicker(on_submit=on_color_picked):
                        add_button(label="Select a Palette Color")
                    add_text_input(
                        placeholder="Or type/paste: [r, g, b, a] float format",
                        on_submit=on_color_text_input,
                        width=300
                    )

                selected_color_txt_id = add_text(content="Selected color = []")

                widget_selected_txt_id = add_text(content="***Selected Widget")

                with Row(spacing=20) as new_widget_row_id:
                # The selected widget should be placed here
                    pass

                add_text(content="Status=")
                with Grid(width=600, columns_amount=6, spacing=10.0):
                            for row in range(8):
                                for col in range(6):
                                    with Container(height=40, align_center=True,
                                                   style_std=ContainerStyleStd.BorderedBox):
                                        add_text(content=f"Grid {row} {col}")

                # Populate state with column IDs for dynamic access
                # state["palette_row_ids"] = [
                #     row_palette_id_0, row_palette_id_1, row_palette_id_2,
                #     row_palette_id_3, row_palette_id_4, row_palette_id_5
                # ]

start_session()
