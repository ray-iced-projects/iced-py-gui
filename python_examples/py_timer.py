from icedpygui import Window, Column, Container,\
    add_event_timer, add_button, update_timer, add_text, start_session,\
    IpgTimerParam, IpgTextParam, update_widget


def on_start(_timer_id: int, tick_count: int, elapsed_ms: int):
    update_widget(wid=txt_id, param=IpgTextParam.Content, 
                  value=f"Timer started: tick_count={tick_count}, elapsed_ms={elapsed_ms}")

def on_tick(_timer_id: int, tick_count: int, elapsed_ms: int):
    update_widget(wid=txt_id, param=IpgTextParam.Content, 
                  value=f"ticking {tick_count} elapsed_ms={elapsed_ms}")

def on_stop(_timer_id: int, tick_count: int, elapsed_ms: int):
    update_widget(wid=txt_id, param=IpgTextParam.Content, 
                  value=f"Timer stopped: tick_count={tick_count}, elapsed_ms={elapsed_ms}")

timer_id = add_event_timer(duration_ms=1000, on_start=on_start, on_tick=on_tick, on_stop=on_stop)

def start_timer(_btn_id):
    update_timer(wid=timer_id, param=IpgTimerParam.Enable, value=True)

def stop_timer(_btn_id):
    update_timer(wid=timer_id, param=IpgTimerParam.Enable, value=False)
    
    

# Add a window
with Window(title="Timer Demo", center=True):

    # Add the container to center everything
    with Container(fill=True, align_center=True):

        # Add the column to hold the widgets
        with Column(spacing=20.0):
            add_button(
                label="Press to Start Timer",
                on_press=start_timer)
            add_button(
                label="Press to Stop Timer",
                on_press=stop_timer)
            
            txt_id = add_text(content="Timer data")        


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()

