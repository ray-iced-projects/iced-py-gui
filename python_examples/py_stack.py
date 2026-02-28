from imports import *

def card_selected(card_id, name):
    update_widget(
        wid=text_id, 
        param=IpgTextParam.Content, 
        value=f"Card selected is {name}")


cwd = os.getcwd()
path = path = cwd + "/python_examples/resources/cards/hearts/"

names = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]

add_window(
        id="main",
        title="Stack",
    size=(400.0, 800.0),
        pos_centered=True)

add_container(
        window_id="main",
        id="cont",
        width_fill=True,
        height_fill=True)

add_column(
        window_id="main",
        id="main_col",
        parent_id="cont",
        height_fill=True)

text_id = add_text(
                parent_id="main_col", 
                content="Card Selected is None")

# Adds the stack container to the window.
stack_id = add_stack(
                    window_id="main",
                    id="stack",
                    parent_id="main_col",
                    width=200.0,
                    height=750.0)

for i in range(1, 14):
    # Adds the column to the stack to hold the space and card.
    add_column(
            window_id="main",
            id=f"col_{i}",
            parent_id="stack")

    file = f"{path}{i}.png"

    # The space, whcich grows with each card, allows for an offset
    # to be able to see all of the cards.  If not used, they are 
    # stacked on top of each other.
    add_space(
            parent_id=f"col_{i}",
            height=35*i-35)

    add_image(
            parent_id=f"col_{i}", 
            image_path=file,
            width=200.0, 
            height=300.0,
            content_fit=IpgImageContentFit.Fill,
            mouse_pointer=IpgMousePointer.Grabbing,
            on_press=card_selected,
            user_data=f"{names[i-1]}")


start_session()

