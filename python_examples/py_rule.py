from icedpygui import Window, Container, Column, start_session, \
    add_space, add_rule, add_rule_style, add_text, Color


# add some styling
st1 = add_rule_style( 
            color=Color.YELLOW, 
            border_radius=[10.0])

st2 = add_rule_style(
            color=Color.BLUE, 
            border_radius=[10.0])

# The fill_mode styling
st3 = add_rule_style(
            color=Color.BLUE, 
            fillmode_percent=50.0)

# The padding is almost like percent except it gives you
# an unsymmetrical ability. It can be seen better if you uncomment the 
# debug mode in the window.
st4 = add_rule_style(
            color=Color.BLUE,
            fillmode_asymmetric_padding=(10, 50))
            
            
# Add the window
with Window(title="Rule Demo", center=True):

    # Add a container for alignment
    with Container(fill=True):

        # Add a column to hold the widgets
        with Column(align_center=True, spacing=20.0, width=200.0):

            # Add some spacing
            add_space(width_fill=True, height=20.0)

            # Add info
            add_text( content="Below are vertical and horizontal rules")

            # Add the rules
            add_rule( 
                thickness=50,
                is_vertical=True, 
                style_id=st1)

            add_rule(
                thickness=5,
                style_id=st2)

            add_text(content="Styling added to above with color and corner radius")

            add_rule(
                thickness=5,
                style_id=st3)

            add_text(content="Additional Styling added to above with 50% color fill")

            add_rule(
                thickness=5, 
                style_id=st4)

            add_text(content="Additional Styling added to above with unsymmetrical padding")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
