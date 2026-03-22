from icedpygui import Window, Column, Container,\
    add_event_timer, add_button, update_widget, start_session,\
    IpgTimerParam


timer_id = add_event_timer()

def timer(_btn_id):
    update_widget(wid=timer_id, param=IpgTimerParam.Enable, value=True)
    

# Add a window
with Window(title="Timer Demo", center=True):

    # Add the container to center everything
    with Container(fill=True, align_center=True):

        # Add the column to hold the widgets
        with Column(spacing=20.0):
            add_button(
                label="Press to Start Timer",
                on_press=timer)
            
                    




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()

