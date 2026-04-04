from icedpygui import Window, Container, Column, Row, Grid, start_session, \
    Scrollable, add_button, add_space, add_text, add_text_input, \
    add_event_timer, IpgContainerStyleStd as std, IpgTimerParam, \
    update_widget, update_timer, GridParam, IpgButtonParam, IpgTextParam

import json, os, re
from dataclasses import dataclass


with open(os.path.join(os.path.dirname(__file__), "resources", "jeopardy", "answers.json")) as f:
    answers = json.load(f)

grid_width = 900.0
columns = 6
cell_width = grid_width/columns
cell_height = 120.0

categories = answers["categories"]
cat_list = list(categories.values())
num_rows = len(cat_list[0])

p1_score = 0
p2_score = 0
timer_status = "stopped"
timer_length = 10


@dataclass
class ids:
    value_id: int = 0
    answer_id: int = 0
    row: int = 0
    col: int = 0


def value_pressed(btn_id: int, ans_id: int):
    timer_status = "running"
    update_widget(btn_id, IpgButtonParam.Show, False)
    update_widget(ans_id, IpgButtonParam.Show, True)
    update_timer(wid=timer_id, param=IpgTimerParam.Enable, value=True)
    ids.value_id = btn_id
    ids.answer_id = ans_id


# data = qst_id, row, col
def answer_pressed(btn_id, data: tuple[int, int, int]):
    if timer_status == "running":
        update_widget(btn_id, IpgButtonParam.Show, False)
        update_widget(data[0], IpgTextParam.Show, True)
        update_timer(wid=timer_id, param=IpgTimerParam.Enable, value=False)
        ids.row = data[1]
        ids.col = data[2]
        
        

def normalize(text: str) -> str:
    text = text.lower().strip().rstrip("?").strip()
    text = re.sub(r"^(what\s+is|what's)\s+(a|an|the)?\s*", "", text)
    text = re.sub(r"[^a-z0-9 ]", "", text)
    return text.strip()


def what_is(input_id: int, value: str):
    expected = cat_list[ids.col][ids.row]["question"]
    if normalize(value) == normalize(expected):
        print("Correct!")
    else:
        print("Wrong!")


def on_tick(_timer_id: int, tick_count: int, elapsed_ms: int):
    if tick_count > timer_length:
        update_widget(wid=txt_time, param=IpgTextParam.Content, value=f"15")
        timer_status = "stopped"
        update_widget(ids.answer_id, IpgButtonParam.Show, False)
        update_widget(ids.value_id, IpgButtonParam.Show, True)
        update_timer(timer_id, IpgTimerParam.Enable, False)
        ids.value_id = 0
    else:
        update_widget(wid=txt_time, param=IpgTextParam.Content, 
                  value=f"{timer_length - tick_count}")


def on_stop(_timer_id: int, tick_count: int, elapsed_ms: int):
    update_widget(wid=txt_time, param=IpgTextParam.Content, value=f"15")


timer_id = add_event_timer(duration_ms=1000, on_tick=on_tick, on_stop=on_stop)


with Window(title="Jeopardy", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=3.0):
            
            with Row(spacing=3.0):
                for cat in categories:
                    with Container(width=cell_width-2.8, height=50, align_center=True, style_std=std.Secondary):
                        add_text(content=cat, width=cell_width)
                    
            with Grid(width=grid_width, columns_amount=columns, spacing=3.0):
                for row in range(num_rows):
                    for col in range(columns):
                        item = cat_list[col][row]
                        with Container(style_std=std.Primary, height=cell_height, width=cell_width, align_center=True):
                            with Column():
                                # question id
                                qst_id = add_text(content=item["question"], show=False)
                                
                                # answer button
                                ans_id = add_button(
                                            label=item["answer"], 
                                            on_press=answer_pressed,
                                            user_data=(qst_id, row, col),
                                            show=False)
                                
                                # value button
                                value_id = add_button(
                                            label=str(f"${200 * (row + 1)}"), 
                                            on_press=value_pressed,
                                            user_data=ans_id)
            
            add_space(height=10)                
            with Row():
                with Row(spacing=5):
                    add_text(content="Player 1 Score: ")
                    add_text(content=f"{p1_score}", width=150)
                add_space(width=50)
                with Row(spacing=5):
                    add_text(content="Player 2 Score: ")
                    add_text(content=f"{p1_score}", width=250)
                add_space(width=50)
                add_text(content="time: ")
                txt_time = add_text(content="15", width=50)
                add_space(width=50)
                add_text(content="What is ")
                add_text_input(placeholder="Type Question, then enter", width=400, on_submit=what_is)            
                            
start_session()
