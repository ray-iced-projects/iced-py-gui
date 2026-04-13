from icedpygui import Window, Container, Column, Row, Grid, start_session, \
    add_button, add_space, add_text, add_text_input, \
    add_event_timer, ContainerParam as cp, ContainerStyleStd as std, TimerParam, \
    update_widget, update_timer, GridParam, ButtonParam, TextParam

import json, os, re
from dataclasses import dataclass

"""
Just to make a demo more interesting, this grid demo is the
Jeopardy game.  

The player scores are at the bottom and the player 
that is highlighted with color has the turn.

Click on a value and the answer is displayed

Type the question in the input box at the bottom.

Press enter to submit and see what happens.

If you are correct, your score increases, if not it decreases.
The answer is displayed if correct or back to the value.

You have 10 seconds (changeable) to type in the answer or the grid cell converts back.

The compare() function is simple but you could connect with an AI and input 
with voice recognition, etc., to make a great game. 

It's never been completed, so no winning actions.

Probably not completely debugged but it seems to work well.

Have fun!

"""

with open(os.path.join(os.path.dirname(__file__), "resources", "jeopardy", "answers.json")) as f:
    answers = json.load(f)


@dataclass
class Pdy:
    grid_width: float = 900.0
    columns: int = 6
    cell_width: float = grid_width/columns
    cell_height = 120.0
    
    categories = answers["categories"]
    cat_list = list(categories.values())
    num_rows = len(cat_list[0])

    value_id: int = 0
    answer_id: int = 0
    question_id: int = 0
    timer_id: int = 0
    
    row: int = 0
    col: int = 0
    value: int = 0
    
    p1_score: int = 0
    p2_score: int = 0
    p1_turn: bool = True
    p2_turn: bool = False
    p1_score_id: int = 0
    p2_score_id: int = 0
    p1_cont_score_id: int = 0
    p2_cont_score_id: int = 0
    
    timer_running: bool = False
    timer: int = 10
    



def value_pressed(btn_id: int, data: tuple[int, int, int, int, int]):
    if not Pdy.timer_running:
        Pdy.timer_running = True
        ans_id = data[0]
        update_widget(btn_id, ButtonParam.Show, False)
        update_widget(ans_id, TextParam.Show, True)
        update_timer(Pdy.timer_id, TimerParam.Enable, True)
        Pdy.value_id = btn_id
        Pdy.answer_id = ans_id
        Pdy.question_id = data[1]
        Pdy.row = data[2]
        Pdy.col = data[3]
        Pdy.value = data[4]
    

def on_tick(_timer_id: int, tick_count: int, elapsed_ms: int):
    if tick_count > Pdy.timer:
        update_widget(wid=txt_time, param=TextParam.Content, value=f"15")
        Pdy.timer_running = False
        update_widget(Pdy.answer_id, TextParam.Show, False)
        update_widget(Pdy.value_id, ButtonParam.Show, True)
        update_timer(Pdy.timer_id, TimerParam.Enable, False)
        Pdy.value_id = 0
    else:
        update_widget(wid=txt_time, param=TextParam.Content, 
                  value=f"{Pdy.timer - tick_count}")


def on_stop(_timer_id: int, tick_count: int, elapsed_ms: int):
    update_widget(txt_time, TextParam.Content, f"15")


Pdy.timer_id = add_event_timer(duration_ms=1000, on_tick=on_tick, on_stop=on_stop)

def normalize(text: str) -> str:
    text = text.lower().strip().rstrip("?").strip()
    text = re.sub(r"^(what\s+is|what's)\s+(a|an|the)?\s*", "", text)
    text = re.sub(r"\bor\b", "", text)
    text = re.sub(r"[^a-z0-9 ]", "", text)
    return text.strip()

def compare(actual: str, player: str) -> bool:
    actual_words = set(actual.lower().split())
    player_words = set(player.lower().split())
    if not actual_words:
        return False
    matches = actual_words & player_words
    return len(matches) / len(actual_words) >= 0.5


def what_is(input_id: int, value: str):
    if not Pdy.timer_running:
        return
    Pdy.timer_running = False
    expected = Pdy.cat_list[Pdy.col][Pdy.row]["question"]
    results = compare(normalize(expected), normalize(value))
    if  results :
        if Pdy.p1_turn:
            Pdy.p1_score += Pdy.value
            update_widget(Pdy.p1_score_id, TextParam.Content, f"{Pdy.p1_score}")
        elif Pdy.p2_turn:
            Pdy.p2_score += Pdy.value
            update_widget(Pdy.p2_score_id, TextParam.Content, f"{Pdy.p2_score}")
        else: 
            print("P1 and P2 turns both set to False")
            quit()
        update_widget(Pdy.question_id, TextParam.Show, True)
        update_widget(Pdy.answer_id, TextParam.Show, False)
        update_timer(Pdy.timer_id, TimerParam.Enable, False)
    
    else:
        if Pdy.p1_turn:
            Pdy.p1_score -= Pdy.value
            update_widget(Pdy.p1_score_id, TextParam.Content, f"{Pdy.p1_score}")
        elif Pdy.p2_turn:
            Pdy.p2_score -= Pdy.value
            update_widget(Pdy.p2_score_id, TextParam.Content, f"{Pdy.p2_score}")
        else:
            print("P1 and P2 turns both set to False")
            quit()
        update_widget(Pdy.answer_id, TextParam.Show, False)
        update_widget(Pdy.value_id, ButtonParam.Show, True)
        update_timer(Pdy.timer_id, TimerParam.Enable, False)
        
        # wrong answer so switch players
        Pdy.p1_turn = not Pdy.p1_turn
        Pdy.p2_turn = not Pdy.p2_turn

        if Pdy.p1_turn:
            update_widget(Pdy.p1_cont_score_id, cp.StyleStd, std.Primary)
            update_widget(Pdy.p2_cont_score_id, cp.StyleStd, std.Transparent)
        else:
            update_widget(Pdy.p1_cont_score_id, cp.StyleStd, std.Transparent)
            update_widget(Pdy.p2_cont_score_id, cp.StyleStd, std.Primary)



with Window(title="Jeopardy", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=3.0):
            
            with Row(spacing=3.0):
                for cat in Pdy.categories:
                    with Container(width=Pdy.cell_width-2.8, height=50, align_center=True, style_std=std.Secondary):
                        add_text(content=cat, width=Pdy.cell_width, align_center=True)
            
            # Grid only needs width and the number of columns, spacing is optional
            # Treat it just like a container and put whatever you want into each cell 
            with Grid(width=Pdy.grid_width, columns_amount=Pdy.columns, spacing=3.0):
                # typical row/column iteration
                for row in range(Pdy.num_rows):
                    for col in range(Pdy.columns):
                        item = Pdy.cat_list[col][row]
                        with Container(style_std=std.Primary, height=Pdy.cell_height, width=Pdy.cell_width, align_center=True):
                            with Column():
                                # question
                                qst_id = add_text(content=item["question"], show=False)
                                
                                # answer
                                ans_id = add_text(content=item["answer"], show=False)
                                
                                # value button
                                value = 200 * (row + 1)
                                value_id = add_button(
                                            label=str(f"${value}"), 
                                            on_press=value_pressed,
                                            user_data=(ans_id, qst_id, row, col, value))
            
            add_space(height=10)                
            with Row():
                # This is styled because P1 starts first
                # When it's P2's turn, then the styles are updated
                with Container(width=175, style_std=std.Primary) as Pdy.p1_cont_score_id:
                    with Row():
                        add_text(content="Player 1 Score: ")
                        Pdy.p1_score_id = add_text(content=f"{Pdy.p1_score}", width=150)
                add_space(width=20)
                with Container(width=175) as Pdy.p2_cont_score_id:
                    with Row():
                        add_text(content="Player 2 Score: ")
                        Pdy.p2_score_id = add_text(content=f"{Pdy.p1_score}", width=150)
                add_space(width=20)
                add_text(content="time: ")
                txt_time = add_text(content=f"{Pdy.timer}", width=30)
                add_space(width=20)
                add_text(content="What is ")
                add_text_input(placeholder="Type Question, then enter", width=400, on_submit=what_is)            
                
start_session()
