from icedpygui import (
    add_window, add_container, add_column, add_row, add_button, start_session,
    IpgColor, IpgWindowTheme, IpgAlignment,
    IpgButtonStyleStandard, IpgButtonArrow, add_button_style,
    IpgHorizontalAlignment, IpgVerticalAlignment
)



def on_press(btn_id):
    print("button pressed")

def add_buttons(window: str):
    # Add a container to hold everything aligning all in the center
    add_container(
            window_id=window, 
            container_id="cont", 
            width_fill=True, 
            height_fill=True)
    
    # Add a column to hold multiple widgets, vertically.
    add_column(
            window_id=window, 
            container_id="col", 
            parent_id="cont")

    # Add a row to hold widgets, horizontally.
    add_row(
            window_id=window, 
            container_id="row_btn", 
            parent_id="col",
            align=IpgAlignment.Center)

    # Add buttons
    add_button(
            parent_id="row_btn", 
            label="Default", 
            on_press=on_press)

    add_button(
            parent_id="row_btn", 
            label="Primary", 
            on_press=on_press,
            style_standard=IpgButtonStyleStandard.Primary)

    add_button(
            parent_id="row_btn", 
            label="Success", 
            on_press=on_press,
            style_standard=IpgButtonStyleStandard.Success)

    add_button(
            parent_id="row_btn", 
            label="Danger", 
            on_press=on_press,
            style_standard=IpgButtonStyleStandard.Danger)

    add_button(
            parent_id="row_btn", 
            label="Text", 
            on_press=on_press,
            style_standard=IpgButtonStyleStandard.Text)
    
    add_button(
            parent_id="row_btn", 
            label="",
            on_press=on_press,
            style_arrow=IpgButtonArrow.ArrowRight)

    add_row(
            window_id=window, 
            container_id="row_btn2", 
            parent_id="col",
            align=IpgAlignment.Center)

    dodger = add_button_style(
                background_color=IpgColor.DODGER_BLUE)

    add_button(
            parent_id="row_btn2", 
            label="Custom Base Only Defined",
            style_id=dodger)

    custom = add_button_style(
                    background_color=IpgColor.DODGER_BLUE,
                    background_color_hovered=IpgColor.BLUE,
                    border_color=IpgColor.DARK_GOLDEN_ROD,
                    shadow_color=IpgColor.DARK_ORANGE,
                    text_color=IpgColor.BLACK,
                    border_radius=[12.0], 
                    border_width=5.0,
                    shadow_offset_x=0.0, 
                    shadow_offset_y=0.0, 
                    shadow_blur_radius=15.0)
    
    add_button(
            parent_id="row_btn2", 
            label="All Colors Custom",
            style_id=custom)
    
    add_row(
            window_id=window, 
            container_id="row_btn3", 
            parent_id="col",
            align=IpgAlignment.Center)
    
    std_border = add_button_style(
                         border_color=IpgColor.GREEN,
                         border_radius=[12.0],
                         border_width=5.0,
                         shadow_color=IpgColor.DARK_GREEN,
                         shadow_blur_radius=10.0,
                         shadow_offset_x=5.0,
                         shadow_offset_y=5.0,
                         )
    
    add_button(
            parent_id="row_btn3", 
            label="Standard with Border and shadow",
            style_id=std_border,
            style_standard=IpgButtonStyleStandard.Success)
    
    if window == "main1":
        add_button(
                parent_id="col",
                label="Alignment = Center/Center",
                width=300.0,
                height=50.0)
        
        add_button(
                parent_id="col",
                label="Alignment = Left/Bottom",
                width=300.0,
                height=50.0,
                text_align_x=IpgHorizontalAlignment.Left,
                text_align_y=IpgVerticalAlignment.Bottom)
        
        add_button(
                parent_id="col",
                label="Alignment = Right/Top",
                width=300.0,
                height=50.0,
                text_align_x=IpgHorizontalAlignment.Right,
                text_align_y=IpgVerticalAlignment.Top)
        

# Add the windows
add_window(
        window_id="main1", 
        title="Button Styling", 
        width=500, 
        height=600,  
        pos_x=100, 
        pos_y=25)

add_window(
        window_id="main2", 
        title="Button Styling", 
        width=500, 
        height=600,  
        pos_x=650, 
        pos_y=25,
        theme=IpgWindowTheme.GruvboxLight)

add_buttons("main1")

add_buttons("main2")

start_session()
