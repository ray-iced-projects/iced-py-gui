from icedpygui import Window, Container, Column, \
    Card, CardParam, add_card, CardStyleStd, \
    add_button, ButtonParam, ButtonStyleStd, \
    update_widget, start_session



# The callback will minimizes the first card, the button at the bottom left will maximize it.
def minimize_card(card_id: int):
    update_widget(card_id, CardParam.IsOpen, False)
    update_widget(btn_id, ButtonParam.Show, True)

def maximize_card(btn_id: int):
    update_widget(card_id, CardParam.IsOpen, True)
    update_widget(btn_id, ButtonParam.Show, False)


# window added first
with Window(title="Card Demo", center=True):

    with Container(fill=True, align_center=True):
        # Add the column to hold the widgets
        # Even though only one widget shows, multiple widgets needed
        # a parent id, so column was used.
        with Column(spacing=10.0, padding=[10.0]):

            # add the the card
            card_id = add_card(
                head=f"Python Iced_aw Card", 
                body="This is the body of the card.", 
                foot="Foot",
                width=300.0,
                height=200.0,
                close_size=20.0,
                style_std=CardStyleStd.Success,
                on_close=minimize_card)
            
            # add the button but make show=False
            # The button can go anyplace you like,
            # you can make it unhidden all the time
            # and just change the label, let your 
            # imagination go wild :)
            btn_id = add_button(
                label="Open Card", 
                show=False,
                padding=[10],
                on_press=maximize_card,
                style_std=ButtonStyleStd.Success)
        
        
            
                
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
