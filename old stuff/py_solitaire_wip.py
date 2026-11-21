#!/usr/bin/env python3
"""
Solitare demo
"""

import random, os
from icedpygui import Window, Column, Container, MouseArea, Row, Stack, start_session, \
        add_button, add_column, add_mouse_area, add_space, add_stack, add_text, TextParam, \
        add_pick_list, add_image, update_widget, \
        add_container_style, move_widget, \
        Color, MousePointer, ContentFit, generate_id, \
        add_container \

class Solitaire:
    """Solitaire Game"""
    def __init__(self) -> None:
        # container ids
        self.wnd: int = 0,
        self.main_col = 0,

        self.stock_area_id: int = 0
        self.waste_area_id: int = 0
        self.foundation_area: list[int] = []
        self.status_id: int = 0
        self.tab_col_ids: list[int] = []

        self.tab_stack_ids = []
        self.tab_stack_ma_ids = []

        self.cwd = os.getcwd()
        self.path = self.cwd + "/python_examples/resources/cards/"
        self.card_width: float = 100.0
        self.card_height: float = 150.0
        self.shuffled_indexes: list = []
        self.white_border: int = 0

        self.cards: dict = []
        self.stock: list = []
        self.waste: list = []
        self.covers: list = []
        self.tableau: list = []
        self.foundation: list = []

        self.card = {}

        # controls on left
        self.rounds_id: int = 0

    def start_game(self):
        """Start Game"""
        self.create_styles()
        self.create_random_card_indexes()
        self.setup_gui()
        self.deal_cards()
        # self.add_remaining_to_stock()

    def setup_gui(self):
        """Gui"""
        with Window(title="Solitaire", size=(1000.0, 700.0), center=True) as self.wnd:

            # Make the basic layout orw with control and card area
            with Row(fill=True, spacing=2.0):
                # Control area
                with Container(
                    width=175.0,
                    height_fill=True,
                    style_id=self.white_border):

                    # make the left control column for buttons, etc
                    with Column(padding=[20.0], height_fill=True) as ctrl_col:
                        self.define_controls(ctrl_col)

                # main card area container
                with Container(fill=True, style_id=self.white_border):
                    with Column(fill=True) as self.main_col:
                        self.create_top_slots()
                        self.create_status_row()
                        self.create_tableau_row()

        start_session()

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
                               selected=str(self.cards_to_play))

        add_space(parent_id=ctrl_col, height=50.0)
        add_text(parent_id=ctrl_col,
            content="Instructions:\n \
                Cards are moved by selecting source and destination using mouse.\n \
                If a card fails to move it means the validation failed, wrong color or value.\n \
                To cancel a move, click any other place on the canvas")

    def create_top_slots(self):
        """Top Slot"""
        # add row for stock, waste, and foundation cards
        with Row(window_id=self.wnd,
            parent_id=self.main_col,
            height=self.card_height,
            spacing=10.0,
            padding=[5.0]) as stock_row:

            # add some beginning space
            add_space(width=20.0)

            # add the stock, waste, and foundation areas the row
            for i in range(6):
                with Container(
                    width=self.card_width,
                    height=self.card_height,
                    style_id=self.white_border): #id=stock

                    # stack is needed because we will need
                    # a mouse area and a blank card added
                    with Stack(fill=True):
                        if i == 0:
                            self.stock_area_id = add_mouse_area(
                                mouse_pointer=MousePointer.Grab,
                                on_press=self.stock_card_selected)
                        elif i == 1:
                            self.waste_area_id = add_mouse_area(
                                mouse_pointer=MousePointer.Grab,
                                on_press=self.waste_card_selected)
                            # need to add a little space after the waste
                            # Note the use of the row id
                            add_space(parent_id=stock_row, width=100)
                        else:
                            self.foundation_area.append(
                                add_mouse_area(
                                    mouse_pointer=MousePointer.Grab,
                                    on_press=self.fd_area_selected,
                                    user_data=i-2) # slots=0,1,2,3
                            )

    def create_status_row(self):
        """Status Row"""
        with Row(parent_id=self.main_col, height=50, align_center=True):
            add_space(width=30.0)
            self.status_id = add_text(content="Status: Selected None")

    def create_tableau_row(self):
        """Tableau row"""
        with Row(spacing=10.0):

            # Add a space at the beginning of the row
            add_space(width=20.0)

            # Add the 7 card tableau slots
            for _ in range(7):
                with Container(
                    style_id=self.white_border,
                    # width=self.card_width,
                    # height=self.card_height
                    ):
                    self.tab_col_ids.append(add_column())


    def create_styles(self):
        """Create Style for the game borders"""
        self.white_border = \
            add_container_style(
                border_color=Color.WHITE,
                border_width=2.0)

    def create_random_card_indexes(self):
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
            index = 1
            for (name, value) in ranks:
                d = {"card_id": None,
                    "suite": suite,
                    "color": color,
                    "name": name,
                    "value": value,
                    "file_index": index,
                    }
                self.shuffled_indexes.append(d)
                index += 1

        random.shuffle(self.shuffled_indexes)

    # def create_stock_waste_foundation(self):
    #     """Create the slots for the cards"""
    #     self.cards = {}
    #     # add row for stock, waste, and foundation cards
    #     with Row(window_id=self.wnd,
    #         parent_id=self.main_col,
    #         height=self.card_height,
    #         spacing=10.0,
    #         padding=[5.0]): # stock_row

    #         # add some beginning space
    #         add_space(width=20.0)

    #         # add the stock container to the row
    #         with Container(
    #             padding=[0.0],
    #             style_id=self.white_border): #id=stock

    #             # add the stack in
    #             with Stack(
    #                 width=self.card_width,
    #                 height=self.card_height): # "stack_stock_pile"

    #                 ma_id = add_mouse_area(
    #                     mouse_pointer=MousePointer.Grab,
    #                     on_press=self.card_selected)

    #         # create the blank card at the bottom so when clicked
    #         # we know it's blank
    #         stock = {}
    #         stock["card_id"] = ma_id
    #         stock["name"] = "stock blank"
    #         stock["value"] = 0
    #         stock["suite"] = None
    #         stock["color"] = None
    #         stock["reload"] = True

    #         # add the balnk card to the the stock
    #         self.stock.append(stock)

    #         # add the waste container to the row
    #         with Container(
    #             width=self.card_width,
    #             height=self.card_height,
    #             padding=[0.0],
    #             style_id=self.white_border): # waste

    #             # add the stack in
    #             # stack_waste_pile
    #             add_stack(
    #                 width=self.card_width,
    #                 height=self.card_height)

    #         # add a space between waste and foundation
    #         add_space(width=self.card_width)

    #         # Add the 4 foundation slots
    #         for i in range(0, 4):
    #             with Stack(
    #                        width=self.card_width,
    #                        height=self.card_height):

    #                 with MouseArea(
    #                     mouse_pointer=MousePointer.Grab,
    #                     on_press=self.card_selected) as ma_id:

    #                     fd = {}
    #                     fd["fd_id"] = ma_id
    #                     fd["name"] = f"foundation {i}"
    #                     fd["value"] = 0
    #                     fd["suite"] = None
    #                     fd["color"] = None
    #                     fd["fd_index"] = i
    #                     self.foundation.append(fd)

    #                     add_container(
    #                         width=self.card_width,
    #                         height=self.card_height,
    #                         padding=[0.0],
    #                         style_id=self.white_border)

    #         # add a container off screen to hide widget that become unused
    #         add_space(width=200.0)
    #         add_stack(container_id="hidden", show=False)

    #     # Add a space between the rows
    #     add_space(parent_id=self.main_col, height=20.0)

    def deal_cards(self):
        """Dealing the cards"""
        self.content = ""
        index = 0
        #  row, cols
        self.tableau = [[0 for _ in range(13)] for _ in range(7)]
        self.tab_col = [[0 for _ in range(13)] for _ in range(7)]
        for i in range(7):
            for j in range(13):

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

    def restart_play(self):
        """Restart"""
        print()

    def select_cards_to_play(self):
        """Number of card to play"""
        print()

    def cards_to_play(self):
        """_summary_
        """
        print()

    def stock_card_selected(self):
        """Stock Card Selected"""
        print()

    def waste_card_selected(self):
        """Waste Card Selected"""
        print()

    def fd_area_selected(self):
        """Fd Area Selected"""
        print()

    def card_selected(self):
        """Card Selected"""
        print()



game = Solitaire()
game.start_game()
