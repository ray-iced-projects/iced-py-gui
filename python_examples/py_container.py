from imports import *

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
        
        add_text(content="Alignments: booleans", align_x=AlignX.Left)
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            # We use add_container here to show the alternate version
            # We add a text widget to each container below to demonstrate 
            # alignment
            add_container(id="cont1", align_top_left=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont1", content="top_left")
            
            add_container(id="cont2", align_top_center=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont2", content="top_center")

            add_container(id="cont3", align_top_right=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont3", content="top_right")

        with Row(width_fill=True, height=75.0, spacing=20.0):
            
            add_container(id="cont4", align_center_left=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont4", content="center_left")
            
            add_container(id="cont5", align_center=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont5", content="center")

            add_container(id="cont6", align_center_right=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont6", content="center_right")
            
        with Row(width_fill=True, height=75.0, spacing=20.0):    
            
            add_container(id="cont7", align_bottom_left=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont7", content="bottom_left")
            
            add_container(id="cont8", align_bottom_center=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont8", content="bottom_center")
            
            add_container(id="cont9", align_bottom_right=True,
                          width=175, height_fill=True,
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont9", content="bottom_right")
            
        add_text(content="Padding padding=[all] or padding=[top, right, bottom, left]", align_x=AlignX.Left)
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            add_container(id="cont10", padding=[20.0],
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont10", content="padding all")
            
            add_container(id="cont11", padding=[0.0, 20.0, 0.0, 20.0],
                          style_std=IpgContainerStyleStd.BorderedBox)
            add_text(parent_id="cont11", content="padding left and right")
            
            
# last thing is to start the session
start_session()
