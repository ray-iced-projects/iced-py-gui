from imports import *

def change_text (sldr_id: int, value: float):
    update_widget(
        wid=txt1,
        param=IpgTextParam.Width,
        value=value)
    update_widget(
        wid=txt2,
        param=IpgTextParam.Width,
        value=value)



# Add a window first
add_window(
        id="main", 
        title="CheckBox Demo",
    size=(600, 600),  
        pos_centered=True,
        debug=True)

# Add a container to center the widgets in the middle
add_container(
        window_id="main", 
        id="cont", 
        width_fill=True,
        height_fill=True,
        centered=True)

add_column(
    window_id="main",
    id="col",
    parent_id="cont")

add_slider(
    parent_id="col",
    min=0.0, 
    max=200.0,
    step=1.0,
    value=200.0,
    on_change=change_text,
    width=175.0)

txt1 = add_text(
        parent_id="col",
        content="This is some very very very very very very very very long text.",
        width=160.0)

txt2 = add_text(
        parent_id="col",
        content="This is some very very very very very very very very long text.",
        width=160.0,
        font="FiraSans-Regular")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
