from icedpygui import Window, Column, Container, Row, start_session, \
    update_widget_params, TextParam, add_space, \
    ContainerStyleStd, add_radio, add_text, update_widget



def wrapping_selected(radio_id: int, data: list[int, str]):
    match data[0]:
        case 0: value = {
            update_widget_params(txt_id, {
                # reset all for default word
                TextParam.WrappingGlyph: None,
                TextParam.WrappingNone: None,
                TextParam.WrappingWordGlyph: None})
        }
        case 1: value = update_widget_params(txt_id, {
                # reset any ones that might be set
                TextParam.WrappingNone: None,
                TextParam.WrappingWordGlyph: None,
                # set the selected
                TextParam.WrappingGlyph: True,})
        case 2: value = update_widget_params(txt_id, {
                # reset any ones that might be set
                TextParam.WrappingGlyph: None,
                TextParam.WrappingWordGlyph: None,
                # set the selected
                TextParam.WrappingNone: True,})
        case 3: value = update_widget_params(txt_id, {
                # reset any that might be set
                TextParam.WrappingGlyph: None,
                TextParam.WrappingNone: None,
                # set the selected
                TextParam.WrappingWordGlyph: True})
        
    
def set_widths(_radio_id: int, data: list[int, str]):
    match data[0]:
        case 0:  # shrink (default)
            update_widget_params(set_txt_id, {
                # reset all for default
                TextParam.Width: None,
                TextParam.WidthFill: None})
        case 1:  # fill
            update_widget_params(set_txt_id, {
                # reset any ones that might be set
                TextParam.Width: None,
                # set the selected
                TextParam.WidthFill: True})
        case 2:  # fixed
            update_widget_params(set_txt_id, {
                # reset any ones that might be set
                TextParam.WidthFill: None,
                # set the selected
                TextParam.Width: 150.0})


def text_alignment(radio_id: int, align_type: list[int, str]):
    match align_type[1]:
        case "AlignBottomLeft":
            update_widget(align_txt_id, TextParam.AlignBottomLeft, True)
        case "AlignBottomCenter":
            update_widget(align_txt_id, TextParam.AlignBottomCenter, True)
        case "AlignBottomRight":
            update_widget(align_txt_id, TextParam.AlignBottomRight, True)
        case "AlignCenterLeft":
            update_widget(align_txt_id, TextParam.AlignCenterLeft, True)
        case "AlignCenter":
            update_widget(align_txt_id, TextParam.AlignCenter, True)
        case "AlignCenterRight":
            update_widget(align_txt_id, TextParam.AlignCenterRight, True)
        case "AlignTopLeft - default":
            update_widget(align_txt_id, TextParam.AlignTopLeft, True)
        case "AlignTopCenter":
            update_widget(align_txt_id, TextParam.AlignTopCenter, True)
        case "AlignTopRight":
            update_widget(align_txt_id, TextParam.AlignTopRight, True)


# Add a window
with Window(title="Text Demo", center=True):
 
    with Row(fill=True):
        # Add a column to hold the widgets
        with Column(width=250.0, height_fill=True, 
                    spacing=10.0, padding=[10.0]):

            add_text(content="Text Size")
                
            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0], fill=True):
                    add_text(content="Text Size (default)")
                    add_text(content="Text Size: 20", size=20.0)
                    add_text(content="Text Size: 10", size=10.0)
                                
            with Column(fill=True, spacing=20.0):
            
                add_text(content="Text Line Height:\nRelative value")

                # add container for the background
                with Container(width_fill=True, height=200.0,
                            style_std=ContainerStyleStd.BorderedBox):
                    
                    # add the column to hold the widgets
                    with Column(spacing=10.0, padding=[10.0], fill=True):
                        
                        # adding a container with outline to show how the 
                        # line height works
                        with Container(style_std=ContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height (default=1.3)")
                        with Container(style_std=ContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height 5", line_height=5.0)
                        with Container(style_std=ContainerStyleStd.RoundedBox):
                            add_text(content="Text Line Height 2", line_height=2.0)
                
        with Column(width=350, height_fill=True, spacing=10.0, padding=[20.0]):
            
            add_text(content="Text Wrapping, Width and Alignment:\n Select radio buttons to see the effect")
            
            # add column to hold the widgets
            with Column(fill=True, spacing=20.0):
                add_text(content="Width types")
                add_radio(labels=["shrink (default)", "fill- for text same as shrink", "fixed=100.0"],
                    on_select=set_widths,
                    radio_spacing=10.0, 
                    selected_index=0)
                
                with Container(width_fill=True, style_std=ContainerStyleStd.RoundedBox):
                    set_txt_id = add_text(content="Select radio to set my width.", width_fill=True)
                    
                add_text(content="Alignment")
                add_radio(labels=[
                    "AlignBottomLeft",
                    "AlignBottomCenter",
                    "AlignBottomRight",
                    "AlignCenterLeft", 
                    "AlignCenter",
                    "AlignCenterRight",
                    "AlignTopLeft - default",
                    "AlignTopCenter",
                    "AlignTopRight"],
                    on_select=text_alignment,
                    radio_spacing=10.0, 
                    selected_index=6)
            
                # used to highlight the text positon with a background color.
                with Container(width_fill=True, height=200, 
                               style_std=ContainerStyleStd.RoundedBox):
                    # for the alignment to work, it's content box must be bigger than
                    # the text, so fill=True to have the text box fill the container.
                    align_txt_id = add_text(content="Select radio to set alignment.", fill=True)

        # add column to hold the widgets
        with Column(spacing=30.0, padding=[10.0], fill=True):

            add_space(height=20)
            add_text(content="Select wrapping type to show the effect")
            add_radio(labels=[
                    "Word — wrap at word boundaries; (default)",
                    "Glyph — wrap at any character",
                    "None - No wrapping",
                    "WordOrGlyph — try word then glyph"],
                    on_select=wrapping_selected,
                    radio_spacing=10.0, 
                    selected_index=0)
            # add container for the background and width sizing
            with Container(width=240.0, height=150.0,
                    style_std=ContainerStyleStd.BorderedBox) as txt_cont:

                    txt_id = add_text(content="This is some text that will show wrapping when the radio button is selected\n\n" + 
                                        "ThisisalongwordthatwillshowWordOrGlyph")
            

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
