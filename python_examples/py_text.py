from imports import *

def wrapping_selected(radio_id: int, data: list[int, str]):
    value = TextWrapping.Word  # default
    match data[0]:
        case 0: value = TextWrapping.TextNone
        case 1: value = TextWrapping.Word
        case 2: value = TextWrapping.Glyph
        case 3: value = TextWrapping.WordOrGlyph
        
    update_widget(wid=txt_id, param=IpgTextParam.TextWrapping, value=value)

text_width=245.0
def text_widths(radio_id: int, data: list[int, str]):
    match data[0]:
        case 0:  # shrink (default)
            update_widget(wid=txt_id, param=IpgTextParam.Width, value=None)
            update_widget(wid=txt_id, param=IpgTextParam.WidthFill, value=None)
        case 1:  # fill
            update_widget(wid=txt_id, param=IpgTextParam.Width, value=None)
            update_widget(wid=txt_id, param=IpgTextParam.WidthFill, value=True)
        case 2:  # fixed
            update_widget(wid=txt_id, param=IpgTextParam.Width, value=text_width)
            update_widget(wid=txt_id, param=IpgTextParam.WidthFill, value=None)


def text_alignment(radio_id: int, width_type: int):
    match width_type:
        case 0: 
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopLeft, value=True)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopCenter, value=False)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopRight, value=False)
        case 1:
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopLeft, value=False)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopCenter, value=True)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopRight, value=False)
        case 2:
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopLeft, value=False)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopCenter, value=False)
            update_widget(wid=txt_id, param=IpgTextParam.AlignTopRight, value=True)
            
            

# Add a window
with Window(
        id="main", 
        title="Text Demo",
        size=(600, 600),  
        center=True, 
        # debug=True
        ):
 
    with Row(width_fill=True, height_fill=True):
        # Add a column to hold the widgets
        with Column(width=250.0, height_fill=True, 
                    spacing=10.0, padding=[10.0]):

            add_text(content="Text Size")
                
            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):
                    add_text(content="Text Size (default)")
                    add_text(content="Text Size: 20", size=20.0)
                    add_text(content="Text Size: 10", size=10.0)
                        
                
                            
            with Column(
                width_fill=True, 
                height_fill=True,
                # padding=[10.0],
                spacing=20.0):
            
                add_text(content="Text Line Height:\nRelative value",
                        align_center_left=True)

                # add container for the background
                with Container(width_fill=True, height=200.0,
                            style_std=IpgContainerStyleStd.BorderedBox):
                    
                    # add the column to hold the widgets
                    with Column(spacing=10.0, padding=[10.0], 
                                width_fill=True, height_fill=True):
                        
                        # adding a container with outline to show how the 
                        # line height works
                        with Container(style_std=IpgContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height (default=1.3)")
                        with Container(style_std=IpgContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height 5", line_height=5.0)
                        with Container(style_std=IpgContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height 2", line_height=2.0)
                
        with Column(width_fill=True, height_fill=True, 
                    spacing=10.0, padding=[20.0]):
            
            add_text(content="Text Wrapping, Width and Alignment:\n Dependent on each other")
            
            # add container for the background
            with Container(width_fill=True, height=120.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                with Row(spacing=10.0):
                    with Column():
                        add_text(content="Width types")
                        add_radio(labels=["shrink (default)", "fill", "fixed"],
                                on_select=text_widths,
                                radio_spacing=10.0, 
                                selected_index=2)
                    with Column():
                        add_text(content="Alignment")
                        add_radio(labels=["left", "center", "right"],
                                on_select=text_widths,
                                radio_spacing=10.0, 
                                selected_index=0)
            
            # add container for the background
            with Container(width_fill=True, height=275.0,
                        style_std=IpgContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], 
                            width_fill=True, height_fill=True):
                    add_text(content="Select wrapping type to show the effect")
                    add_radio(labels=[
                            "TextNone — no wrapping; text overflows",
                            "Word — wrap at word boundaries; (default)",
                            "Glyph — wrap at any character",
                            "WordOrGlyph — try word then glyph"],
                            on_select=wrapping_selected,
                            radio_spacing=10.0, selected_index=1)

            # add container for the background and width sizing
            with Container(width=240.0, height=100.0,
                    style_std=IpgContainerStyleStd.BorderedBox) as txt_cont:

                    txt_id = add_text(content="This is some text that will show wrapping when the radio button is selected\n" + 
                                      "ThisisalongwordthatwillshowWordOrGlyph")
            
            
                


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
