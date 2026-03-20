from icedpygui import Window, Container, Column, Row, IpgCardParam,\
    add_card, add_button, IpgButtonParam, update_widget, IpgCardStyleStd, start_session

card_ids = []
btn_ids = []

# The callback will minimizes the first card, the button at the bottom left will maximize it.
def minimize_card(card_id: int, index: int):
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
    update_widget(
        wid=btn_ids[index],
        param=IpgButtonParam.Show,
        value=True)


# Pressing the button will maximize the card.
# Note the callback is from the button so the card_ids have to be
# accessible by this maximize method.
def maximize_card(_btn_id: int, index: int):
    global card_ids
    update_widget(
        wid=card_ids[index], 
        param=IpgCardParam.IsOpen, 
        value=True)
    update_widget(
        wid=btn_ids[index],
        param=IpgButtonParam.Show,
        value=False)

# The style id is used in the card style_id to set the style
# In this case we create 4 std styles for later use
style_ids = [IpgCardStyleStd.Primary, IpgCardStyleStd.Success, IpgCardStyleStd.Danger, IpgCardStyleStd.Secondary]


# window added first
with Window(
    title="Card Demo",
    size=(800, 600),
    center=True):

    
    with Container(fill=True, align_center=True):
        # Add the column to hold the cards
        with Column(
            align_center=True, 
            width=400.0,
            spacing=10.0, 
            padding=[10.0]):

            # add the the cards.
            for index in range(0, 4):
                id = add_card(
                    head=f"Python Iced_aw Card {index}", 
                    body="This is the body of the card.", 
                    foot="Foot",
                    width=300.0,
                    height=200.0,
                    style_std=style_ids[index],
                    on_close=minimize_card,
                    user_data=index)
                
                card_ids.append(id) # needed by maximize method

            # Add a row to how the buttons
            with Row(spacing=20.0):
                for index in range(0, 4):
                    id = add_button(
                            label=f"Card {index}",
                            on_press=maximize_card,
                            show=False,
                            user_data=index)
                    
                    btn_ids.append(id)
            
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
