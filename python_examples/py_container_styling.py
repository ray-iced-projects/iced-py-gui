from icedpygui import Window, Column, Container, Row, start_session, \
    add_container_style, ContainerStyleStd, Color, add_text

# Add the styling container widget
cont_style = add_container_style(
                background_color=Color.AQUA,
                border_color=Color.BLUE,
                border_radius=[10.0],
                border_width=10.0,
                shadow_color=Color.YELLOW,
                shadow_blur_radius=20.0,
                shadow_offset_xy=[8.0, 8.0],
                text_color=Color.BLACK)

# Add the windows
with Window(title="Container Styling", 
            size=(500, 600), center=True):
    
    # add a ccolumn to hold the containers
    with Column(spacing=50.0, align_center=True, padding=[20.0], width_fill=True):
    
        with Container(
            width=200.0,
            height=200.0,
            align_center=True,
            style_id=cont_style):
            
            add_text(content="Custom Style")

        # add the container to work on
        with Container(
            width=200.0, 
            height=200.0,
            align_center=True,
            style_std=ContainerStyleStd.RoundedBox):

            add_text(content="Std Style: RoundedBox")


# last thing is to start the session
start_session()