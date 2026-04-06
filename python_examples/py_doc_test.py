from icedpygui import Window, Column, Container, IpgContainerStyleStd,\
    add_container_style, add_text, Color, start_session

style = add_container_style(
            background_color=Color.AQUA,
            border_color=Color.BLUE,
            border_radius=[10.0],
            border_width=5.0,
            shadow_color=Color.YELLOW)

with Window(title="Container Styling", center=True):
    with Column(spacing=20.0, padding=[20.0], align_center=True, width_fill=True):
        
        with Container(align_center=True, width=400.0, height=200.0, style_id=style):
            add_text(content="Some Container Custom Styling")
            
        with Container(align_center=True, width=400.0, height=200.0, 
                       style_std=IpgContainerStyleStd.BorderedBox):
            add_text(content="Some Container Standard Styling\n BorderedBox")

start_session()
