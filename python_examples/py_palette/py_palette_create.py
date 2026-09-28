#!/usr/bin/env python3
"""
Palette Creator: Interactive tool to create and manage widget color palettes.

Workflow:
1. Load or create a palette file
2. Define base color using ColorPicker or text input
3. Create palette with status variants (Active, Hovered, Pressed, Disabled)
4. Get palette_id to use with widgets
"""

from dataclasses import dataclass
import json
from pathlib import Path
import os

from python_examples.py_palette.demo_helpers import demo_populate_palette_area
from python_examples.py_palette.widget_helpers import (
    WidgetConfig, place_selected_widget, set_selected_widget_opacity, set_selected_widget_palette)
from icedpygui import (
    Window,
    Arrow,
    ColorPicker,
    Column,
    Container,
    add_container_style,
    ContainerStyleStd,
    Row,
    Scrollable,
    start_session,
    add_button,
    add_button_style,
    ButtonParam,
    ButtonStyleStd,
    add_event_keyboard,
    InputFloat,
    InputFloatParam,
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
    StateVariant,
    StylePart,
    add_file_system_dialog,
    FileSystemDialogParam as FsdParam,
    FileSystemDialogCallbackType as FsdType,
    update_widget,
    generate_id,
    get_widget_parameters,
    get_widget_palette_parameters,
)



# ============================================================================
# Palette storage and file I/O
# ============================================================================

PALETTES_DIR = Path.home() / ".icedpygui_palettes"
PALETTES_DIR.mkdir(exist_ok=True)

@dataclass
class WidgetStatus:
    """Statuses"""
    Active=...
    Hovered=...
    Disabled=...
    Pressed=...


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

opacity_default_kp = {'location': 'standard', 'modifier': 'None',
               'name': 'key pressed', 'key': 'ArrowDown'}
border_default_kp = {'location': 'standard', 'modifier': 'None',
               'name': 'key pressed', 'key': 'ArrowUp'}

class PaletteCreator:
    """Manages palette state and widget selection logic."""

    def __init__(self):
        self.widget_name: str = None,
        self.widget_list: list[str] = []
        self.demo_widget_list: list[str] = []
        self.demo_color: list[float, 4] = [0.32, 0.2, 0.13, 1.0]
        self.demo_active: bool = False
        self.statuses: list[any] = []

        self.selected_widget_statuses: dict = {
            "active": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
            "hovered": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
            "pressed": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
            "disabled": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
            "normal": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
            "checked": {"name": None, "wid": 0, "style_id": 0, "pal_id": 0},
        }

        self.opacity_btn_hovered: int = 0
        self.opacity_btn_ids: dict = {}
        self.opacity_input_ids: list[int] = []
        self.opacity_key_pressed: dict = opacity_default_kp
        self.opacity_step_size: float = 0.1
        self.opacity_step_txt_id: int = 0.1
        self.opacity: float = 1.0

        self.border_btn_hovered: int = 0
        self.border_btn_ids: dict = {}
        self.border_input_ids: list[int] = []
        self.border_key_pressed: dict = border_default_kp
        self.border_step_size: float = 1.0
        self.border: float = 2.0

        self.palette_ids: dict = {}  # palette_name -> palette_id
        self.palette_name: str = ""
        self.palette_popup_cnt_ids: list[int] = []
        self.palette_popup_cnt_style_ids: list[int] = []
        self.palette_popup_cnt_text_ids: list[int] = []
        self.palette_popup_ma_ids: list[int] = []
        self.palette_row_ids: list[int] = []  # list of row IDs for palette display
        self.palette_widget_ids: list[int] = []
        self.palette: dict = {}

        self.parts_file_name: str = None
        self.parts_file: dict = {}
        self.parts_list_ids: list[int] = []
        self.part_status_cnts: list[int] = []

        self.selected_widget_row_id: int = None
        self.selected_color: list[float, 4] = []

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
    place_selected_widget(pc)


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
def on_palette_selected(_btn_id, user_data: tuple[int, int]):
    """update theselected widget colors
    user_data:
        int = button popup open row index
        int = palette index (0..8) from the popup container
    """
    # Will need to match both widget and status later
    (row_, pal_index) = user_data
    (popup_id_, _) = pc.popup_open_btn_ids[row_]

    update_widget(popup_id_, PopUpParam.Opened, False)
    set_selected_widget_palette(pc, row_, pal_index)


def open_palette_popup(_btn_id: int, popup_id_: int):
    """Opens the popup to select the palette"""
    update_widget(popup_id_, PopUpParam.Opened, True)


def clicked_outside(pop_id: int):
    """Called when mouse clicked outside palette selection popup"""
    update_widget(pop_id, PopUpParam.Opened, False)


def event_kp(_kb_id: int, keyboard: dict):
    """Key press event"""
    if pc.opacity_btn_hovered == 0 and pc.border_btn_hovered == 0:
        return

    key_pressed = keyboard.get("key")

    if key_pressed == "Escape":
        if pc.opacity_btn_hovered > 0:
            pc.opacity_btn_ids[pc.opacity_btn_hovered]["modifier"] = "None"
        else:
            pc.border_btn_ids[pc.border_btn_hovered]["modifier"] = "None"
        return

    # Handle opacity button
    if pc.opacity_btn_hovered > 0:
        if key_pressed == "Control":
            pc.opacity_btn_ids[pc.opacity_btn_hovered]["modifier"] = key_pressed
        else:
            pc.opacity_btn_ids[pc.opacity_btn_hovered]["key"] = keyboard.get("key")

    # Handle border button
    if pc.border_btn_hovered > 0:
        if key_pressed in ["Control", "Shift"]:
            pc.border_btn_ids[pc.border_btn_hovered]["modifier"] = key_pressed
        else:
            pc.border_btn_ids[pc.border_btn_hovered]["key"] = keyboard.get("key")

    match key_pressed:
        case "ArrowUp":
            if pc.opacity_btn_hovered > 0:
                update_widget(pc.opacity_btn_hovered, ButtonParam.StyleArrow, Arrow.ArrowUp)
            else:
                update_widget(pc.border_btn_hovered, ButtonParam.StyleArrow, Arrow.ArrowUp)
        case "ArrowDown":
            if pc.opacity_btn_hovered > 0:
                update_widget(pc.opacity_btn_hovered, ButtonParam.StyleArrow, Arrow.ArrowDown)
            else:
                update_widget(pc.border_btn_hovered, ButtonParam.StyleArrow, Arrow.ArrowDown)


def event_kr(_kb_id: int, _key: dict):
    """Kep release event"""


add_event_keyboard(enabled=True,
                   on_key_press=event_kp,
                   on_key_release=event_kr)

def opacity_btn_pressed(btn_id_: int):
    """Increment or decrement opacity based on stored modifier-key combination."""
    if btn_id_ not in pc.opacity_btn_ids:
        return

    btn_state = pc.opacity_btn_ids[btn_id_]
    modifier = btn_state.get("modifier", "None")
    key = btn_state.get("key", "ArrowDown")

    # Determine step size based on modifier
    step = pc.opacity_step_size
    if modifier == "Control":
        step = step / 10  # Fine adjustment: 0.01 instead of 0.1

    # Apply step based on key direction
    opacity = btn_state["opacity"]
    if key == "ArrowUp":
        opacity += step
    elif key == "ArrowDown":
        opacity -= step
    else:
        return  # Invalid key

    # Clamp to [0, 1]
    opacity = max(0.0, min(1.0, opacity))
    btn_state["opacity"] = opacity

    # Update the widget
    input_id = btn_state.get("input_id")
    if input_id:
        update_widget(input_id, InputFloatParam.Value, opacity)
        set_selected_widget_opacity(pc, btn_id_)


def border_btn_pressed(btn_id_: int, border_input_id: int):
    """Increment or decrement border width based on stored modifier-key combination."""
    if btn_id_ not in pc.border_btn_ids:
        return

    btn_state = pc.border_btn_ids[btn_id_]
    modifier = btn_state.get("modifier", "None")
    key = btn_state.get("key", "ArrowUp")

    # Determine step size based on modifier
    if modifier == "None":
        step = 1.0      # Normal: ±1.0
    elif modifier == "Control":
        step = 0.5      # Medium: ±0.5
    elif modifier == "Shift":
        step = 0.1      # Fine: ±0.1
    else:
        return          # Unknown modifier

    # Apply step based on key direction
    width = btn_state["width"]
    if key == "ArrowUp":
        width += step
    elif key == "ArrowDown":
        width -= step
    else:
        return  # Invalid key

    # Clamp to [0, ∞)
    width = max(0.0, width)
    btn_state["width"] = width

    # Update the widgets
    # get the btn_index which correlates to the row
    btn_ids = list(pc.opacity_btn_ids.keys())
    if btn_id in btn_ids:
        idx = btn_ids.index(btn_id)

    update_widget(border_input_id, InputFloatParam.Value, width)


def ma_entered(_ma_id: int, btn_id_: int):
    """Determine which opacity button is hovered"""
    if pc.opacity_btn_ids.get(btn_id_):
        pc.opacity_btn_hovered = btn_id_
        pc.border_btn_hovered = 0

    if pc.border_btn_ids.get(btn_id_):
        pc.opacity_btn_hovered = 0
        pc.border_btn_hovered = btn_id_


def ma_exited(_ma_id, _):
    """Unsets the hovered op_id"""
    pc.opacity_btn_hovered = 0
    pc.border_btn_hovered = 0


# populate the dropdown for the demo widgets
parts_file_path = os.path.join(CWD, "python_examples", "py_palette", "widget_palette_parts.yml")
pc.demo_widget_list = WidgetConfig.get_widget_names_from_file(parts_file_path)

def load_demo(_pl_id: int, selected: str):
    """Loading a demo setup"""
    pc.widget_name = selected
    pc.selected_color = pc.demo_color
    update_widget(selected_color_txt_id, TextParam.Content,
                  f"Selected color = {pc.selected_color}")
    place_selected_widget(pc)
    demo_populate_palette_area(pc)

def print_pal(_btn_id):
    """Temp"""
    params = get_widget_parameters(4463)
    print(params, "\n")
    pal = get_widget_palette_parameters(4461)
    for status in pal["statuses"]:
        if status.get("status") == "Active":
            print(status)

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
                add_button(label="print pal", on_press=print_pal)
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

                with Row(spacing=10) as selected_widget_row_id:
                    pc.selected_widget_row_id = selected_widget_row_id
                # The selected widget should be placed here


                # This area is hidden until the color and widget is selected
                # area prepopulated with widget that only need to be updated later
                # one could take the approach to add these widgets as needed.

                headers = ["Parts-Status", "Palette Selectors", "Palette Opacity", "Border Width"]
                with Table(
                    row_height=30.0,
                    col_widths=[200, 150, 125, 125],
                    ):

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
                                        with Container(fill=True, show=False) as part_status_cnt:
                                            pc.parts_list_ids.append(
                                                add_text(content="part-status"))
                                            pc.part_status_cnts.append(part_status_cnt)
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
                                            btn_style_id = add_button_style(text_center=True)
                                            open_id = add_button(
                                                        label="Select Palette",
                                                        on_press=open_palette_popup,
                                                        width=150,
                                                        style_id=btn_style_id,
                                                        style_std=ButtonStyleStd.Subtle,
                                                        show=False,
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
                                                            user_data=(row, column),
                                                            ) as ma_id:
                                                            # Store the mouse area id
                                                            pc.palette_popup_ma_ids[-1]\
                                                                .append(ma_id)
                                                            # add the container
                                                            with Container(width=100,
                                                                    style_id=style_id,
                                                                    align_center=True)\
                                                                    as popup_cnt_id:
                                                                # store the id
                                                                pc.palette_popup_cnt_ids[-1].append(
                                                                    popup_cnt_id)
                                                                # Add some text which will later be
                                                                # changed to the palette name,
                                                                # store the id
                                                                pc.palette_popup_cnt_text_ids[-1]\
                                                                    .append(add_text(content="Pal"))
                                    case 2:
                                        with InputFloat(
                                            value=1.0,
                                            align_center=True,
                                            show=False) as input_op_id:
                                            # add mouse area to detect when hovered
                                            # key changes only when button is hovered
                                            btn_id = generate_id()
                                            with MouseArea(
                                                on_enter=ma_entered,
                                                on_exit=ma_exited,
                                                user_data=btn_id
                                                ):
                                                add_button(
                                                    on_press=opacity_btn_pressed,
                                                    style_arrow=Arrow.ArrowDown,
                                                    gen_id=btn_id)
                                                pc.opacity_input_ids.append(input_op_id)
                                                pc.opacity_btn_ids[btn_id] = {
                                                    "modifier": "None",
                                                    "key": "ArrowDown",
                                                    "opacity": 1.00,
                                                    "input_id": input_op_id,
                                                    "row": row,
                                                }
                                    case 3:
                                        with InputFloat(
                                            value=0.0,
                                            align_center=True,
                                            show=False) as input_border_id:
                                            btn_id = generate_id()
                                            # add mouse area to detect when hovered
                                            # key changes only when button is hovered
                                            with MouseArea(
                                                on_enter=ma_entered,
                                                on_exit=ma_exited,
                                                user_data=btn_id
                                                ):
                                                add_button(
                                                    on_press=border_btn_pressed,
                                                    style_arrow=Arrow.ArrowUp,
                                                    user_data=input_border_id,
                                                    gen_id=btn_id)
                                                pc.border_input_ids.append(input_border_id)
                                                pc.border_btn_ids[btn_id] = {
                                                    "modifier": "None",
                                                    "key": "ArrowUp",
                                                    "width": 2.00,
                                                    "row": row,
                                                }

if __name__ == "__main__":
    start_session()
