from imports import *



# callback to print the button id
def print_id(id):
    print(f"Button id {id} pressed")
    

#  First add a window
with Window(id="main", title="App", 
            size=(800, 600),center=True):
    
    # To get the centering to work, the container width/height
    # must be greater than the container widgets
    # or the container shrinks to wrap the widgets.
    # In this case, only width needed to be expanded
    # but in other cases if could be height or both.
    with Container(width_fill=True, center=True):
        
        with Column(spacing=20.0, padding=[20.0], 
                    align_x=IpgAlignment.Center):
            
            add_text(content="Button with padding variations")
           
            with Row(spacing=20.0):

                # button with only a label parameter all other parameters are defaults
                add_button(label="No Padding")

                # padding
                add_button(
                    label="All sides Padding",
                    padding=[10.0])
                
                # padding 
                add_button(
                    label="Top/Bottom Padding",
                    padding=[10.0, 0.0, 10.0, 0.0]) # top & botton
                
                # padding 
                add_button(
                    label="Left/Right Padding",
                    padding=[0.0, 10.0, 0.0, 10.0]) # left & right
            
            add_text(content="Button label Alignment")
                
            with Row(spacing=20.0):
                
                add_button(
                    label="Align Top-left default",
                    width=200.0,
                    height=50.0
                )
                
                add_button(
                    label="Align Top",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Center,
                    text_align_y=AlignY.Top,
                )
                
                
                

start_session()