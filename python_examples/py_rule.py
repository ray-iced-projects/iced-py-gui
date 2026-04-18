#!/usr/bin/env python3
"""
Rule demo use demo

The Rule is a widget that can separate items,
it's essentially a line widget.
"""

from icedpygui import Window, Container, Column, start_session, \
    add_space, add_rule, add_rule_style, add_text, Color, ContainerStyleStd


# add some styling
st1 = add_rule_style(
            color=Color.YELLOW,
            border_radius=[10.0])

st2 = add_rule_style(
            color=Color.BLUE,
            border_radius=[20.0])

# The fill_mode styling
# The rule will still fill the container
# but the color will only be 50%
st3 = add_rule_style(
            color=Color.BLUE,
            fillmode_percent=50.0)

# The padding is almost like percent except it gives you
# an unsymmetrical ability.
st4 = add_rule_style(
            color=Color.BLUE,
            fillmode_asymmetric_padding=(10, 50))


# Add the window
with Window(title="Rule Demo", center=True):

    # Add a container for alignment
    with Container(fill=True, align_center=True):

        # Add a column to hold the widgets
        with Column(align_center=True, spacing=10.0, fill=True):

            # Add some spacing
            add_space(width_fill=True, height=20.0)

            # Add info
            add_text(content="The rule's length are determined by the container they are in")
            add_text(content="The holding containers' width=200 controls the horizontal")
            add_text(content="The holding container's height=100 controls the vertical")
            add_text(content="All of these could hve used a single Column, except for the vertical")
            add_text(content="The Container helps highlight the rule's position better, in this case")

            with Container(style_std=ContainerStyleStd.BorderedBox,
                           width=200, height=100, align_center=True):
                # Add the rules
                add_rule(
                    thickness=5,
                    is_vertical=True,
                    style_id=st1)

            with Container(style_std=ContainerStyleStd.BorderedBox,
                           width=200, height=50, align_center=True):
                add_rule(
                    thickness=10,
                    style_id=st2)

            add_text(content="Styling added to above with color and corner radius")

            with Container(style_std=ContainerStyleStd.BorderedBox,
                           width=200, height=50, align_center=True):
                add_rule(
                    thickness=5,
                    style_id=st3)

            add_text(content="Styling added to above with 50% color fill")

            with Container(style_std=ContainerStyleStd.BorderedBox,
                           width=200, height=50, align_center=True):

                add_rule(
                    thickness=5,
                    style_id=st4)

            add_text(content="Styling added to above with unsymmetrical padding")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
