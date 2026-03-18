from imports import *

# Column and add_column() Demo
# A column adds widgets vertically


# Add the window
with Window(
    title="Column", 
    size=(600, 600),  
    center=True) as wnd:

    # Need a column and row to hold the widget
    with Column(spacing=20.0, padding=[20.0], fill=True):
        
        add_text(content="Spacing of 10.0 and 20.0")
        
        with Row(width_fill=True, height=100.0, spacing=20.0):
            # We use add_column here to show the alternate version
            # We add two text widgets to each column below to demonstrate spacing
            # The container is just for showing a little background for highlighting
            with Container(width=200.0, height_fill=True,
                    style_std=IpgContainerStyleStd.BorderedBox):
                with Column(spacing=10.0):
                    add_text(content="Text in Column 1")
                    add_text(content="Text in Column 1")
            
            with Container(width=200.0, height_fill=True,
                    style_std=IpgContainerStyleStd.BorderedBox):    
                with Column(spacing=20.0):
                    add_text(content="Text in Column 2")
                    add_text(content="Text in Column 2")

        add_text(content="Padding of [all] and [top, right, botton, left]\n" +
                "Note the space around the Column outline")
        
        
        with Row(width_fill=True, height=100.0, spacing=20.0):
            with Container(style_std=IpgContainerStyleStd.BorderedBox):
                with Column(padding=[20.0]):
                    add_text(content="Padding all")
                    add_text(content="Padding all")
            
            with Container(style_std=IpgContainerStyleStd.BorderedBox):    
                with Column(padding=[20.0, 0.0, 20.0, 0.0]):
                    add_text(content="Padding Top/Bottom")
                    add_text(content="Padding Top/Bottom")

        add_text(content="Alignment parameter = align\n" +
                 "values = align_left(default), align_center, align_right\n" +
                 "NOTE the empty space(s) beside each text group")
        
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(style_std=IpgContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_left=True):
                    add_text(content="Left")
                    add_text(content="Left")
            
            with Container(style_std=IpgContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_center=True):
                    add_text(content="Center")
                    add_text(content="Center")
            
            with Container(style_std=IpgContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_right=True):
                    add_text(content="Right")
                    add_text(content="Right")

# last thing is to start the session
start_session()
