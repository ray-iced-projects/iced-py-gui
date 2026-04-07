from icedpygui import Window, Column, Container, Row, Scrollable, start_session, \
    add_checkbox, add_text, ContainerStyleStd, Icon


# callback to simply print if checkbox is checked
# The checkbox always return a bool and a user_data, if used
# Normally the chk_id would not be used, the user would
# be performing some other operation.  Therefore, it's
# good practise to remind yourself, that the id is the source
# and that you probably need other ids to do the operations.
def on_toggle(chk_id, is_checked):
    print(f"Checkbox id {chk_id} toggled to {is_checked}")

def on_toggle_user_data(chk_id, is_checked, user_data):
    print(f"Checkbox id {chk_id}, checked={is_checked}, user_data={user_data}")

with Window(title="Checkbox Demo",
            size=(800, 600), center=True):

    with Scrollable(width_fill=True):

        with Column(spacing=20.0, padding=[20.0], width_fill=True):
            
            # ***********************Checkbox Basics***************************

            add_text(content="Basic Checkbox\nCheck the ones for callback and user_data to see results printed")

            with Row(spacing=20.0):

                add_checkbox(label="Default Checkbox")

                add_checkbox(
                    label="With Callback",
                    on_toggle=on_toggle)

                add_checkbox(
                    label="With User Data",
                    on_toggle=on_toggle_user_data,
                    user_data="some info"
                    )
            
            # ***********************Checkbox state***************************

            add_text(content="Checkbox is_checked State")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Unchecked (default)",
                    is_checked=False)

                add_checkbox(
                    label="Checked",
                    is_checked=True)

            # ***********************Checkbox Sizing***************************

            add_text(content="Checkbox Size (box size)")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Size",
                    is_checked=True)

                add_checkbox(
                    label="Size 25",
                    size=25.0,
                    is_checked=True)

                add_checkbox(
                    label="Size 40",
                    size=40.0,
                    is_checked=True)

            # ***********************Spacing***************************

            add_text(content="Checkbox Spacing (gap between box and label)")

            with Row(spacing=40.0):

                add_checkbox(
                    label="Default Spacing",
                    is_checked=True)

                add_checkbox(
                    label="Spacing 5",
                    spacing=5.0,
                    is_checked=True)

                add_checkbox(
                    label="Spacing 30",
                    spacing=30.0,
                    is_checked=True)

            # ***********************Text sizes***************************
            
            add_text(content="Checkbox Text Size")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Text Size",
                    is_checked=True)

                add_checkbox(
                    label="Text Size 20",
                    text_size=20.0,
                    is_checked=True)

                add_checkbox(
                    label="Text Size 30",
                    text_size=30.0,
                    is_checked=True)

            # *************************Widths/Heights**********************
            
            add_text(content="Checkbox Width")

            with Row(spacing=20.0, width_fill=True):
                # Added to container to better show background
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_checkbox(
                        label="Default Width=Shrink",
                        is_checked=True)

                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_checkbox(
                        label="Width 200",
                        width=200.0,
                        is_checked=True)

                with Container(style_std=ContainerStyleStd.BorderedBox, width_fill=True):
                    add_checkbox(
                        label="Width Fill, fills remaining area",
                        width_fill=True,
                        is_checked=True)

            # ***********************Checkbox icons***************************

            add_text(content="Checkbox Icon Variations (check the boxes to see icons)")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Icon (Check)",
                    is_checked=True)

                add_checkbox(
                    label="X Icon",
                    icon=Icon.X,
                    is_checked=True)

                add_checkbox(
                    label="Asterisk Icon",
                    icon=Icon.Asterisk,
                    is_checked=True)

                add_checkbox(
                    label="Heart Icon",
                    icon=Icon.HeartFill,
                    is_checked=True)

            # ***********************Checkbox Icon Sizing***************************

            add_text(content="Checkbox Icon Size")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Icon Size",
                    is_checked=True)

                add_checkbox(
                    label="Icon Size 25",
                    icon_size=25.0,
                    is_checked=True)

                add_checkbox(
                    label="Icon Size 40",
                    icon_size=40.0,
                    size=40.0,
                    is_checked=True)

            # ***********************Checkbox No label***************************

            add_text(content="Checkbox No Label")

            with Row(spacing=20.0):

                add_checkbox(is_checked=True)

                add_checkbox(is_checked=False)


start_session()
