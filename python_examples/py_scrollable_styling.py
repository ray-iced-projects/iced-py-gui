from imports import *

add_window(
        id="main", 
        title="Scrollable - Styling",
    size=(600, 600),
        pos_centered=True)


# Add a container just to get some spacing from the top
add_container(
        window_id="main", 
        id="cont",
        width_fill=True, 
        height_fill=True)

# Let's add some styling for the scrollable before adding it.
# Let's style the bar first with a dark color
color = add_scrollable_style(
                background_color=IpgColor.GRAY,
                border_color=IpgColor.LIGHT_GREEN,
                border_width=4.0,
                border_radius=[8.0],
                scrollbar_color=IpgColor.LIGHT_BLUE,
                scroller_color=IpgColor.BLUE,
                scroller_color_hovered=IpgColor.GREEN,
                scroller_color_dragged=IpgColor.HOT_PINK,
                shadow_color=IpgColor.BEIGE,
                shadow_blur_radius=20.0,
                text_color=IpgColor.RED)

# Add the scrollable restricting the width so it can center and height 
# which is less than the length of the data in the column for scrolling.
scroll_id_1 = add_scrollable(
                    window_id="main", 
                    id="scroll",
                    parent_id="cont",
                    width=500, height=150.0,
                    style_id=color,
                    v_bar_width=20.0,
                    v_scroller_width=15.0,
                )

# Add the column for the data
add_column(
        window_id="main", 
        id="col",
        parent_id="scroll", 
        width_fill=True,
        align=IpgAlignment.Center)

#  Add some content to scroll
for i in range(0, 25):
    add_text(
            parent_id="col", 
            content="Scroll Me Up and Down! Scroll Me Up and Down!")


start_session()