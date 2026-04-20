#!/usr/bin/env python3
"""
Timer use demo
"""

from icedpygui import Window, Column, Container,\
    add_event_timer, add_button, update_timer, add_text, start_session,\
    TimerParam, TextParam, update_widget


def on_start(_timer_id: int, tick_count: int, elapsed_ms: int):
    """Callback by timer when started"""
    update_widget(wid=txt_id, param=TextParam.Content,
                  value=f"Timer started: tick_count={tick_count}, elapsed_ms={elapsed_ms}")

def on_tick(_timer_id: int, tick_count: int, elapsed_ms: int):
    """Callback by timer on each tick"""
    update_widget(wid=txt_id, param=TextParam.Content,
                  value=f"ticking {tick_count} elapsed_ms={elapsed_ms}")

def on_stop(_timer_id: int, tick_count: int, elapsed_ms: int):
    """Callback by timer when stopped"""
    update_widget(wid=txt_id, param=TextParam.Content,
                  value=f"Timer stopped: tick_count={tick_count}, elapsed_ms={elapsed_ms}")

def start_timer(_btn_id):
    """Callback via button to update enable the timer"""
    update_timer(wid=timer_id, param=TimerParam.Enable, value=True)

def stop_timer(_btn_id):
    """Callback vua button to disable the timer"""
    update_timer(wid=timer_id, param=TimerParam.Enable, value=False)

timer_id = add_event_timer(duration_ms=100, on_start=on_start, on_tick=on_tick, on_stop=on_stop)

# Add a window
with Window(title="Timer Demo", center=True):

    # Add the container to center everything
    with Container(fill=True, align_center=True):

        # Add the column to hold the widgets
        with Column(spacing=20.0, width=200, height=300):
            add_button(
                label="Press to Start Timer",
                on_press=start_timer)
            add_button(
                label="Press to Stop Timer",
                on_press=stop_timer)

            # Put the text in a conainer to keep the
            # buttoms from jumping around as the text changes
            # The columns width and height needed to be added too.
            with Container():
                txt_id = add_text(content="Timer data")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
