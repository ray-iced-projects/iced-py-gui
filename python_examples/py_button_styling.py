from imports import *

# The style can be add anywhere as long as the id is global enough
# or if your are using a class just equate it class header
yellow = add_button_style(background_color=IpgColor.LIGHT_YELLOW)

# Add the windows
with Window( 
        title="Button Styling", 
        size=(650, 600),  
        center=True):

    # Add a column to hold multiple widgets, vertically.
    with Column(spacing=20.0, padding=[20.0]):

        add_text(content="Buttons with style standard and custom types")
        
        # Add a row to hold widgets, horizontally.
        with Row(spacing=20.0):

            add_button(
                label="Primary=Default",
                padding=[5.0],
                style_std=IpgButtonStyleStd.Primary)
            
            add_button(
                label="Secondary",
                padding=[5.0],
                style_std=IpgButtonStyleStd.Secondary)
             
            add_button(
                label="Subtle",
                padding=[5.0],
                style_std=IpgButtonStyleStd.Subtle)

            add_button(
                label="Success",
                padding=[5.0],
                style_std=IpgButtonStyleStd.Success)

            add_button(
                label="Danger",
                padding=[5.0],
                style_std=IpgButtonStyleStd.Danger)

            add_button(
                label="Warning",
                padding=[5.0], 
                style_std=IpgButtonStyleStd.Warning)  
                 
        with Row(spacing=20.0):
            
            add_button(
                label="Text", 
                style_std=IpgButtonStyleStd.Text)
            
            
            add_button(
                padding=[5.0], 
                style_arrow=IpgArrow.ArrowRight)

            add_button(
                label="Custom Color",
                padding=[5.0],
                style_id=yellow)
        
        add_text(content="Button Border Radius Options")
         
        with Row(spacing=20.0):
            
            # These style were separated for this demo
            # but can be used with all the style variants
            # in one style id.
            border = add_button_style(
                border_color=IpgColor.ALICE_BLUE,
                border_width=2.0)
            
            radius = add_button_style(
                border_color=IpgColor.ALICE_BLUE,
                border_width=2.0,
                border_radius=[5.0])
            
            add_button(
                label="Border Width",
                padding=[5.0],
                style_id=border)

            add_button(
                label="Border Radius",
                padding=[5.0],
                style_id=radius)
        
        add_text(content="Button Border Shadow Options")
        
        with Row(spacing=20.0):
            
            shadow = add_button_style(
                shadow_color=IpgColor.ALICE_BLUE,
                shadow_blur_radius=10,)
            
            shadow_offset_x = add_button_style(
                shadow_color=IpgColor.ALICE_BLUE,
                shadow_blur_radius=10,
                shadow_offset_xy=[10, 0],)
            
            shadow_offset_y = add_button_style(
                shadow_color=IpgColor.ALICE_BLUE,
                shadow_blur_radius=10,
                shadow_offset_xy=[0, 10],)
            
            shadow_offset_xy = add_button_style(
                shadow_color=IpgColor.ALICE_BLUE,
                shadow_blur_radius=10,
                shadow_offset_xy=[10, 10],)

            add_button(
                label="Shadow",
                padding=[5],
                style_id=shadow,)
                
            add_button(
                label="Shadow offset x",
                padding=[5],
                style_id=shadow_offset_x,)
       
            add_button(
                label="Shadow offset y",
                padding=[5],
                style_id=shadow_offset_y,)

            add_button(
                label="Shadow offset xy",
                padding=[5],
                style_id=shadow_offset_xy,)


start_session()
