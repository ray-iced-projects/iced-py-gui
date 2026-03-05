from imports import *


# A button demo for styling the buttons and using a simple callback
# In ths demo, the container were carefully aligned so that each button
# would go into the right window and container.  Not something you would
# usually do.  In many cases, you would have different subroutines
# that would add the various widgets.  Sometimes when subs are used, the 
# preceding container/window id might not be carried through so take
# a look at the container examples to see how this works.

def on_press(btn_id):
    print("button pressed")

def add_buttons(window: str):
    # Add a container to hold everything aligning all in the center
    with Container(window_id=window, 
            width_fill=True, 
            height_fill=True,
            center=True):
    
        # Add a column to hold multiple widgets, vertically.
        with Column(window_id=window,
                spacing=10.0,
                align_x=IpgAlignment.Center):

            # Add a row to hold widgets, horizontally.
            with Row(window_id=window,
                    spacing=5.0,
                    align_y=IpgAlignment.Center):

                # Add buttons
                add_button(
                        label="Default", 
                        on_press=on_press)

                add_button(
                        label="Primary", 
                        on_press=on_press,
                        style_standard=IpgButtonStyleStandard.Primary)

                add_button(
                        label="Success", 
                        on_press=on_press,
                        style_standard=IpgButtonStyleStandard.Success)

                add_button(
                        label="Danger", 
                        on_press=on_press,
                        style_standard=IpgButtonStyleStandard.Danger)

                add_button(
                        label="Text", 
                        on_press=on_press,
                        style_standard=IpgButtonStyleStandard.Text)
                
                add_button( 
                        label="",
                        on_press=on_press,
                        style_arrow=IpgArrow.ArrowRight)

            with Row(window_id=window,
                    align_y=IpgAlignment.Center):

                dodger = add_button_style(
                            background_color=IpgColor.DODGER_BLUE)

                add_button(
                        label="Custom Base Only Defined",
                        style_id=dodger)

                custom = add_button_style(
                                background_color=IpgColor.DODGER_BLUE,
                                border_color=IpgColor.DARK_GOLDEN_ROD,
                                shadow_color=IpgColor.DARK_ORANGE,
                                text_color=IpgColor.BLACK,
                                border_radius=[12.0], 
                                border_width=5.0,
                                shadow_offset_xy=(0.0, 0.0), 
                                shadow_blur_radius=15.0)
                
                add_button(
                        label="All Colors Custom",
                        style_id=custom)
    
            with Row(window_id=window,
                    align_y=IpgAlignment.Center):

                    # these styles can be placed anywhere since they don't
                    # use a parent_id.  The equated if just has to be 
                    # global enough to be seen by the widget.
                    std_border = add_button_style(
                                        border_color=IpgColor.GREEN,
                                        border_radius=[12.0],
                                        border_width=5.0,
                                        shadow_color=IpgColor.DARK_GREEN,
                                        shadow_blur_radius=10.0,
                                        shadow_offset_xy=(5.0, 5.0),
                                        )
                    
                    add_button(
                            label="Standard with Border and shadow",
                            style_id=std_border,
                            style_standard=IpgButtonStyleStandard.Success)
                    
        
            # add additional buttons to add only the main1 window
            
            if window == "main1":
                add_button(
                        label="Alignment = Center/Center",
                        width=300.0,
                        height=50.0,
                        text_align_x=IpgAlignmentX.Center,
                        text_align_y=IpgAlignmentY.Center)
                
                add_button(
                        label="Alignment = Left/Bottom",
                        width=300.0,
                        height=50.0,
                        text_align_x=IpgAlignmentX.Left,
                        text_align_y=IpgAlignmentY.Bottom)
                
                add_button(
                        label="Alignment = Right/Top",
                        width=300.0,
                        height=50.0,
                        text_align_x=IpgAlignmentX.Right,
                        text_align_y=IpgAlignmentY.Top)
   
# Add the windows
add_window(
        id="main1", 
        title="Button Styling", 
        size=(500, 600),  
        position=(100, 25))

add_window(
        id="main2", 
        title="Button Styling",  
        size=(500, 600),  
        position=(600, 25),
        theme=IpgWindowTheme.GruvboxLight)

add_buttons("main1")

add_buttons("main2")

start_session()
