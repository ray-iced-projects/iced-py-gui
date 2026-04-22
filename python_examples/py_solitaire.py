#!/usr/bin/env python3
"""
Solitare demo
"""

import random, os
from icedpygui import Window, Column, Container, MouseArea, Row, Stack, start_session, \
        add_button, add_mouse_area, add_space, add_stack, add_text, TextParam, \
        add_pick_list, add_image, update_widget, \
        add_container_style, move_widget, \
        Color, MousePointer, ContentFit, generate_id, \
        add_container \

class solitaire:
    def __init__(self) -> None:
        # container ids
        self.wnd: int = 0,
        self.main_col = 0,
        self.tab_stack_ids = []
        self.tab_stack_ma_ids = []
        self.tab_col = []

        self.cwd = os.getcwd()
        self.path = self.cwd + "/python_examples/resources/cards/"
        self.card_width: float=100.0
        self.card_height: float=150.0
        self.shuffled_indexes: list=[]
        self.cards: dict={}
        self.stock: list=[]
        self.waste: list=[]
        self.covers: list=[]
        self.tableau: list=[]
        self.status_id: int=0
        self.deal_amount: int=3
        self.content: str=""
        self.rounds = 0
        self.rounds_id = 0
        self.cards_to_play = "3"
        self.tab_cover_cards_wids = []

        self.stock_cover_id: int=0

        self.origin: int=None
        self.target: int=None

        self.foundation_top_card_value: list=[0] * 4
        self.foundation_top_card_suite: list=[None] * 4

        # styles
        self.white_border = 0

        # if the cards in the tableau going outside of this height,
        # an error will occur.  Increase the window height and this
        # if needed
        self.stack_height = 470.0

    def start_game(self):
        """Start Game"""
        self.create_styles()
        self.load_shuffle_cards()

        # add the main containers
        with Window(title="Solitaire", size=(1000.0, 700.0), center=True) as self.wnd:

            with Row(fill=True, spacing=2.0) as main_row:

                with Container(
                    width=175.0,
                    height_fill=True,
                    style_id=self.white_border) as control_cont:

                    with Column(padding=[20.0], height_fill=True) as ctrl_col:
                        self.define_controls(ctrl_col)

                with Container(fill=True, style_id=self.white_border):

                    with Column(fill=True) as self.main_col:
                        self.create_top_slots()
                        self.create_status_row()
                        self.create_tableau_cards()
                        self.deal_cards()
                        self.add_remaining_to_stock()

        start_session()

    def restart_play(self, _btn_id: int):
        """Restart"""
        self.rounds = 0
        update_widget(self.rounds_id, TextParam.Content, f"Card Play Rounds: {self.rounds}")

        # reshuffle the original card indexes
        random.shuffle(self.shuffled_indexes)

        # get the stock cover card amd mouseareas from the cards before re-initializing
        stock_cover_card = self.cards.get(self.stock_cover_id)
        mas = []
        tab_cover_cards = {}
        for wid in self.cards:
            card = self.cards.get(wid)
            name = card.get("name")
            if name == "stock blank":
                mas.append(card)
            if "foundation" in name:
                mas.append(card)
            if name == "tab_mousearea":
                mas.append(card)
            if name == "tab_cover":
                tab_cover_cards[wid] = card

        # fill the tableau
        self.cards = {}
        self.tableau = []
        self.content = ""
        index = 0
        cover_index = 0
        for i in range(0, 7):
            self.tableau.append([])
            for j in range(0, 13):

                if j <= i:
                    card = self.shuffled_indexes[index]
                    card["tab_column"] = i
                    card["tab_index"] = j
                    card["tableau"] = True
                    card["foundation"] = False
                    card["stock"] = False
                    card["waste"] = False
                    wid = card.get("wid")
                    self.cards[wid] = card
                    self.tableau[i].append(wid)
                    move_widget(window_id="main",
                                widget_id=wid,
                                target_container_str_id=f"tabcol_{i}_{j}"
                                )
                    index += 1

                if j < i:
                    wid = self.tab_cover_cards_wids[cover_index]
                    cover = tab_cover_cards.get(wid)
                    cover["index"] = index-1
                    cover["tab_column"] = i
                    cover["tab_index"] = j
                    cover["is_cover"] = True
                    cover["tableau"] = True
                    cover_index += 1
                    move_widget(window_id="main",
                                widget_id=wid,
                                target_container_str_id=f"tab_blank_{i}_{j}"
                                )
                    self.cards[wid] = cover

        # add cards left to stock
        self.stock = []
        self.waste = []

        for idx in range(index, len(self.shuffled_indexes)):
            card = self.shuffled_indexes[idx].copy()
            card["stock"] = True
            card["waste"] = False
            card["tableau"] = False
            card["foundation"] = False

            wid = card.get("wid")
            self.stock.append(wid)
            self.cards[wid] = card
            move_widget(window_id="main",
                                 widget_id=wid,
                                 target_container_str_id="stack_stock_pile"
                                 )

        # restore the stock cover card and mouseares cards
        move_widget("main", self.stock_cover_id, "stack_stock_pile")
        self.cards[self.stock_cover_id] = stock_cover_card

        for ma in mas:
            self.cards[ma.get("wid")] = ma

    def create_styles(self):
        """Create Style"""
        self.white_border = add_container_style(
                                border_color=Color.WHITE,
                                border_width=2.0)

    def define_controls(self, ctrl_col):
        """Define Controls"""
        add_button(parent_id=ctrl_col,
                        label="Restart Play",
                        on_press=self.restart_play)

        self.rounds_id = add_text(parent_id=ctrl_col,
                          content="Card Play Rounds:")

        add_text(parent_id=ctrl_col,
                          content="Cards to Play:")

        add_pick_list(parent_id=ctrl_col,
                               options=["1", "3"],
                               on_select=self.select_cards_to_play,
                               selected=self.cards_to_play)

        add_space(parent_id=ctrl_col, height=50.0)
        add_text(parent_id=ctrl_col,
                          content="Instructions:\nCards are moved by selecting source and destination using mouse.  If a card fails to move it means the validation failed, wrong color or value.\nTo cancel a move, click any other place on the canvas")

    def select_cards_to_play(self, _picklist_id: int, value: str):
        """Select the Cards"""
        self.cards_to_play = int(value)

    def create_top_slots(self):
        """Create the slots for the cards"""
        self.cards = {}
        # add row for stock, waste, and foundation cards
        with Row(window_id=self.wnd,
            parent_id=self.main_col,
            height=self.card_height,
            spacing=10.0,
            padding=[5.0]): # stock_row

            # add some beginning space
            add_space(width=20.0)

            # add the stock container to the row
            with Container(
                padding=[0.0],
                style_id=self.white_border): #id=stock

                # add the stack in
                with Stack(
                           width=self.card_width,
                           height=self.card_height): # "stack_stock_pile"

                    wid = add_mouse_area(
                                    mouse_pointer=MousePointer.Grab,
                                    on_press=self.card_selected,
                                    )
            stock = {}
            stock["wid"] = wid
            stock["name"] = "stock blank"
            stock["value"] = 0
            stock["suite"] = None
            stock["color"] = None
            stock["foundation"] = False
            stock["tableau"] = False
            stock["stock"] = True
            stock["waste"] = False
            stock["is_cover"] = True
            stock["reload"] = True
            self.cards[wid] = stock

            # add the waste container to the row
            with Container(
                width=self.card_width,
                height=self.card_height,
                padding=[0.0],
                style_id=self.white_border): # waste

                # add the stack in
                # stack_waste_pile
                add_stack(
                    width=self.card_width,
                    height=self.card_height)

            # add a space between waste and foundation
            add_space(width=self.card_width)

            # Add the 4 foundation slots
            for i in range(0, 4):
                with Stack(
                           width=self.card_width,
                           height=self.card_height):

                    with MouseArea(
                        mouse_pointer=MousePointer.Grab,
                        on_press=self.card_selected):

                        fd = {}
                        fd["wid"] = wid
                        fd["name"] = f"foundation {i}"
                        fd["value"] = 0
                        fd["suite"] = None
                        fd["color"] = None
                        fd["foundation"] = True
                        fd["tableau"] = False
                        fd["fd_index"] = i
                        self.cards[wid] = fd

                        add_container(
                                    # id=f"foundation_container_{i}",
                                    width=self.card_width,
                                    height=self.card_height,
                                    padding=[0.0],
                                    style_id=self.white_border)

            # add a container off screen to hide widget that become unused
            add_space(width=200.0)
            add_stack(container_id="hidden", show=False)

        # Add a space between the rows
        add_space(parent_id=self.main_col, height=20.0)

    def create_status_row(self):
        """Status Row"""
        with Row(parent_id=self.main_col):
            add_space(width=20.0)
            self.status_id = add_text(content="Status: Selected None")

    def create_tableau_cards(self):
        """Tableau Cards"""
        with Row(spacing=10.0):

            # Add a space at the beginning of the row
            add_space(width=20.0)

            # Add the 7 card tableau slots
            for _ in range(7):
                with Container():
                    with Column():
                        # Add in the stacks
                        self.tab_stack_ids.append(add_stack(
                            width=self.card_width,
                            height=self.stack_height))

                        self.tab_stack_ma_ids.append(add_mouse_area(
                                mouse_pointer=MousePointer.Grab,
                                on_press=self.card_selected))

    def load_shuffle_cards(self):
        """Shuffle Cards"""
        suites = [
            ("hearts", "RED"),
            ("diamonds", "RED"),
            ("clubs", "BLACK"),
            ("spades", "BLACK"),
        ]
        ranks = [
            ("Ace", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("10", 10),
            ("Jack", 11),
            ("Queen", 12),
            ("King", 13),
        ]

        self.shuffled_indexes = []

        for (suite, color) in suites:
            for (name, value) in ranks:
                d = {"wid": None,
                    "suite": suite,
                    "color": color,
                    "name": name,
                    "value": value,
                    "stock": False,
                    "waste": False,
                    "tableau": False,
                    "foundation": False,
                    "tab_column": None,
                    "tab_index": None,
                    "fd_index": 0,
                    "is_cover": False,
                    "is_card": True,
                    }
                self.shuffled_indexes.append(d)

        random.shuffle(self.shuffled_indexes)

    def deal_cards(self):
        """Dealing the cards"""
        self.content = ""
        index = 0
        #  row, cols
        self.tableau = [[0 for _ in range(13)] for _ in range(7)]
        self.tab_col = [[0 for _ in range(13)] for _ in range(7)]
        for i in range(7):
            for j in range(13):
                with Column(window_id=self.wnd, parent_id=self.tab_stack_ids[i]) as tab_col:
                    self.tab_col[i][j] = tab_col

                    # Add a blank at top to hide the card below
                    add_space(height=20*j)

                    if j == i:
                        # Show card
                        card = self.shuffled_indexes[index]
                        card["tab_column"] = i
                        card["tab_index"] = j
                        card["tableau"] = True
                        file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
                        image_id = generate_id()
                        with MouseArea(mouse_pointer=MousePointer.Grab,
                                        on_press=self.card_selected,
                                        user_data=image_id) as wid:
                            add_image(
                                path=file,
                                width=self.card_width,
                                height=self.card_height,
                                content_fit=ContentFit.Fill,
                                gen_id=image_id)

                        self.shuffled_indexes[index]["wid"] = wid  # needed later when restarting
                        card["wid"] = wid
                        self.cards[wid] = card
                        self.tableau[i][j] = wid
                        index += 1

                    if j < i:
                        # add the blank over the card unless last one.
                        with Column():
                            cover = {}
                            cover["name"] = "tab_cover"
                            cover["index"] = index-1
                            cover["tab_column"] = i
                            cover["tab_index"] = j
                            cover["is_cover"] = True
                            cover["tableau"] = True
                            file = f"{self.path}/card_back.png"

                            wid = add_image(
                                        path=file,
                                        width=self.card_width,
                                        height=self.card_height,
                                        content_fit=ContentFit.Fill)

                            self.cards[wid] = cover
                            self.tab_cover_cards_wids.append(wid)

    def add_remaining_to_stock():
        """add cards left to stock"""
        # self.stock = []
        # for idx in range(index, len(self.shuffled_indexes)):
        #     card = self.shuffled_indexes[idx]
        #     file = f"{self.path}/{card.get('suite')}/{card.get('value')}.png"
        #     card["stock"] = True

        #     wid = add_image(parent_id="stack_stock_pile",
        #                         image_path=file,
        #                         width=self.card_width,
        #                         height=self.card_height,
        #                         content_fit=ContentFit.Fill,
        #                         mouse_pointer=MousePointer.Grabbing,
        #                         on_press=self.card_selected,
        #                         )
        #     self.stock.append(wid)
        #     card["wid"] = wid
        #     self.cards[wid] = card

        # # add a cover
        # file = f"{self.path}/card_back.png"
        # wid = add_image(parent_id=f"stack_stock_pile",
        #                     image_path=file,
        #                     width=self.card_width,
        #                     height=self.card_height,
        #                     content_fit=ContentFit.Fill,
        #                     mouse_pointer=MousePointer.Grabbing,
        #                     on_press=self.card_selected,
        #                     )
        # cover = {}
        # cover["name"] = "stock_cover"
        # cover["is_cover"] = True
        # cover["stock"] = True
        # cover["foundatiuon"] = False
        # cover["waste"] = False
        # cover["tableau"] = False
        # cover["wid"] = wid
        # self.stock_cover_id = wid
        # self.cards[wid] = cover

    def card_selected(self, _mouse_id: int, card_id: int):
        """Card Selected"""
        if self.origin is None:
            self.origin = card_id
            card = self.cards.get(card_id)
            if card.get("is_cover") and card.get("tableau"):
                # check to see that the turnover card is the last one
                if card.get("tab_index") < len(self.tableau[card.get("tab_column")])-1:
                    return
                self.turn_tab_cover_over()
                self.origin = None
                return
            elif card.get("is_cover") and card.get("stock") and card.get("reload"):
                self.reload_stock()
                self.origin = None
                return
            elif card.get("is_cover") and card.get("stock"):
                self.move_stock_to_waste()
                self.origin = None
                return
            elif card.get("stock") and card.get("stock"):
                self.origin = None
                return
            elif card.get("name") == "tab_mousearea":
                self.origin = None
                return

        elif self.target is None:
            self.target = card_id
        else:
            print(self.cards.get(self.origin))
            print(self.cards.get(self.target))
            raise Exception("origin and target are both not None")

        if self.origin is not None and self.target is not None:
            self.move_card()
            update_widget(self.status_id, TextParam.Content, self.content)

    def move_card(self):
        """Move Card"""
        ids = []
        if self.cards.get(self.origin).get("tableau") and self.cards.get(self.target).get("tableau"):
            ids = self.move_tab_to_tab()
        elif self.cards.get(self.origin).get("tableau") and self.cards.get(self.target).get("foundation"):
            ids = self.move_tab_to_foundation()
        elif self.cards.get(self.origin).get("waste") and self.cards.get(self.target).get("tableau"):
            ids = self.move_waste_to_tableau()
        elif self.cards.get(self.origin).get("waste") and self.cards.get(self.target).get("foundation"):
            ids = self.move_waste_to_foundation()
        elif self.cards.get(self.origin).get("stock") and self.cards.get(self.target).get("waste"):
            ids = self.move_stock_to_waste()
        elif self.cards.get(self.origin).get("waste") and self.cards.get(self.target).get("stock"):
            self.content = "Cannot move waste to stock"
            ids = None
        elif self.cards.get(self.origin).get("tableau") and self.cards.get(self.target).get("stock"):
            self.content = "Cannot move a card to stock"
            ids = None
        elif self.cards.get(self.origin).get("Stock") and self.cards.get(self.target).get("stock"):
            self.content = "Cannot move a card back to stock"
            ids = None

        if ids is not None:
            for wid, str_id in ids:
                move_widget(window_id="main",
                                    widget_id=wid,
                                    target_container_str_id=str_id,
                                    move_before=None,
                                    move_after=None
                                    )
        self.origin = None
        self.target = None

    def turn_tab_cover_over(self):
        # hide the cover card by moving it off screen
        move_widget(window_id="main",
                                widget_id=self.origin,
                                target_container_str_id="hidden"
                                )

    def move_tab_to_tab(self):
        origin = self.cards.get(self.origin)
        target = self.cards.get(self.target)

        if target.get("is_cover"):
            content = "You cannot move a card to a cover card"
            return None

        # if tab empty and card is a king then move
        if target.get("name") == "tab_mousearea" and origin.get("value") == 13:
            # ok adding king to empty slot
            pass
        else:
            if target.get("name") == "tabmousearea" and origin.get("value") != 13:
                self.content = "You cannot move a card to an empty space"
                return None

            if origin.get("color") == target.get("color"):
                self.content = "Origin and target card colors must not match"
                return None

            if origin.get("value") != target.get("value")-1:
                self.content = "The origin value must be one less than the target value"
                return None

            if target.get("tab_index") < len(self.tableau[target.get("tab_column")])-1:
                self.content = "The selected taget must be the last card in the column"
                return None

        # find the index of the selected card
        tab_card_ids = self.tableau[origin.get("tab_column")]
        origin_id = origin.get("wid")
        found = False
        ids_to_move = []
        for i, card_id in enumerate(tab_card_ids):
            if origin_id == card_id:
                found = True
            if found:
                ids_to_move.append(card_id)

        tar_container_id = []
        tab_index = target.get("tab_index")
        tar_tab_column = target.get("tab_column")
        origin_tab_column = origin.get("tab_column")

        for i, wid in enumerate(ids_to_move):
            card_to_move = self.cards.get(wid)

            tar_tab_index =  tab_index + i + 1 # add 1 since moved after
            tar_container_id.append((wid, f"tabcol_{tar_tab_column}_{tar_tab_index}"))

            # tableau index adjustments
            self.tableau[origin_tab_column].remove(wid)
            self.tableau[tar_tab_column].append(wid)

            # adjust the origin card indexes
            self.cards[wid]["tab_column"] = tar_tab_column
            self.cards[wid]["tab_index"] = tar_tab_index

        return tar_container_id

    def move_tab_to_foundation(self):
        target = self.cards.get(self.target)
        origin = self.cards.get(self.origin)
        fd_slot = target.get("fd_index")

        # if foundation empty and card is an ace then continue
        if target.get("value") == 0 and  origin.get("value") == 1:
            pass
        else:
            # check the value
            if target.get("value") != origin.get("value") - 1:
                self.content = f"You cannot move the card {origin.get('name')} with a value of {origin.get('value')} to the foundation slot {fd_slot}"
                return None

            if target.get("suite") != origin.get("suite"):
                self.content = f"You cannot move the card having a suite of {origin.get('suite')} to foundation slot {fd_slot}"
                return  None

        origin_id = origin.get("wid")
        tab_col = origin.get("tab_column")
        self.tableau[tab_col].remove(origin_id)
        self.cards[origin_id]["foundation"] = True
        self.cards[origin_id]["tableau"] = False
        self.cards[origin_id]["fd_index"] = fd_slot
        self.cards[origin_id]["tab_column"] = None
        self.cards[origin_id]["tab_index"] = None

        self.content = f"Card {origin.get('name')} was moved to foundation slot {fd_slot}"
        return [(origin_id, f"foundation_{fd_slot}")]

    def move_waste_to_tableau(self):
        origin = self.cards.get(self.origin)
        target = self.cards.get(self.target)

        # if tab empty and card is a king then move
        if target.get("tab_index") == -1 and origin.get("value") == 13:
            pass
        else:
            if target.get("name") == "tab_mousearea" and origin.get("value") != 13:
                self.content = "Card cannot be move to an empty space"
                return None

            if origin.get("color") == target.get("color"):
                self.content = "Origin and target card colors must not match"
                return None

            if origin.get("value") != target.get("value")-1:
                self.content = "The origin value must be one less than the target value"
                return None

        tar_tab_column = target.get("tab_column")
        tar_tab_index = target.get("tab_index") + 1 # add 1 since moved after
        tar_container_id = f"tabcol_{tar_tab_column}_{tar_tab_index}"

        origin_id = origin.get("wid")

        # tableau ids adjustments
        self.tableau[tar_tab_column].append(origin_id)

        # adjust the origin card indexes
        self.cards[origin_id]["tab_column"] = tar_tab_column
        self.cards[origin_id]["tab_index"] = tar_tab_index
        self.cards[origin_id]["tableau"] = True
        self.cards[origin_id]["waste"] = None
        self.waste.remove(origin_id)

        return [(origin_id, tar_container_id)]

    def move_stock_to_waste(self):
        ids_to_move = []

        if len(self.stock) == 0:
            move_widget("main", self.stock_cover_id, "hidden")

        if self.cards_to_play == "3":
            if len(self.stock) >= 3:
                ids_to_move = self.stock[-3:]
                self.stock = self.stock[0:len(self.stock)-3]
            elif len(self.stock) >= 2:
                ids_to_move = self.stock[-2:]
                self.stock = self.stock[0:len(self.stock)-2]
            elif len(self.stock) >= 1:
                ids_to_move = self.stock[-1:]
                self.stock = self.stock[0:len(self.stock)-1]
            else:
                return None
        else:
            if len(self.stock) >= 1:
                ids_to_move = self.stock[-1:]
                self.stock = self.stock[0:len(self.stock)-1]
            else:
                return None

        for wid in ids_to_move:
            self.cards.get(wid)["stock"] = False
            self.cards.get(wid)["waste"] = True
            move_widget(window_id="main",
                                widget_id=wid,
                                target_container_str_id="stack_waste_pile",
                                )

        self.waste.extend(ids_to_move)
        self.content = "Cards dealt"
        return None

    def reload_stock(self):
        # Move the ids back to the stock and reverse the order
        self.stock = self.waste
        self.waste = []
        self.stock.reverse()
        # move the cards back to stock
        for wid in self.stock:
            self.cards.get(wid)["stock"] = True
            self.cards.get(wid)["waste"] = False
            move_widget("main", wid, "stack_stock_pile")
        # Move the cover card on top
        move_widget("main", self.stock_cover_id, "stack_stock_pile")
        self.rounds += 1
        update_widget(self.rounds_id, TextParam.Content, f"Card Play Rounds: {self.rounds}")

    def move_waste_to_foundation(self):
        target = self.cards.get(self.target)
        origin = self.cards.get(self.origin)
        fd_slot = target.get("fd_index")

        # if foundation empty and card is an ace then continue
        if target.get("value") == 0 and  origin.get("value") == 1:
            None
        else:
            # check the value
            if target.get("value") != origin.get("value") - 1:
                self.content = f"You cannot move the card {origin.get('name')} with a value of {origin.get('value')} to the foundation slot {fd_slot}"
                return None

            if target.get("suite") != origin.get("suite"):
                self.content = f"You cannot move the card having a suite of {origin.get('suite')} to foundation slot {fd_slot}"
                return  None

        origin_id = origin.get("wid")
        self.cards[origin_id]["foundation"] = True
        self.cards[origin_id]["waste"] = False
        self.cards[origin_id]["fd_index"] = fd_slot
        self.waste.remove(origin_id)

        self.content = f"Card {origin.get('name')} was moved to foundation slot {fd_slot}"
        return [(origin_id, f"foundation_{fd_slot}")]


game = solitaire()
game.start_game()

