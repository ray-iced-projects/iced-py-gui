from icedpygui import Window, Column, Container, Stack, start_session, \
    add_image, add_text, add_space, update_widget, IpgTextParam, \
    IpgContentFit, MouseArea, IpgMousePointer
    
import os


def card_selected(card_id, name):
    update_widget(
        wid=text_id, 
        param=IpgTextParam.Content, 
        value=f"Card selected is {name}")


cwd = os.getcwd()
path = path = cwd + "/python_examples/resources/cards/hearts/"

names = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]

with Window(title="Stack", center=True):

    with Container(fill=True, align_center=True):

        with Column(width=200, height_fill=True):
            add_space(height=30)
            text_id = add_text(content="Card Selected is None")
            add_space(height=30)
            # Adds the stack container to the window.
            with Stack(height_fill=True):

                for i in range(1, 14):
                    # Adds the column to the stack to hold the space and card.
                    with Column():

                        file = f"{path}{i}.png"

                        # The space, which grows with each card, allows for an offset
                        # to be able to see all of the cards.  If not used, they are 
                        # stacked on top of each other.
                        add_space(height=35*i-35)
                        with Container(width=150, height=250):
                            with MouseArea(
                                mouse_pointer=IpgMousePointer.Grab,
                                on_press=card_selected,
                                user_data=names[i-1]):
                                add_image(
                                    image_path=file,
                                    content_fit=IpgContentFit.Fill)


start_session()

