from imports import *

# Callback from toggler, updated a text widget
def toggled(_tog_id, is_toggled):
    update_widget(
        text_id, 
        IpgTextParam.Content, 
        f"The toggler is {is_toggled}.")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.
def update_label(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.Label, 
        "New Toggle Label")


def update_width(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.Width, 
        100.0)


def update_width_fill(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.WidthFill, 
        True)


def update_alignment(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.TextAlignment, 
        AlignX.Left)


def update_size(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.Size, 
        30.0)


def update_text_size(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.TextSize, 
        30.0)


def update_line_height(_btn_id):
    update_widget(
        tog_id, 
        IpgTogglerParam.LineHeight, 
        2.0)


# Add the window
with Window(
    id="main", 
    title="Toggler Demo",
    size=(700, 625),  
    center=True):

    # Add a main column to hold everything
    with Column(
        width_fill=True, 
        height_fill=True,
        align_x=IpgAlignment.Center, 
        spacing=5.0):

        # Add a column at the top for the toggler and a text widget
        with Column(
            align_x=IpgAlignment.Center,
            height=110.0, 
            spacing=5.0, 
            padding=[5.0], 
            width_fill=True):

            tog_id = add_toggler(toggled=toggled)

            text_id = add_text(content="The toggler is False.")

        # Add another column to hold the remaining buttons
        with Column(
            align_x=IpgAlignment.Center, 
            spacing=10.0, 
            padding=[5.0]):

            add_text(
                content="Press the buttons, below, in order to best see the effects, top to bottom, left to right")

            add_button(
                label="Update Label", 
                on_press=update_label)

            add_button(
                text_align_x=AlignX,
                label="Update Width\n the width will shrink stacking the label",
                on_press=update_width)

            add_button(
                label="Setting WidthFill=True\n The toggler expand the width of the container. the label is centered, "
                            "the default setting",
                on_press=update_width_fill)

            add_button(
                label="Setting the alignment to Left\n This caused the label to move left",
                on_press=update_alignment)

            add_button(
                label="Setting the size\n This makes the toggler bigger", 
                on_press=update_size)

            add_button(
                label="Increasing the TextSize", 
                on_press=update_text_size)

            # The text line height basically makes the outer box container the widget bigger
            add_button(
                label="Increasing the TextLineHeight\n Set window debug=True to see this better,",
                on_press=update_line_height)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
