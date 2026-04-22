#!/usr/bin/env python3
"""
IcedPyGuie Demo

IcedPyGui is based on Rust Iced gui at https://github.com/iced-rs/iced.
Some code is used from Iced_aw at https://github.com/iced-rs/iced_aw.
Pyo3 is used as the python wrapper at https://github.com/pyo3/pyo3.
Maturin is used to build and publish the module at https://github.com/PyO3/maturin.

IPG is easy to use.  The syntax used follows closely with
that found using dearpygui, my inspiration for this.
IPG has a backend of Rust versus c++ which does result in some
differences.  The key difference will be in the way
certain data is structured.  Rust doesn't allow mixed types in lists or
dictionaries, but this difference has mostly been shielded from the user.
The table is where you will mostly see this.  For example, instead of
using a dictionary for the data, List of dictionaries where used and
have distinct data types like {string, List[int]} or
{String, List[float]}, etc.  The only mixed list is a tuple but even
that has to be strictly defined in rust like (int, string) or (string, int).
The user data is special because it is only passed through to rust and
back out as a PyObject or PyAny.  Therefore any python data can be used
since it is never extracted into a rust type.

A few simple rules need to be followed.
    import IPG as indicated above in this demo.

    The last line of code to execute must be ipg.start_session().
    Any code after that will not be executed because rust Iced is now
    running.  You can place it anywhere, just make sure its last executed.
    If you start your program and nothing happens, it might mean that
    you aren't executing start_session() or you forgot to add it in.

    Every widget needs to have a parent container previously defined and
    every container needs to have a window and optionally a parent container
    defined.  If the container is placed into a window then no parent_id is required.
    Therefore at least one window needs to be added first and at least
    one container needs to be added to the window before any widgets are
    added.  As long as you have defined a parent, you can add to it.

The organization of your program is your choice.  You can use a class
or just functions for simple programs.  A @dataclass is not supported
at this time but should be soon.  You can add all of your containers
at once if you want or add the container and all or some of its widgets.
The key is that a container needs to be added first.

The major containers are Container, Column, and Row.  A Container can
have only one widget.  Therefore, if you have more than one, you need
to add them to a Column or a Row first and then place the Column or
Row into the Container, if needed.

There are some non-obvious containers like Scrollable.  It's not only a
container but a widget too, because it has callbacks.

Besides width and height, the Container defaults to centering which
aligns the item in the center of the Container.  This is very handy
for the centering of your Column or Row.  Other options are available.

A Column aligns your widget vertically.  So as you add widgets, they are
placed top to bottom.  The Column has a spacing parameter but you can add
the spacing widget, if you have other spacing requirements.

A Row is like the Column except it aligns the widgets horizontally.
As you place your widgets into a row, they are placed left to right.

The alignment depends a lot on the width and height of the container.
The 3 basic options for setting the width and height are:
    1. Shrink (default): container shrinks to the size of the largest widget.
    2. Specific value using a float.
    3. Setting the width_fill  or height_fill parameters to True which
        overrides the float, fills available space container its in.

The interaction of the above setting can be a bit difficult to figure out
sometimes.  However, by using the debug=True option in the window parameters,
you will be able to see how the layout looks.  If you don't see your widget
on the screen, its because certain combinations cause the fill to
exceed the windows width or height and your widgets are off screen.
I find placing everything into a Container and centering usually
brings it back on the screen.  You can also set a specific width
and height to help you figure how things are placed.

The nice thing about using fill as much as possible is that when you
resize your your window, everything resizes and you don't have to
go back and recalculate your sizes based on the window size.
In some cases you will need to do this so use the window
on_resize to get the width and height then recalculate your sizes
as needed in your callback using the update_item.

If you hide a widget, currently a small placeholder remains
future additions will add hidden with and without a placeholder.

A big part of constructing your gui layout is using the id's of
the widgets, containers, and windows.  The ids are a central part of
how IPG operates.  During the execution of the python program,
the functions are called in rust and a structure having all of the
necessary information for each widget is stored in a global list
based on the window id and the id of all the widgets.

Once the session is started, Iced is started up, the empty windows are
created, unique container ids are determined, and a recursive function
processes all of the nested children.

When the window needs to be updated, the update routine in Iced
determines which widget type needs to be updated by processing a generic
enum structure and then the widget type module is called.

The module for the widget type determines which widget needs changing
based on the id, makes changes and returns to the iced update function
which sends any new messages and/or updates the windows.

Since the window_id, container_id, and parent_id are strings, typo's
can occur throughout your program and changing them can be tedious
for large programs.  Therefore for larger programs, I prefer to assign
my ids in the class such as self.wnd_1: str="window_id_1",  for example.
Then your IDE will supply a dropdown of your variables and hopefully
reduce typos.  If you group them together properly, you might find
that your naming could be improved and easily changed.

Callbacks are the only way to update your windows, as discussed above,
Iced uses a messaging system and these are processed and sent back to
python by calling the specified function set by the user.  For example,
a button has an on_press=user supplied function, on_press=button_pressed.

The returning callback data varies depending on the widget.
For example, a button has no data so the callback only sends back
an id of the button.  A color_picker sends back a list of the rgba
values and so on.

The callbacks, as you'll see in the below program, have up to 3 returning
pieces of data, widget id, some data, and user_data.  Keep in mind that the id is the
id of the calling widget, which may or may not be the id you want to use
for updating an item.  Try not to use the term id in the parameter list because
that is a python reserved name.  Also, name the id after the calling widget so that
you remember what the widget is and if that's the id you want to use.

For example, if you have a callback for the button widget and want to change
a text widget to read "Button Pressed" then to update the text widget,
you'll need the text widget id.  You can get this by equating the text
widget to a variable which you would use as the id in update_item.

def create_button_and_text():
    ipg.add_button(parent_id="col", label="Press Me", on_press=button_pressed)
    text_id = ipg.add_text(parent_id="col", content="Some text")

Your callback function
def button_pressed(btn_id: int):
    ipg.update(text_id, IpgTextParam.Content, "Button was Pressed")

In this callback function you only have one returning parameter, btn_id.
Most other widgets have a second data parameter.  The user_data parameters
depends on if you use the user_data option on the widget.  If you don't use
the user_data option,make sure and not to put that parameter in the callback function
or you'll get an error.  You'll also get an error if you use the user_data and forget
to add that parameter to your callback function.  The names of the parameters
can be whatever you like, the order is the most important:
calling widget id, data(if present), user_data(if present).

It's important to look through all the demos to get a feel for how things operate.
I tried to vary up things to include different ideas.  However, a demo doesn't
really do much just use a lot of text widgets to show the results.  Give it a try with
with a real program and let me know through the git repository or discord if you have problems,
questions, or suggestions.

Have fun with IPG!!
"""

from icedpygui import Window, Container, Column, Row, Table, start_session, \
    add_button, add_checkbox, add_date_picker, add_progress_bar, add_slider, \
    add_pick_list, add_radio, add_text, add_text_input, \
    ButtonParam, ProgressBarParam, TextParam, update_widget


class Demo:
    """ IcedPyGui Demo"""
    def __init__(self) -> None:

        # window ids
        self.wnd_1: str="main_1"
        self.wnd_2: str="main_2"

        # containers for window 1
        self.row_1: str="row_1"
        self.l_col_1: str="left_col_1"
        self.r_col_1: str="right_col_1"

        # widgets in window 1
        # 0 is not a valid id so if not initialized,
        # you'll get an error of not finding
        # the id.
        self.btn_id: int=0
        self.button_presses: int=0
        self.btn_text_id: int=0
        self.text_id_chkbox: int=0
        self.bar_id: int=0
        self.slider_text_id: int=0
        self.picklist_text_id: int=0
        self.radio_1_text_id: int=0
        self.radio_2_text_id: int=0
        self.text_input_id: int= 0

        # containers for window 2
        self.l_col_2: str="left_col_2"
        self.r_col_2: str="right_col_2"

        # Widgets in window 2
        self.date_text_id: int=0

    def start_gui(self):
        """Starup Methods"""
        self.construct_window_1()
        self.construct_window_2()

        # required to be last executed
        start_session()

    def construct_window_1(self):
        """Wndow 1 Construct"""
        with Window(title="Demo Window 1 - Iced Wrapped in Python",
                    size=[500, 500],
                    position=[100, 25]) as self.wnd_1:

            # Add container to center everything
            with Container(fill=True, padding=[20]):

                # add row to hold the 2 columns
                with Row(spacing=20):

                    # *****************LeftColumn in Window 1*************************
                    with Column(spacing=20):
                        add_button(label="Press Me!",
                                    on_press=self.button_pressed)

                        self.btn_text_id = \
                            add_text(content=f"A text can count too {self.button_presses}")

                        add_checkbox(label="Check Me",
                                     on_toggle=self.box_checked_id)

                        self.text_id_chkbox = \
                            add_text(content="You Checked the box above",
                                     show=False)  # note: show is False

                        self.bar_id = add_progress_bar(
                                min=0.0,
                                max=100.0,
                                value=50.0,
                                width_fill=True)

                        add_slider(
                            min=0.0,
                            max=100.0,
                            step=1.0,
                            value=50.0,
                            width_fill=True,
                            on_change=self.slider_on_change,
                            on_release=self.slider_on_release)

                        self.slider_text_id = \
                            add_text(content="Slider content here.")

                        add_pick_list(
                            options=["one", "two", "three"],
                            on_select=self.picked_item,
                            placeholder="Choose a string number")

                        self.picklist_text_id = \
                            add_text(content="You picked:")

                        add_radio(
                            labels=["A", "B", "C"],
                            horizontal=True,
                            on_selected=self.radio_selected_h)

                        self.radio_2_text_id = \
                            add_text(content="You selected:")

                    # *****************Right Column in Window 1*************************
                    with Column(spacing=20):

                        add_radio(
                            labels=["Radio A", "Radio B", "Radio C"],
                            on_selected=self.radio_selected_v)

                        self.radio_1_text_id = \
                            add_text(content="You selected:")

                        add_text_input(
                            placeholder="My Placeholder",
                            width=200.0,
                            on_submit=self.text_input_submitted,
                            on_input=self.text_on_input)

                        self.text_input_id = \
                            add_text(content="Will fill while typing")

    def button_pressed(self, btn_id: int):
        """Button Callback"""
        self.button_presses += 1
        update_widget(
            btn_id,
            ButtonParam.Label,
            f"You Pressed {self.button_presses} times!")

        update_widget(
            self.btn_text_id,
            TextParam.Content,
            f"A text can count too: {self.button_presses} times!")

    def box_checked_id(self, _chk_id: int, show: bool):
        """Checkbox Callback"""
        update_widget(
            self.text_id_chkbox,
            TextParam.Show,
            show)  # show set to True

    def slider_on_change(self, _slider_id: int, data: float):
        """Slider Callback"""
        update_widget(
            self.slider_text_id,
            TextParam.Content,
            f"Slide = {data}")

        update_widget(
            self.bar_id,
            ProgressBarParam.Value,
            data)

    def slider_on_release(self, slider_id, data: float):
        """Slider Callback"""
        print(slider_id, data)

    def picked_item(self, _pl_id: int, data: str):
        """Picklist Callback"""
        update_widget(
            self.picklist_text_id,
            TextParam.Content,
            f"You Picked: {data}")

    # The radio on_select returns a tuple (index, label)
    def radio_selected_v(self, _radio_id: int, data: tuple[int, str]):
        """Radio Callback"""
        update_widget(
            self.radio_1_text_id,
            TextParam.Content,
            f"You selected: {data}")

    # The radio on_select returns a tuple (index, label)
    def radio_selected_h(self, _radio_id: int, data: tuple[int, str]):
        """Radio Callback"""
        update_widget(
            self.radio_2_text_id,
            TextParam.Content,
            f"You selected: {data}")


    def text_input_submitted(self, _input_id, data: str):
        """Text Input Callback - Submitted"""
        update_widget(
            self.text_input_id,
            TextParam.Content,
            f"You submitted: {data}")

    def text_on_input(self, _input_id, data: str):
        """Text Input Callback - On Input"""
        update_widget(
            self.text_input_id,
            TextParam.Content,
            f"Adding while typing: {data}")

    # **********************window_2*****************************************************

    def construct_window_2(self):
        """Construct Window 2"""
        with Window(
            title="Demo Window 2 - Iced Wrapped in Python",
            size=[600, 500],
            position=[650, 25]) as self.wnd_2:

            # Add container to center everything
            with Container(fill=True):

                # add a column to hold multiple widgets
                with Column(width_fill=True, align_center=True) as col_id:

                    add_date_picker(on_submit=self.date_selected)

                    self.date_text_id = add_text(content="")

                    self.construct_table(col_id)


    def date_selected(self, _date_id, date: str):
        """Date Picker Callback"""
        update_widget(
            self.date_text_id,
            TextParam.Content,
            f"You selected: {date}")

    def construct_table(self, col_id: int):
        """Table Construct"""
        # define the column widths
        column_widths = [100.0] * 4
        # create the data
        headers = ["str", "one", "two", "three"]
        body = [
            [0.0, 1.0, 2.0, 3.0],
            [0.0, 2.0, 4.0, 6.0],
            [0.0, 3.0, 6.0, 9.0],
            [0.0, 4.0, 8.0, 12.0],
            [0.0, 5.0, 10.0, 15.0],
            [0.0, 6.0, 12.0, 18.0],
            [0.0, 7.0, 14.0, 21.0],
            [0.0, 8.0, 16.0, 24.0],
            [0.0, 9.0, 18.0, 27.0],
            [0.0, 10.0, 20.0, 30.0],
            [0.0, 11.0, 22.0, 33.0],
        ]
        footers = ["", "", "", ""]

        # Add the table.
        with Table(
            window_id=self.wnd_2,
            parent_id=col_id,
            headers=headers,
            body=body,
            footers=footers,
            column_widths=column_widths,
            height=150.0,
            custom_footer_rows=1):

            footer = ["This", "is", "a", "footer"]

            for f in footer:
                add_text(
                    content=f,
                    size=14.0)

    def widget_button(self, tbl_id: int, wid_index: tuple[int, int]):
        """Table Callback"""
        print(tbl_id, wid_index)

    def widget_checkbox(self, tbl_id: int, wid_index: tuple[int, int], is_checked: bool):
        """Table Callback"""
        print(tbl_id, wid_index, is_checked)

    def on_text_enter(self, tbl_id, text_index: tuple[int, int]):
        """Table Callback"""
        print(tbl_id, text_index)



demo = Demo()
demo.start_gui()
