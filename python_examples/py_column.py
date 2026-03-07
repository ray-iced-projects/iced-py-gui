from imports import *

# Column and add_column() Demo
# A column adds widgets vertically

# NOTE: Toggle the debug at the top of the window
# to show the outline of the columns and widgets.

# A function to toggle the the debug
# or outline mode
def toggle_debug(tog_id: int, value: bool, wnd_id: int):
    update_widget(wid=wnd_id, param=IpgWindowParam.Debug, value=value)



# Add the window
with Window(
    title="Column", 
    size=(600, 600),  
    center=True) as wnd:

    print(wnd)

    # Need a column and row to hold the widget
    with Column(spacing=20.0, padding=[20.0], width_fill=True, height_fill=True):
        
        add_toggler(label="Toggle to set and unset the Window debug mode", toggled=toggle_debug, user_data=wnd)
        
        add_text(content="Spacing of 10.0 and 20.0")
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            # We use add_column here to show the alternate version
            # We add two text widgets to each column below to demonstrate spacing
            add_column(id="col1", spacing=10.0)
            add_text(parent_id="col1", content="Text in Column 1")
            add_text(parent_id="col1", content="Text in Column 1")
            
            add_column(id="col2", spacing=20.0)
            add_text(parent_id="col2", content="Text in Column 2")
            add_text(parent_id="col2", content="Text in Column 2")

        add_text(content="Padding of [all] and [top, right, botton, left]\n" +
                "Note the space around the Column outline", 
                align_x=AlignX.Left)
        
        with Row(width_fill=True, height=100.0, spacing=20.0):
            add_column(id="col3", padding=[20.0])
            add_text(parent_id="col3", content="Text in Column 3")
            add_text(parent_id="col3", content="Text in Column 3")
            
            add_column(id="col4", padding=[20.0, 0.0, 20.0, 0.0])
            add_text(parent_id="col4", content="Text in Column 4")
            add_text(parent_id="col4", content="Text in Column 4")
        
        add_text(content="Alignment parameter = align\n" +
                 "values = Align.Start(default), Align.Center, Align.End\n" +
                 "NOTE the empty space(s) beside each text group",
                 align_x=AlignX.Left)
        
        with Row(width_fill=True, height=75.0, spacing=20.0):
            add_column(id="col5", align_x=AlignX.Left, width=150.0)
            add_text(parent_id="col5", content="Text in Column 5")
            add_text(parent_id="col5", content="Text in Column 5")
            
            add_column(id="col6", align_x=AlignX.Center, width=150.0)
            add_text(parent_id="col6", content="Text in Column 6")
            add_text(parent_id="col6", content="Text in Column 6")
            
            add_column(id="col7", align_x=AlignX.Right, width=150.0)
            add_text(parent_id="col7", content="Text in Column 7")
            add_text(parent_id="col7", content="Text in Column 7")

# last thing is to start the session
start_session()