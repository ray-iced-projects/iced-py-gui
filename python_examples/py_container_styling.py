from imports import *

# To change the style of the container, 
# use the style id, not the container id.
def change_container_styling(btn_id):
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.BackgroundIpgColor, 
            value=IpgColor.RED)
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.BorderIpgColor, 
            value=IpgColor.LIGHT_SALMON)
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.BorderRadius, 
            value=[5.0])
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.BorderWidth, 
            value=5.0)
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.ShadowBlurRadius, 
            value=10.0)
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.ShadowIpgColor, 
            value=IpgColor.RED)
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.ShadowOffsetXY, 
            value=[0.0, 0.0])
    
    update_widget(
            wid=cont_style, 
            param=IpgContainerStyleParam.TextIpgColor, 
            value=IpgColor.WHITE)


# Add the styling container widget
cont_style = add_container_style(
                    background_color=IpgColor.AQUA,
                    border_color=IpgColor.BLUE,
                    border_radius=[10.0],
                    border_width=10.0,
                    shadow_color=IpgColor.YELLOW,
                    shadow_blur_radius=20.0,
                    shadow_offset_xy=[8.0, 8.0],
                    text_color=IpgColor.BLACK)

# Add the windows
add_window(
    id="main", 
    title="Container Styling", 
    size=(500, 600),  
    center=True)

# add a container to hold the demo container 
# in the middle of the window
add_container(
    window_id="main",
    id="cont1",
    width_fill=True,
    height_fill=True,
    center=True)

# add the container to work on
with Container(
    window_id="main",
    id="cont2",
    parent_id="cont1",
    width=200.0, 
    height=200.0,
    style_id=cont_style,
    center=True):

    # Add a column to hold the widgets
    # the column has an transparent background
    # so the container style shows through
    add_column(
        window_id="main",
        id="col",
        parent_id="cont2",
        spacing=20.0,
        padding=[20.0])

    # Add some text.  Since this text is not styled
    # it would be a bit hard to see because the container
    # will attempt to default style the text but
    # won't always work best, So you can either
    # style the text or use the container text_color
    # to style all of the text in the container.
    # This text styling will override the container 
    # text color
    add_text(
        parent_id="col",
        content="Some Text")

    # Add a button the change the background color 
    # or any of the style settings
    add_button(
        parent_id="col",
        label="Press to\nchange styling",
        text_align_x=IpgAlignmentX.Center,
        on_press=change_container_styling)


# last thing is to start the session
start_session()