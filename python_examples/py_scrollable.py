#!/usr/bin/env python3
"""
Scrollable demo

The scollable has 5 parts.
1. add_scrollable: The actual widget
2. add_scrollable_style: The widget style
3. add_scroller: Used when more scroller parameters are needed.
4. add_rail_style: The styling for the rail(s).
5. add_autoscroll_style: The styling used when auto scrolling,
        mouse or mouse pad movement which causes scrolling.

Add a scrollable container widget.

A scrollable container that can scroll its children
vertically, horizontally, or both.

Parameters
----------
window_id : str
    Sets the window this scrollable belongs to.
container_id : str
    Sets the Unique string identifier for the scrollable.
parent_id : str, Optional
    Sets the parent container ID.  Defaults to the window itself.
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the scrollable fills available width.
height : float, Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the scrollable fills available height.
fill : bool, Optional
    Whether to fill both the available width and height
both_scrollers : bool, Optional
    Whether to show both horizontal and vertical scrollers.
scroller_x_id : int, Optional
    Sets the ID of the horizontal scroller parameters.
scroller_y_id : int, Optional
    Sets the ID of the vertical scroller parameters.
on_scroll : callable, Optional
    Sets the Callback method to invoke when the scrollable is scrolled.
user_data : Any, Optional
    Sets the Arbitrary data forwarded to callbacks.
style_id : int, Optional
    Sets the ID of a custom style created with ``add_scrollable_style``.

Returns
-------
int
    The numeric widget ID of the newly created scrollable.
**********************************************************
Add styling to a scrollable.

Creates a custom style that can be applied to a scrollable
via its ``style_id`` parameter.

Parameters
----------
vertical_rail_style_id : int, Optional
    Sets the ID of a rail style for the vertical scrollbar.
horizontal_rail_style_id : int, Optional
    Sets the ID of a rail style for the horizontal scrollbar.
auto_scroll_style_id : int, Optional
    Sets the ID of an autoscroll style.
gap_color : Color, Optional
    Sets the gap color using a predefined color variant.
gap_color_alpha : float, Optional
    Sets the alpha of the Color.
gap_rgba : list of float, Optional
    Sets the gap color in rgba format as [r, g, b, a].
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a scrollable's ``style_id``.
***************************************************************
Add scroller parameters.

Creates scroller parameters that can be assigned to a
scrollable's ``scroller_x_id`` or ``scroller_y_id``.

Parameters
----------
width : float, Optional
    Sets the width of the scrollbar track.
margin : float, Optional
    Sets the margin around the scrollbar.
scroller_width : float, Optional
    Sets the width of the scroller thumb.
spacing : float, Optional
    Sets the spacing between the scrollbar and content.
anchor : Anchor, Optional
    Sets the anchor position of the scrollbar.
hidden : bool, Optional
    Whether the scrollbar is hidden.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric ID to pass to a scrollable's scroller parameter.
****************************************************************
Add styling to a scrollbar rail.

Creates a custom rail style that can be applied to a
scrollable style's ``vertical_rail_style_id`` or
``horizontal_rail_style_id``.

Parameters
----------
background_color : Color, Optional
    Sets the rail background color using a predefined color variant.
background_color_alpha : float, Optional
    Sets the alpha of the Color.
background_rgba : list of float, Optional
    Sets the rail background color in rgba format as [r, g, b, a].
border_color : Color, Optional
    Sets the rail border color using a predefined color variant.
border_color_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba : list of float, Optional
    Sets the rail border color in rgba format as [r, g, b, a].
border_width : float, Optional
    Sets the rail border width in logical pixels.
border_radius : list of float, Optional
    Sets the radius of the rail corners as [all] or
    [top-left, top-right, bottom-right, bottom-left].
scroller_background_color : Color, Optional
    Sets the scroller thumb background color using a predefined color variant.
scroller_background_color_alpha : float, Optional
    Sets the alpha of the Color.
scroller_background_rgba : list of float, Optional
    Sets the scroller thumb background color in rgba format as [r, g, b, a].
scroller_border_color : Color, Optional
    Sets the scroller thumb border color using a predefined color variant.
scroller_border_color_alpha : float, Optional
    Sets the alpha of the Color.
scroller_border_rgba : list of float, Optional
    Sets the scroller thumb border color in rgba format as [r, g, b, a].
scroller_border_width : float, Optional
    Sets the scroller thumb border width in logical pixels.
scroller_border_radius : list of float, Optional
    Sets the radius of the scroller thumb corners as [all] or
    [top-left, top-right, bottom-right, bottom-left].
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a scrollable style's rail parameter.
    
****************************************************************
Add styling to an autoscroll indicator.\n///
Creates a custom autoscroll style that can be applied to a
scrollable style's ``auto_scroll_style_id``.

Parameters
----------
background_color : Color, Optional
    Sets the background color using a predefined color variant.
background_color_alpha : float, Optional
    Sets the alpha of the Color.
background_rgba : list of float, Optional
    Sets the background color in rgba format as [r, g, b, a].
border_color : Color, Optional
    Sets the border color using a predefined color variant.
border_color_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba : list of float, Optional
    Sets the border color in rgba format as [r, g, b, a].
border_width : float, Optional
    Sets the border width in logical pixels.
border_radius : list of float, Optional
    Sets the radius of the corners as [all] or
    [top-left, top-right, bottom-right, bottom-left].
shadow_color : Color, Optional
    Sets the shadow color using a predefined color variant.
shadow_color_alpha : float, Optional
    Sets the alpha of the Color.
shadow_rgba : list of float, Optional
    Sets the shadow color in rgba format as [r, g, b, a].
shadow_offset : list of float, Optional
    Sets the shadow offset as [x, y] in logical pixels.
shadow_blur_radius : float, Optional
    Sets the shadow blur radius in logical pixels.
shadow_icon_color : Color, Optional
    Sets the shadow icon color using a predefined color variant.
shadow_icon_color_alpha : float, Optional
    Sets the alpha of the Color.
shadow_icon_rgba : list of float, Optional
    Sets the shadow icon color in rgba format as [r, g, b, a].
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a scrollable style's ``auto_scroll_style_id``.
"""

from icedpygui import Window, Container, Column, start_session, \
        Scrollable, add_scroller, add_rail_style, add_scrollable_style, \
        add_text, Color, ContainerStyleStd, add_space

# The rail id is used in the add_scrollable_style
rail_x = add_rail_style (
            background_color=Color.GREEN)

# Add the style ids to the scrollable style
scr_style = add_scrollable_style(
    horizontal_rail_style_id=rail_x,
    )

# the scroller return a large amount of data in dict format
# just select what you need to use.
# The scroller callback is activated on startup, if you don't
# want the callback to be executed at that point, add a condition
# as been done here either using the x or y absolute to prevent
# the callback from executing before any scrolling occus.
def on_scroll(_scroller_id, data: dict):
    """Scroller callback"""
    if data['abs_y'] != 0:
        print(data)



with Window(title="Scrollable Demo", center=True):

    with Container(align_center=True, fill=True):

        with Column(spacing=50.0):

            # default in the y direction, if you want to change the parameters
            # of the y scroller, use add_scroller(scroller parameters)
            with Scrollable( width=200.0, height=100.0, on_scroll=on_scroll):
                with Container(
                    width_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):

                    add_text(content=("This is Some Text \n")*20)

            # The y direction scroller will show when the height of the widgets exceed
            # the height of the container.  Therefore, the only time you need to use the
            # scroller_y_id is when you want to change the parameters.  The scroller_x
            # will always need to be added when needed.
            with Scrollable(width=200.0,
                            height=50.0,
                            scroller_x_id=add_scroller()):

                add_text(content=("A lot of text, ")*20)

            # Some styling added
            # Auto scroll is set to True which allows one to
            # use the mouse to scroll, click the middle mouse button
            # move the mouse and when done, click any other mouse button.
            with Scrollable(
                width=200.0,
                height=100.0,
                scroller_x_id=add_scroller(),
                scroller_y_id=add_scroller(),
                auto_scroll=True,
                style_id=scr_style
                ):

                # When the x scroller is added, a container
                # is needed with some padding so that the
                # last line shows or add a space, as done here.
                for _ in range(100):
                    add_text(content="A lot of text")

                add_text(content=("Some styling added, ")*10,
                        wrapping_none=True)
                add_space(height=10)

start_session()
