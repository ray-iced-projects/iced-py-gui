from icedpygui import Window, Container, Column, ToolTip, \
    add_container_style, add_text, add_card, add_checkbox, \
    start_session, IpgToolTipParam, IpgToolTipPosition, \
    IpgContainerStyleStd, Color



cont_style = add_container_style(background_color=Color.AQUA)

    
# Add a window first
with Window(title="TooTip Demo", center=True):

    # Add a container to center the widgets in the middle
    with Container(fill=True, align_center=True):
        with Column(spacing=50.0):
            
            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                style_id= cont_style):

                add_text(content="Some text with a tooltip with custom background style")
                
            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                position=IpgToolTipPosition.Right,
                style_std= IpgContainerStyleStd.BorderedBox):

                add_text(content="Some text with a tooltip with standard style")    
                
            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                delay_sec=1):

                add_text(content="Some text with a tooltip with no style and delayed 1 sec") 
            
            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5):
                
                with Container(width=200.0, height=50.0, style_std=IpgContainerStyleStd.BorderedBox):
                    add_text(content="This container has a tooltip")
                    
            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5):
                
                add_checkbox(label="Chexbox with tooltip")
                
            with ToolTip(padding=5.0, gap=5):
                
                add_card(head="Card with head", body="Some info", padding=[5.0], width=200.0)

                add_text(content="Some text with a tooltip that is a widget")
                
                # Uncomment the add_text below to see a warning about using too many widgets or containers.
                # A ToolTip will only use two items or one item if the text parameter is used.
                
                # add_text(content="Some text with a tooltip that is a widget or container container widgets")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
