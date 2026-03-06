from imports import *

def change_alignment(btn_id):
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.AlignX, 
            value=AlignX.Left)
    
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.AlignY, 
            value=AlignY.Top)
    
    
# Moves the text to the center position
def recenter(btn_id):
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.Centered, 
            value=True)
    
    
# Moves text off center because padding on the left side
# padding = [top, right, bottom, left]
def padding(btn_id):
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.Padding, 
            value=[0.0, 0.0, 0.0, 50.0])  
    
# change container width
def width(btn_id):
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.Width, 
            value=300.0)
    
    
# change container height
def height(btn_id):
    update_widget(
            wid=cont2, 
            param=IpgContainerParam.Height, 
            value=300.0)
 

# Add the styling container widget
cont_style = add_container_style(
                background_color=IpgColor.GRAY)

# Add the windows
with Window(id="main", title="Container Styling",
            size=(600, 600),  center=True):

    # add a container to hold the demo container 
    # in the middle of the window
    with Container(id="cont1",width_fill=True,
            height_fill=True,center=True):

        # Add a column to hold the widgets
        # the column has an transparent background
        # so the container style shows through
        with Column():

            # add the container to work on
            with Container(width=100.0, height=50.0,
                    style_id=cont_style, center=True) as cont2:

                add_text(content="Some Text")

            # Add a button the change the alignment 
            add_button(label="Change Alignment",
                    on_press=change_alignment)

            # Add a button recenter the text 
            add_button(label="Recenter",
                    on_press=recenter)

            # Add a button add padding of the contained items
            add_button(label="Padding",
                    on_press=padding)

            # Add a button change the container width
            add_button(label="Width",
                    on_press=width)

            # Add a button change the container height
            add_button(label="Height",
                    on_press=height)

# last thing is to start the session
start_session()
