from icedpygui import Window, Container, Column, start_session, \
    add_separator, IpgSeparatorType


# Add a window first
# if you make debug=True, you can see 
# the outline of the box containing
# the separators and the effect of the 
# width and height settings.
with Window(
    title="Separator Demo", 
    center=True,
    debug=False):

        with Container(fill=True, align_center=True):

            # Add a column to hold the widgets
            with Column(spacing=20.0):

                # for the dot, the width and height are not used
                # unless one wants a sort of padding around the circle.
                add_separator(
                    separator_type=IpgSeparatorType.Dot,
                    dot_radius=5.0,
                    dot_count=10,
                    spacing=5.0,)

                # The height is needed here and needs to be
                # larger than the height of the text.
                add_separator(
                    separator_type=IpgSeparatorType.Label,
                    height=22.0,
                    label="Some Label",
                    spacing=5.0)

                # The line needs both the width and the height
                add_separator(
                    width=200.0,
                    height=50.0,
                    separator_type=IpgSeparatorType.Line)

    
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
