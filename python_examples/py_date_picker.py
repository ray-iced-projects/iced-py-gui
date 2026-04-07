from icedpygui import Window, Column, Container, Row, start_session, \
    add_date_picker, DatePickerParam, add_button, ButtonStyleStd, \
    update_widget, add_text, ButtonParam, TextParam

# Callback for the date picker. The id is the date_picker id, so you have to get
# the id of whatever widget you want to update using a class or, for small projects,
# a global variable. Dataclass is not supported at this time, but you can use a class
# as seen in other demo files.
def date_selected(_dp_id: int, date: str):
    update_widget(
        wid=selected_date_id,
        param=TextParam.Content,
        value=f"You submitted {date}"
    )

# Another callback with some user data
def date_selected_with_ud(_dp_id: int, date: str, user_data: any):
    update_widget(
        wid=selected_with_ud_id,
        param=TextParam.Content,
        value=f"You submitted {date} with user_data = {user_data}"
    )

    update_widget(
        wid=btn_id,
        param=ButtonParam.Show,
        value=True
    )

# Another callback for the date picker that changes its size
def date_resize(_dp_id: int):
    update_widget(
        wid=dp2_id,
        param=DatePickerParam.SizeFactor,
        value=1.5
    )


# Add a window first
with Window(
    title="Date Picker Demo",
    size=(400, 800),
    center=True):

    # Add the container to center both x and y. Holds only one widget.
    with Container(fill=True, align_center=True):

        # Add a column to hold more than one widget and put this into the container.
        with Column(align_center=True, spacing=10.0):

            # Add info text
            add_text(
                content="Press the first calendar buttons to access the calendar. " +
                "Select a date, then press submit. Do the same for the second " +
                "button, and you will see another button to resize the calendar."
            )

            # The date picker size can be scaled from > 1.0. Anything less than 1 will
            # give an error and is not readable anyway.
            add_date_picker(
                size_factor=1.2,
                on_submit=date_selected
            )

            # Text widget id needed for callback.
            selected_date_id = add_text(content="No selection")

            # Another date picker to test the user_data and button style
            dp2_id = add_date_picker(
                size_factor=1.2,
                on_submit=date_selected_with_ud,
                user_data="Some user data",
                button_style_standard=ButtonStyleStd.Success
            )

            # Text widget id needed for callback.
            selected_with_ud_id = add_text(content="No selection")

            # Add the button for the resize but hide it until the second calendar is opened
            btn_id = add_button(
                label="Click to resize the calendar",
                on_press=date_resize,
                show=False
            )

# Required to be the last widget sent to Iced. If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
