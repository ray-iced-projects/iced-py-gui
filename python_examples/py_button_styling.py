from icedpygui import Window, Column, Row, start_session, \
    add_button, add_button_style, ButtonStyleStd, Arrow, \
    add_text, Color

# The style can be add anywhere as long as the id is global enough
# or if your are using a class just equate it class header
yellow = add_button_style(background_color=Color.LIGHT_YELLOW)

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
                style_std=ButtonStyleStd.Primary)
            
            add_button(
                label="Secondary",
                padding=[5.0],
                style_std=ButtonStyleStd.Secondary)
             
            add_button(
                label="Subtle",
                padding=[5.0],
                style_std=ButtonStyleStd.Subtle)

            add_button(
                label="Success",
                padding=[5.0],
                style_std=ButtonStyleStd.Success)

            add_button(
                label="Danger",
                padding=[5.0],
                style_std=ButtonStyleStd.Danger)

            add_button(
                label="Warning",
                padding=[5.0], 
                style_std=ButtonStyleStd.Warning)  
                 
        with Row(spacing=20.0):
            
            add_button(
                label="Text Button", 
                style_std=ButtonStyleStd.Text)
            
            
            add_button(
                padding=[5.0], 
                style_arrow=Arrow.ArrowRight)

            add_button(
                label="Custom Color",
                padding=[5.0],
                style_id=yellow)
        
        add_text(content="Button Border & Radius Options")
         
        with Row(spacing=20.0):
            
            # These style were separated for this demo
            # but can be used with all the style variants
            # in one style id.
            border = add_button_style(
                border_color=Color.ALICE_BLUE,
                border_width=2.0)
            
            radius = add_button_style(
                border_color=Color.ALICE_BLUE,
                border_width=2.0,
                border_radius=[5.0])
            
            add_button(
                label="Border Width = 2",
                padding=[5.0],
                style_id=border)

            add_button(
                label="Border Radius = [5]",
                padding=[5.0],
                style_id=radius)
        
        add_text(content="Button Border Shadow Options")
        
        with Row(spacing=20.0):
            
            shadow = add_button_style(
                shadow_color=Color.ALICE_BLUE,
                shadow_blur_radius=10,)
            
            shadow_offset_x = add_button_style(
                shadow_color=Color.ALICE_BLUE,
                shadow_blur_radius=10,
                shadow_offset_xy=[10, 0],)
            
            shadow_offset_y = add_button_style(
                shadow_color=Color.ALICE_BLUE,
                shadow_blur_radius=10,
                shadow_offset_xy=[0, 10],)
            
            shadow_offset_xy = add_button_style(
                shadow_color=Color.ALICE_BLUE,
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
