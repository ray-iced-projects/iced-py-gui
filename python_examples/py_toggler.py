from imports import *

# Callback from toggler
def toggled(tog_id, is_toggled):
    print(tog_id, is_toggled)

wrapping_index=1
def wrapping_selected(radio_id: int, data: list[int, str]):
    global wrapping_index
    wrapping_index = data[0]
    value = TextWrapping.Word  # default
    # Since 
    match data[0]:
        case 0: value = TextWrapping.TextNone
        case 1: value = TextWrapping.Word
        case 2: value = TextWrapping.Glyph
        case 3: value = TextWrapping.WordOrGlyph
        
    update_widget(wid=tog, param=IpgTogglerParam.TextWrapping, value=value)


    

wnd_width = 700.0

# Add the window
with Window(
    id="main", 
    title="Toggler Demo",
    size=(wnd_width, 700),  
    center=True):

    # Add a main row to hold two columns
    with Row(width_fill=True, height_fill=True):
        with Column(
            width = wnd_width/2.0, 
            height_fill=True,
            padding=[20.0],
            spacing=20.0):

            add_text(content="Label alignment:\nvalid if width > text width",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    # text alignment only works if width > length of text
                    add_toggler(label="Text left (default)", width=200.0)
                    add_toggler(label="Text center", width=200.0, text_center=True)
                    add_toggler(label="Text right", width=200.0, text_right=True)
            
            add_text(content="Toggler Size",
                    align_center_left=True)
            
            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    add_toggler(label="Size (default)", width=200.0)
                    add_toggler(label="Size: 30", width=200.0, size=30.0)
                    add_toggler(label="Size: 10", width=200.0, size=10.0)
            
            add_text(content="Toggler Text Size",
                    align_center_left=True)
            
            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):
                    add_toggler(label="Text Size (default)")
                    add_toggler(label="Text Size: 20", text_size=20.0)
                    add_toggler(label="Text Size: 5", text_size=5.0)
                    
            add_text(content="Label spacing:",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    
                    add_toggler(label="Text spacing (default)")
                    add_toggler(label="Text spacing 0", spacing=0.0)
                    add_toggler(label="Text spacing 30", spacing=30.0)
                        
        with Column(
            width_fill=True, 
            height_fill=True,
            padding=[20.0],
            spacing=20.0):
        
            add_text(content="Label text Line Height:",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=175.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    
                    # adding a container with outline to show how the 
                    # line height works
                    with Container(style_std=IpgContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height (default=0)")
                    with Container(style_std=IpgContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height 5", text_line_height=5.0)
                    with Container(style_std=IpgContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height 2", text_line_height=2.0)
            
            # add container for the background
            with Container(width_fill=True, height=250.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    add_text(content="Select wrapping type then use slider(below) to show the effect")
                    add_radio(labels=[
                            "TextNone — no wrapping; text overflows",
                            "Word — wrap at word boundaries; (default)",
                            "Glyph — wrap at any character",
                            "WordOrGlyph — try word then glyph"],
                            on_select=wrapping_selected,
                            radio_spacing=10.0, selected_index=wrapping_index)

            # add container for the background and width sizing
            with Container(width=240.0, height=50.0,
                    style_std=IpgContainerStyleStd.BorderedBox) as tog_cont:

                    tog = add_toggler(label="This is some text that will show wrapping", width=250.0)
            
            
                           
                

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
