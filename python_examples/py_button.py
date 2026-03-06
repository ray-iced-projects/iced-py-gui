from imports import *



# callback to print the button id
# or do anything else you like
def print_id(id):
    print(f"Button id {id} pressed")
    
def print_user_data(id, some_data):
    print(f"User data {some_data}")
    

#  First add a window
with Window(id="main", title="App", 
            size=(800, 600),center=True):
    
    # Need a Scrollable since long content
    with Scrollable(width_fill=True):
        
        # Need a Container to hold everything, in this case a Column
        with Column(spacing=20.0, padding=[20.0]):
            
            add_text(content="Button with padding variations [Top, Right, Bottom, Left] [all]")
            
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
            
            add_text(content="Button label Alignment, Center X and Center Y defaults")
                
            with Row(spacing=20.0):
                
                add_button(
                    label="Align Center default",
                    width=200.0,
                    height=50.0
                )
                
                add_button(
                    label="Align Top",
                    width=200.0,
                    height=50.0,
                    text_align_y=AlignY.Top,
                )
                
                add_button(
                    label="Align Bottom",
                    width=200.0,
                    height=50.0,
                    text_align_y=AlignY.Bottom,
                )
                
            with Row(spacing=20.0):
                
                add_button(
                    label="Align Left Top",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Left,
                    text_align_y=AlignY.Top,
                )
                
                add_button(
                    label="Align left Center",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Left,
                )
                
                add_button(
                    label="Align Left Bottom",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Left,
                    text_align_y=AlignY.Bottom,
                )
                
            with Row(spacing=20.0):
                
                add_button(
                    label="Align Right Top",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Right,
                    text_align_y=AlignY.Top,
                )
                
                add_button(
                    label="Align Right Center",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Right,
                )
                
                add_button(
                    label="Align Right Bottom",
                    width=200.0,
                    height=50.0,
                    text_align_x=AlignX.Right,
                    text_align_y=AlignY.Bottom,
                )
                
            add_text(content="Button label text size")
                
            with Row(spacing=20.0):
                
                add_button(label="Size default")
                
                add_button(label="Size", text_size=20.0)
                
                
            add_text(content="Button Width Height")
                
            # Container needed to show the outline
            with Container(width_fill=True, height=150, style_std=IpgContainerStyleStd.BorderedBox):
                # Need a coloumn to hold the row and then the lone button
                with Column(width_fill=True, height=200.0, spacing=10.0):
                    # Need row for the first two buttons
                    with Row(spacing=20.0, width_fill=True):
                        
                        add_button(label="Width Height Default=Shrink")
                        
                        add_button(
                            label="Width Height Values",
                            width=200.0,
                            height=50.0)
                
                    add_button(label="width height Fill - fills with the Column values or whatever is left if other widgets",
                                width_fill=True,
                                height_fill=True)
            
            add_text(content="Button Clipping")
            
            # Need row to hold 2 buttons  
            with Row(spacing=20.0, width_fill=True):
                
                add_button(label="The text on this button will wrap", 
                        width=200.0)
                
                add_button(label="The text on this button is clipped....",
                        width=200.0,
                        clip=True)
                
            add_text(content="Button Callbacks")
            
            # Need column to hold the buttons  
            with Column(spacing=20.0, width_fill=True, height=200.0):
                
                add_button(label="Press me to run the callback to print id",
                           padding=[10.0],
                           on_press=print_id)
                
                add_button(label="Press me to run the callback to print user data",
                           padding=[10.0],
                           on_press=print_user_data,
                           user_data="Some data")
                
                add_button(label="Press me to run the callback to print more user data",
                           padding=[10.0],
                           on_press=print_user_data,
                           user_data=[10.0, 20.0])

start_session()