from icedpygui import Window, Column, Container, Row, start_session, \
    add_text, ContainerStyleStd

# Container and add_container() Demo
# A container has only one widget and
# can control the position better along 
# with having styling for background and 
# style variations.  See py_example_styling.py


# Add the window
with Window(
    title="Column", 
    size=(600, 600),  
    center=True) as wnd:

    # Need a column and row to hold the widget
    with Column(spacing=20.0, padding=[20.0], width_fill=True, height_fill=True):
        
        add_text(content="Alignments: booleans")
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(align_top_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_left")
            
            with Container(align_top_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_center")

            with Container(align_top_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_right")

        with Row(width_fill=True, height=75.0, spacing=20.0):
            
            with Container(align_center_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center_left")
            
            with Container(align_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center")

            with Container(align_center_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center_right")
            
        with Row(width_fill=True, height=75.0, spacing=20.0):    
            
            with Container(align_bottom_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_left")
            
            with Container(align_bottom_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_center")
            
            with Container(align_bottom_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_right")
            
        add_text(content="Padding padding=[all] or padding=[top, right, bottom, left]")
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(padding=[20.0],
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="padding all")
            
            with Container(padding=[0.0, 20.0, 0.0, 20.0],
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="padding left and right")
            
            
# last thing is to start the session
start_session()
