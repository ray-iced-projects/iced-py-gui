from icedpygui import Window, Container, Column, start_session, \
    add_button

# The button demo add a new button to the window when the first
# button is pressed.  You can dynamically add any widget through a callback

count = 0
def on_press(_btn_id: int):
    global count
    count += 1
    # any widget can be added
    # you would probably use move_widget
    # to place the new widget properly
    # in this case, the new button just
    # appends to the column
    add_button(
        label=f"Button_{count}",
        )


# Add the windows
with Window(
    title="Adding Button", 
    size=(400, 400),  
    center=True,
    ):

    # Add a container to hold everything aligning all in the center
    with Container(fill=True, align_center=True):

        # Add a column to hold multiple widgets, vertically.
        with Column(spacing=20.0) as col:

            add_button(
                label="Press Me to Add A Button", 
                on_press=on_press)

start_session()
