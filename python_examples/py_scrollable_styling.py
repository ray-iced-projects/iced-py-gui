from imports import *

'''
    The Scrollable has a number of parts 
    to keep the parameter list from getting
    too larger, the styling is separated into
    5 parts.

    container_style_id,
    rail_x_style_id,
    rail_y_style_id,
    auto_scroll_style_id,
    gap_background_color,
    gap_background_rgba,
    
    The container styling used the - add_container_style
    Both rails are styled using - add_rail_style
    The auto scroll is styled using - add_auto_scroll_style
    The gap styling is just equated to a color or rgba type.
'''

# Create an id from each type of style or parameter
cont_id = add_container_style(
            background_color=Color.DARK_GREEN)

rail_x = add_rail_style (
            background_color=Color.GREEN)

# If any parameters are needed for a scroller, then the
# scroller parameter widget is needed
scroller_y = add_scroller_param(
                width=20.0,  # default=10.0
                margin=5.0, # default=0.0
            )

# Add the style ids to the scrollable style
scr_style = add_scrollable_style(
    container_style_id=cont_id,
    vertical_rail_style_id=rail_x,
    )


with Window(id="main", title="Scrollable", center=True):

    with Container(center=True, width_fill=True, height_fill=True):

        with Column(spacing=20.0):

            # if no add_scrollbar() use, the default is 
            # in the y direction
            add_scrollable(
                id="scroll_y", 
                width=200.0, 
                height=100.0,
                style_id=scr_style,         # add the scrollable style
                scroller_y_id=scroller_y,   # add the scroller parameters, if none, defaults to vertical, unless hidden
            )

            txt = ("This is Some Text \n")*20

            add_text(
                parent_id="scroll_y",
                content=txt,
            )


start_session()
