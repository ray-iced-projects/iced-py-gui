from icedpygui import Window, Column, Container, add_button, add_text, show_widget, hide_widget, start_session


def show(_id: int):
    show_widget(txt_id1)
    show_widget(txt_id2)
    
def hide(_id: int):
    hide_widget(txt_id1)
    hide_widget(txt_id2)

with Window(title="Test Widget Delete", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=20.0):
            add_button(
                label="Pressing me to SHOW the text below.",
                on_press=show)
            add_button(
                label="Pressing me to HIDE the text below.",
                on_press=hide)
            txt_id1 = add_text(content="Hi there", show=False)
            txt_id2 = add_text(content="Hi there", show=False)
    
start_session()
