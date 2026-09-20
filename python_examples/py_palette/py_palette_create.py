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
    WidgetConfig, place_widget, set_new_widget_palette)
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    add_container_style,
    ContainerStyleStd,
    Row,
    Scrollable,
    start_session,
    add_button,
    ButtonParam,
    add_pick_list,
    MouseArea,
    PickListParam,
    PopUp,
    PopUpParam,
    Table,
    TableHeader,
    TableBody,
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
    get_widget_parameters,
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
        self.demo_color: list[float, 4] = [0.32, 0.2, 0.13, 1.0]
        self.demo_active: bool = False
        self.statuses: list[any] = []
        self.widget_active_id: int = None
        self.widget_active_style_id: int = None
        self.widget_hovered_id: int = None
        self.widget_hovered_style_id: int = None
        self.widget_pressed_id: int = None
        self.widget_pressed_style_id: int = None
        self.widget_disabled_id: int = None
        self.widget_disabled_style_id: int = None
        self.widget_normal_id: int = None
        self.widget_normal_style_id: int = None
        self.widget_name: str = None
        self.widget_parts: list[str] = []
        self.palette_widget_ids: list[int] = []
        self.palette_popup_cnt_ids: list[int] = []
        self.palette_popup_cnt_text_ids: list[int] = []
        self.palette_popup_cnt_style_ids: list[int] = []
        self.palette_popup_ma_ids: list[int] = []
        self.new_widget_row_id: int = None
        self.parts_file: dict = {}
        self.parts_file_name: str = None
        self.unique_parts_list: list[str] = []
        self.unique_status_list: list[str] = []
        self.parts_list_ids: list[int] = []
        self.selected_color: list[float, 4] = []
        self.palette: dict = {}
        self.palette_name: str = ""
        self.palette_ids: dict = {}  # palette_name -> palette_id
        self.palette_row_ids: list[int] = []  # list of row IDs for palette display
        self.row_ids: list[int] = [] # rows containing the widget and checkboxes
        self.checkbox_grid: list[list[int, 2]] = [] # 2D grid: [row][col] for matrix selection
        self.popup_ids: list[int] = []
        self.popup_open_btn_ids: list[int] = []

pc = PaletteCreator()

# Default directory
CWD = os.getcwd()
DEFAULT_DIRECTORY = os.path.join(CWD, "python_examples", "py_palette")
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
    place_widget(pc)


def on_color_picked(_cp_id: int, color: list[float]):
    """Handle color selection from ColorPicker."""
    pc.selected_color = color
    formatted = [round(c, 2) for c in color]
    update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {formatted}")
    demo_populate_palette_area(pc)


def on_color_text_input(_ti_id: int, text: str):
    """Handle color input from TextInput (format: [r, g, b, a])."""
    color = pm.parse_color_from_text_input(text)
    if color:
        pc.selected_color = color
        update_widget(selected_color_txt_id, TextParam.Content, f"Selected color = {color}")
    else:
        update_widget(selected_color_txt_id, TextParam.Content,
                      "Invalid color format, use float values")
    demo_populate_palette_area(pc)

# When the palette button is pressed this def is called
# Key point is to remember to add the user_data parameter
def on_palette_selected(btn_id, user_data: tuple[int, int]):
    """update the new widget colors
    user_data:
        int = button popup open column index
        int = palette index
    """
    # Will need to match both widget and status later
    (btn_open_col_index, pal_index) = user_data
    (popup_id_, open_id_) = pc.popup_open_btn_ids[btn_open_col_index]
    # (part_, status_, rgba, popup_id_, popup_btn_id) = user_data
    update_widget(popup_id_, PopUpParam.Opened, False)
    # set_new_widget_palette(pc, part_, status_, rgba)
    # change the palette select button to reflect the selected palette
    # params_select_btn = get_widget_parameters(btn_id)
    # update_widget(open_id_, ButtonParam.Label, params_select_btn.get("label"))


def open_palette_popup(_btn_id: int, popup_id_: int):
    """Opens the popup to select the palette"""
    update_widget(popup_id_, PopUpParam.Opened, True)


def clicked_outside(pop_id: int):
    """Called when mouse clicked outside palette selection popup"""
    update_widget(pop_id, PopUpParam.Opened, False)


# populate the dropdown for the demo widgets
parts_file_path = os.path.join(CWD, "python_examples", "py_palette", "widget_palette_parts.yml")
pc.demo_widget_list = WidgetConfig.get_widget_names_from_file(parts_file_path)

def load_demo(_pl_id: int, selected: str):
    """Loading a demo setup"""
    pc.widget_name = selected
    pc.selected_color = pc.demo_color
    update_widget(selected_color_txt_id, TextParam.Content,
                  f"Selected color = {pc.selected_color}")
    place_widget(pc)
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
                headers = ["Parts-Status", "Palette Selectors", "Palette Opacity", "Border Width"]
                with Table(
                    row_height=30.0,
                    col_widths=[200]*len(headers)):

                    with TableHeader():
                        for h in headers:
                            add_text(content=h, align_center=True, fill=True, size=14)

                    with TableBody():
                        # There are a possible 64 palette selectors (8 x 8)
                        # The info will be stored in a 8 x 8 lists
                        for row in range(64):
                            for column in range(4):
                                match column:
                                    case 0:
                                        with Container(fill=True):
                                            pc.parts_list_ids.append(add_text(content="part-status"))
                                    case 1:
                                        # Create a popup that holds the 8 containers
                                        # with a mouse area. Each of the 8 containers is
                                        # a color palette that can be selected
                                        pc.palette_popup_cnt_ids.append([])
                                        pc.palette_popup_cnt_style_ids.append([])
                                        pc.palette_popup_ma_ids.append([])
                                        pc.palette_popup_cnt_text_ids.append([])
                                        with PopUp(position_top=True,
                                                    on_click_outside=clicked_outside) as popup_id:
                                            # add the button to open the popup
                                            open_id = add_button(
                                                        label="Select Palette",
                                                        on_press=open_palette_popup,
                                                        width=130,
                                                        padding=[5],
                                                        user_data=popup_id)
                                            # store the ids
                                            pc.popup_open_btn_ids.append((popup_id, open_id))
                                            # Create the 8 containers in the popup to
                                            # hold the palettes
                                            with Container(
                                                style_std=ContainerStyleStd.BorderedBox):
                                                with Column(spacing=5):
                                                    for _ in range(8):
                                                        # a style id is needed to generate
                                                        # a palette later on
                                                        style_id = add_container_style()
                                                        # store the style_id
                                                        pc.palette_popup_cnt_style_ids[-1]\
                                                            .append(style_id)
                                                        # add the mouse area to detect the selection
                                                        with MouseArea(
                                                            on_press=on_palette_selected,
                                                            user_data=(row, column)
                                                            ) as ma_id:
                                                            # Store the mouse area id
                                                            pc.palette_popup_ma_ids[-1]\
                                                                .append(ma_id)
                                                            # add the container
                                                            with Container(width=100,
                                                                    style_id=style_id,
                                                                    align_center=True) as popup_cnt_id:
                                                                # store the id
                                                                pc.palette_popup_cnt_ids[-1].append(
                                                                    popup_cnt_id)
                                                                # Add some text which will later be
                                                                # changed to the palette name,
                                                                # store the id
                                                                pc.palette_popup_cnt_text_ids[-1]\
                                                                    .append(add_text(content="Pal"))
                                    case 2:
                                        add_text(content="Opacity")
                                    case 3:
                                        add_text(content="Border Width")

start_session()
