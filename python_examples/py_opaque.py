from icedpygui import Window, Container, start_session, \
    Opaque, Stack, add_button, IpgContainerStyleStd

def no_press(_btn_id: int):
    print("This should not happen")
          
def on_press(_btn_id: int):
    print("You press top button")


with Window(center=True):
    with Container(fill=True, align_center=True):
        with Stack():
            with Container(width=200, height=100, align_top_center=True, style_std=IpgContainerStyleStd.BorderedBox):
                add_button(label="I'm on the bottom, so you can't press me", on_press=no_press)
            # The opaque in this case needed to include the container too because
            # the container is stacked on top of the previous container which
            # holds the bottom button.  Therefore, if you just put the button
            # in the Opaque, then the Opaque size shrinks around the button and
            # the other button is revealed through this second container.
            with Opaque():
                with Container(width=200, height=100, align_bottom_center=True):
                    add_button(label="I'm on the top", on_press=on_press)
            
            
start_session()