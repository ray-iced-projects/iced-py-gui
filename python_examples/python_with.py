from imports import *



with Window( title="Scrollable", center=True) as wnd_id:
    with Container(center=True, width_fill=True, height_fill=True) as cont_id:
        
        add_text(
            content="This is Some Text",
        )

add_container()
start_session()
add_container()