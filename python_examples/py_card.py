from icedpygui import Window, Container, Column, IpgCardParam,\
    add_card, add_card_style, add_button, update_widget, IpgColor, start_session

card_id = 0

# The callback will minimizes the first card, the button at the bottom left will maximize it.
def minimize_card(card_id: int):
    # In this case the card has a built in button, it can trigger the minimization.
    # Therefore, unlike most other widgets, the id is the card_id needed.
    # The update widget will always need a type where the correct
    # parameter can be selected.  In this case is was IsOpen.
    # id you look at the Card widget docs, you will know what the value
    # type will be, in this case a boolean.
    update_widget(
        wid=card_id, 
        param=IpgCardParam.IsOpen, 
        value=False)


# Pressing the bottom button will maximize the card, returning it to the top.
# Note the callback is from the button so the card_id has to be global.
# Normally, you would use a class or dataclass to store these ids.
def maximize_card(_btn_id: int):
    global card_id
    update_widget(
        wid=card_id, 
        param=IpgCardParam.IsOpen, 
        value=True)

# The style id is used in the card style_id to set the style
# In this case we create 4 simple styles for later use
colors = [IpgColor.PRIMARY, IpgColor.SUCCESS, IpgColor.DANGER, IpgColor.BLUE]
style_ids = []
for i in range(0, 4):
    id = add_card_style(
            body_background_color=colors[i],
            foot_background_color=IpgColor.DARK_GREY)
    style_ids.append(id)

# window added first
with Window(
    title="Card Demo",
    size=(800, 600),
    center=True):

    
    with Container(fill=True, align_center=True):

        with Column(
            align_center=True, 
            width=400.0,
            spacing=10.0, 
            padding=[10.0]):

            # define the head and body of the cards.
            head = "Python Iced_aw Card"
            body = "This is the body of the card."
            foot = "Foot"
            for _ in range(0, 4):
                add_card(
                    head=head, 
                    body=body, 
                    foot=foot,
                    width=300.0,
                    height=200.0,
                    style_id=style_ids[i],
                    # on_close=minimize_card
                    )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
