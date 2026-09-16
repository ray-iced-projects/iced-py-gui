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

from python_examples.py_palette.demo_helpers import demo_populate_palette_area
from python_examples.py_palette.widget_helpers import (
    WidgetConfig, load_demo_config, match_widget_str, place_widget, set_new_widget_palette)
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    ContainerStyleStd,
    Row,
    Scrollable,
    start_session,
    add_button,
    add_button_style,
    add_pick_list,
    PickListParam,
    PopUp,
    PopUpParam,
    add_text,
    add_text_input,
    TextParam,
    custom_palette,
    PaletteKey,
    WidgetStatus,
    StateVariant,
    StylePart,
    add_file_system_dialog,
    FileSystemDialogParam as FsdParam,
    FileSystemDialogCallbackType as FsdType,
    update_widget,
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


pm = PaletteManager()

class PaletteCreator:
    """Manages palette state and widget selection logic."""

    def __init__(self):
        self.widget_list: list[str] = []
        self.demo_widget_list: list[str] = []
        self.widget_active_id: int = None
        self.widget_active_style_id: int = None
        self.widget_hovered_id: int = None
        self.widget_hovered_style_id: int = None
        self.widget_pressed_id: int = None
        self.widget_pressed_style_id: int = None
        self.widget_disabled_id: int = None
        self.widget_disabled_style_id: int = None
        self.widget_name: str = None
        self.widget_parts: dict = {}
        self.widget_config: WidgetConfig = WidgetConfig()
        self.palette_widget_ids: list[int] = []
        self.palette_popup_btn_ids: list[int] = []
        self.palette_popup_btn_style_ids: list[int] = []
        self.new_widget_row_id: int = None
        self.parts_file: dict = {}
        self.parts_file_name: str = None
        self.unique_parts_list: list[str] = []
        self.unique_status_list: list[str] = []
        self.parts_list_ids: list[int] = []
        self.current_color: list[float, 4] = []
        self.palette: dict = {}
        self.palette_name: str = ""
        self.palette_ids: dict = {}  # palette_name -> palette_id
        self.palette_row_ids: list[int] = []  # list of row IDs for palette display
        self.row_ids: list[int] = [] # rows containing the widget and checkboxes
        self.checkbox_grid: list[list[int, 2]] = [] # 2D grid: [row][col] for matrix selection
        self.popup_ids: list[int] = []
        self.popup_btn_ids: list[int] = []


    def set_new_widget(self, widget_str: str):
        """Place the selected widget"""
        widget = match_widget_str(widget_str)
        if not widget is None:
            place_widget(self, widget)
        else:
            print(f"Failed to sett the new widget, unable to find {widget_str}")


pc = PaletteCreator()

# Default directory
cwd = os.getcwd()
DEFAULT_DIRECTORY = os.path.join(cwd, "python_examples", "py_palette")
BUTTON_WIDTH = 150


def load_parts_file(_btn_id: int):
    """Opens the FSDr"""
    update_widget(fsd_id, FsdParam.LoadFile, True)


def on_file_path_selected(_fsd_id: int, results: tuple[FsdType, str]):
    """Callback results for the FSD"""
    _fsd_type, file_path = results
    pc.widget_list = WidgetConfig.get_widget_names_from_file(file_path)
    update_widget(widget_pl_id, PickListParam.Options, pc.widget_list)



# file_types = get_dialog_filters
# The results of using get_dialog_filters gives you  a list of the various
# filters that the dialog widget has.  'All files' is always append to the end
# so that if the file is not available, you can simple select all file in the
# dialog to search for any file.  The 'All Files' is also the default.
# ['All Files', 'Archive Files', 'Audio Files', 'C Files', 'C++ Files', 'CSS Files',
# 'HTML Files', 'Image Files', 'JPEG Images', 'JSON Files', 'Java Files', 'JavaScript Files',
# 'PDF Files', 'PNG Images', 'Python Files', 'Rust Files', 'Text Files', 'TypeScript Files',
# 'Video Files', 'Word Documents', 'XML Files', 'YAML Files']

# Setup the FSD to select the file name
fsd_id = add_file_system_dialog(
    results_callback=on_file_path_selected,
    filters=['YAML Files'],
    default_directory=DEFAULT_DIRECTORY,
)

def open_fsd_for_file_path(_btn_id: int):
    """Opens the file dialog"""
    update_widget(fsd_id, FsdParam.SelectFile, True)


def parse_parts_selection(_pl_id: int, selection: str):
    """Parsing the parts file for the selected widget"""
    pc.widget_name = selection
    load_demo_config(pc)
    pc.set_new_widget(selection)


def on_color_picked(_cp_id: int, color: list[float]):
    """Handle color selection from ColorPicker."""
    pc.current_color = color
    formatted = [round(c, 2) for c in color]
    update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {formatted}")
    demo_populate_palette_area(pc)


def on_color_text_input(_ti_id: int, text: str):
    """Handle color input from TextInput (format: [r, g, b, a])."""
    color = pm.parse_color_from_text_input(text)
    if color:
        pc.current_color = color
        update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {color}")
    else:
        update_widget(selected_color_txt_id, TextParam.Content,
                      "Invalid color format, use float values")
    demo_populate_palette_area(pc)

# When the palette button is pressed this def is called
# Key point is to remember to add the user_data parameter
def on_palette_selected(_btn_id, user_data: tuple[str, str, list[float], int]):
    """update the new widget colors
    user_data:
        str=widget part, i.e. background,
        str=widget status, i.e., Active
        list[float] = rgba color
        int = popup_id
    """
    # Will need to match both widget and status later
    (part_, status_, rgba, popup_id_) = user_data
    update_widget(popup_id_, PopUpParam.Opened, False)
    set_new_widget_palette(pc, part_, status_, rgba)


def open_palette_popup(_btn_id: int, popup_id_: int):
    """Opens the popup to select the palette"""
    update_widget(popup_id_, PopUpParam.Opened, True)


def clicked_outside(pop_id: int):
    """Called when mouse clicked outside palette selection popup"""
    update_widget(pop_id, PopUpParam.Opened, False)


# populate the dropdown for the demo widgets
parts_file_path = os.path.join(cwd, "python_examples", "py_palette", "widget_palette_parts.yml")
pc.demo_widget_list = WidgetConfig.get_widget_names_from_file(parts_file_path)

def load_demo(_pl_id: int, selected: str):
    """Loading a demo setup"""
    pc.widget_name = selected
    load_demo_config(pc)
    update_widget(selected_color_txt_id, TextParam.Content,
                  f"Selected color = {pc.widget_config.selected_color}")
    pc.set_new_widget(selected)
    demo_populate_palette_area(pc)


# ============================================================================
# GUI
# ============================================================================

with Window(title="Palette Creator - Interactive Workflow", center=True, size=(1000, 800)):

    with Container(padding=[20]):
        add_pick_list(options=pc.demo_widget_list,
                      placeholder="Load the Demo Widget Palette file",
                      on_select=load_demo)

    with Container(width_fill=True, padding=[40]):
        with Scrollable(width_fill=True, height=750):
            with Column(spacing=15, width_fill=True) as col:
                # Instructions
                add_text(content=
                    "Palette Creation Workflow\n")

                add_text(content="***Step 1: Load Palettes Part file.")
                with Row(spacing=20):
                    add_button(label="Select Palette Parts file", on_press=open_fsd_for_file_path)
                    parts_file_status_txt_id = add_text(content="Parts file selected = None")


                add_text(content="***Step 2: Select the widget to create the palette for")
                widget_pl_id = add_pick_list(options=pc.widget_list,
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
                    pc.new_widget_row_id = new_widget_row_id
                # The selected widget should be placed here

                # This area is hidden until the color and widget is selected
                # area prepopulated with widget that only need to be updated later
                # one could take the approach to add these widgets as needed.
                with Row(spacing=20):
                    with Column(spacing=20):
                        for part in range(8):
                            pc.parts_list_ids.append([])
                            for status in range(8):
                                pc.parts_list_ids[-1].append(
                                    add_text(content="", show=False))
                    with Column(spacing=20):
                        for pt_idx in range(64):
                            pc.palette_popup_btn_ids.append([])
                            pc.palette_popup_btn_style_ids.append([])

                            with PopUp(position_top=True,
                                        on_click_outside=clicked_outside) as popup_id:
                                pc.popup_ids.append(popup_id)
                                pc.popup_btn_ids.append(add_button(label="Select Palette",
                                                               on_press=open_palette_popup,
                                                               show=False,
                                                               user_data=popup_id))
                                with Container(style_std=ContainerStyleStd.BorderedBox):
                                    with Column():
                                        for _ in range(8):
                                            style_id = add_button_style()
                                            pc.palette_popup_btn_style_ids[pt_idx].append(style_id)
                                            pc.palette_popup_btn_ids[pt_idx].append(
                                                add_button(label="Pal",
                                                            width=100,
                                                            style_id=style_id,
                                                            on_press=on_palette_selected,
                                                            user_data=popup_id))


start_session()
