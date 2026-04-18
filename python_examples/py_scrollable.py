"""
Scrollable demo

Allows scrolling when widgets exceed the container's size
"""

from icedpygui import Window, Container, Column, Row, start_session, \
        Scrollable, ScrollableParam, \
        add_button, add_text, update_widget

class DemoScrollable:
    """Scrollable demo"""
    def __init__(self) -> None:
        self.wnd_width: int = 400
        self.wnd_height: int = 600

        self.cb_text_v: int = 0
        self.cb_text_h1: int = 0
        self.cb_text_h2: int = 0
        self.cb_text_b: int = 0

        self.scroll_id_1: int = 0
        self.scroll_id_3: int = 0

        self.h_bar_width: float = 10.0
        self.v_bar_width: float = 10.0
        self.h_bar_margin: float = 0.0
        self.v_bar_margin: float = 0.0
        self.h_scroller_width: float = 10.0
        self.v_scroller_width: float = 10.0

    # start_session must be the last function called
    def create_gui(self):
        """Create the gui"""
        self.create_scroll_vertical()
        self.create_scroll_horizontal()
        # self.create_scroll_both()

        # Required to be the last widget sent to Iced,  If you start the program
        # and nothing happens, it might mean you forgot to add this command.
        start_session()

    # ***************Window 1-scrolling a column container vertically*******************
    # We will create 3 containers, a header, a scrollable, and an empty bottom container.
    def create_scroll_vertical(self):
        """Create vertical scrollable"""
        with Window(title="Scrollable - Vertical",
            size=(self.wnd_width, self.wnd_height),
                    position=(50, 25)):

            # The header is just a title of sorts.
            with Container(height=100.0, padding=[20]):

                add_text(content="Try Scrolling a Column Container.")

            # A container is put into a scrollable, so it needs to be added first.
            # The width and height should be used control the size of the scrollable,
            # depending if its horizontal or vertical.  In this case, we control
            # the height by setting it a value.  If you used height_fill=True in this case,
            # the container and text widget would be pushed out of the window but you
            # could still scroll because the height is less than the data height.
            with Scrollable(
                width_fill=True,
                height=150.0,
                on_scroll=self.on_scroll_vertical) as self.scroll_id_1:

                # A column is next added since the expectation is that you have a long list of
                # items that need to be scrolled.
                # NOTE that the column height should be shrink which is the default.
                # The scrollable size will control the size of the scrollable container.
                with Column(width_fill=True, align_center=True):

                    for _ in range(0, 25):
                        add_text(content="Scroll Me Up and Down! Scroll Me Up and Down!")

                # Container for the callback text
                with Container(fill=True):
                    add_text(content="Some data when scrolled")

                # Adding row for buttons to change things
                with Row():
                    add_button(
                        label="Press to Change Width",
                        on_press=self.change_width)

                    add_button(
                        label="Press to Change Height",
                        on_press=self.change_height)

    def change_width(self, _btn_id):
        """Changing width"""
        update_widget(
            self.scroll_id_1,
            ScrollableParam.Width,
            300.0)

    def change_height(self, _btn_id):
        """Changing height"""
        update_widget(
            self.scroll_id_1,
            ScrollableParam.Height,
            200.0)

    def on_scroll_vertical(self, _scroller_id, data: any):
        """Scroll Vertical"""
        print("Scrolling Vertical", data)

    # ************Window 2 scrolling a row container horizontally************

    def create_scroll_horizontal(self):
        """Horizontal Scrollable"""
        with Window(title="Scrollable - Horizontal",
                size=(200, self.wnd_height),
                position=(500, 25)):

            with Container(
                    width_fill=True,
                    height=200.0,
                    padding=[20]):

                add_text(content="Try Scrolling a Row Container.")

            # Unlike for the vertical scroller above, it's ok to use the full width
            # screen because nothing is in the way and the data is larger than the
            # window width.  However, we wanted to keep the scrollable height small
            # since there is only a single line of text.
            with Scrollable(
                        window_id=self.wnd_h,
                        id=self.scroll_h,
                        direction=ScrollableDirection.Horizontal,
                        width_fill=True,
                        height=50.0,
                        on_scroll=self.on_scroll_h,
                        user_data="Some scrolling Horizontal data")

                # NOTE: The row width and height should be left at default, no value.
                self.add_row(
                            window_id=self.wnd_h,
                            id=self.cont_h_middle,
                            parent_id=self.scroll_h)

                for i in range(0, 25):
                    self.add_text(
                                self.cont_h_middle,
                                content="Scroll Me left or Right!")

        # The final mostly empty container is added at the bottom
        self.add_column(
                    window_id=self.wnd_h,
                    id=self.cont_h_bottom,
                    parent_id=self.wnd_h)

        self.cb_text_h1 = self.add_text(
                                    parent_id=self.cont_h_bottom,
                                    content=f"Some data when scrolled")
        self.cb_text_h2 = self.add_text(
                                    parent_id=self.cont_h_bottom,
                                    content=f"User data when scrolled")

    # The data in this case in a dictionary, absolute, relative, and absolute reversed
    # {'abs_x': 0.0, 'abs_y': 0.0, 'rel_x': 0.0, 'rev_x': 0.0, 'rev_y': 0.0, 'rel_y': 0.0}
    def on_scroll_v(self, id, data):
        text = "\n" + 'abs_x = ' + str(data.get('abs_x'))
        text += "\n" + 'abs_y = ' + str(data.get('abs_y'))
        text += "\n" + 'rel_x = ' + str(data.get('rel_x'))
        text += "\n" + 'rel_y = ' + str(data.get('rel_y'))
        text += "\n" + 'rev_x = ' + str(data.get('rev_x'))
        text += "\n" + 'rev_y = ' + str(data.get('rev_y'))

        self.update_widget(
            wid=self.cb_text_v,
            param=TextParam.Content,
            value=f"scrollable id = {id}\n{text}")

    def on_scroll_h(self, id, data, user_data: any):
        text = "\n" + 'abs_x = ' + str(data.get('abs_x'))
        text += "\n" + 'abs_y = ' + str(data.get('abs_y'))
        text += "\n" + 'rel_x = ' + str(data.get('rel_x'))
        text += "\n" + 'rel_y = ' + str(data.get('rel_y'))
        text += "\n" + 'rev_x = ' + str(data.get('rev_x'))
        text += "\n" + 'rev_y = ' + str(data.get('rev_y'))

        self.update_widget(
            wid=self.cb_text_h1,
            param=TextParam.Content,
            value=f"scrollable id = {id}\n{text}")

        self.update_widget(
            wid=self.cb_text_h2,
            param=TextParam.Content,
            value=f"user data = {user_data}")

#     # ***************Window 3-scrolling both directions with other property setting****

#     def create_scroll_both(self):
#         # Add the 3rd window
#         self.add_window(
#                     id=self.wnd_b,
#                     title="Scrollable - Both",
#     size=(self.wnd_width + 100, self.wnd_height),
#                     position=(760, 25))

#         # The container is added to center the contents below.
#         self.add_container(
#                     window_id=self.wnd_b,
#                     id=self.cont_b,
#                     width_fill=True,
#                     height_fill=True)

#         # Add a column to hold all the widgets
#         self.add_column(
#                     window_id=self.wnd_b,
#                     id="col",
#                     parent_id=self.cont_b,
#                     spacing=10,
#                     align=Alignment.Center)

#         # Display some info
#         self.add_text(
#                     parent_id="col",
#                     content="You may have to press buttons many times to see the changes")

#         # The scrollable size controls the viewport for the column container.
#         self.scroll_id_3 = self.add_scrollable(
#                                         window_id=self.wnd_b,
#                                         id=self.scroll_b,
#                                         parent_id="col",
#                                         width=250,
#                                         height=100.0,
#                                         direction=ScrollableDirection.Both)

#         # NOTE:  The column width and height should default to shrink, no value.
#         self.add_column(
#                     window_id=self.wnd_b,
#                     id=self.col_b,
#                     parent_id=self.scroll_b,
#                     align=Alignment.Center)

#         for _ in range(0, 25):
#             self.add_text(
#                         parent_id=self.col_b,
#                         content="Scroll Me Up, Down, left, or Right!"
#                                 "Scroll Me Up, Down, left, or Right!")

#         # Add row to hold the buttons.
#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_1",
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_1",
#                     label="Press to + H Bar Width",
#                     on_press=self.inc_dec_h_bar_width,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_1",
#                     label="Press to - H Bar Width",
#                     on_press=self.inc_dec_h_bar_width,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_2",
#                     padding=[5],
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_2",
#                     label="Press to + V Bar Width",
#                     on_press=self.inc_dec_v_bar_width,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_2",
#                     label="Press to - V Bar Width",
#                     on_press=self.inc_dec_v_bar_width,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_3",
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_3",
#                     label="Press to + H Bar Margin",
#                     on_press=self.inc_dec_h_bar_margin,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_3",
#                     label="Press to - H Bar Margin",
#                     on_press=self.inc_dec_h_bar_margin,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_4",
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_4",
#                     label="Press to + V Bar Margin",
#                     on_press=self.inc_dec_v_bar_margin,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_4",
#                     label="Press to - V Bar Margin",
#                     on_press=self.inc_dec_v_bar_margin,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_5",
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_5",
#                     label="Press to + H Scroller Width",
#                     on_press=self.inc_dec_h_scroller_width,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_5",
#                     label="Press to - H Scroller Width",
#                     on_press=self.inc_dec_h_scroller_width,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_6",
#                     parent_id="col")

#         self.add_button(
#                     parent_id="row_6",
#                     label="Press to Change V Scroller Width",
#                     on_press=self.inc_dec_v_scroller_width,
#                     padding=[5],
#                     user_data=1)

#         self.add_button(
#                     parent_id="row_6",
#                     label="Press to - V Scroller Width",
#                     on_press=self.inc_dec_v_scroller_width,
#                     padding=[5],
#                     user_data=-1)

#         self.add_row(
#                     window_id=self.wnd_b,
#                     id="row_7",
#                     parent_id="col")

#     def inc_dec_h_bar_width(self, btn_id, inc_dec):
#         self.h_bar_width += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.HBarWidth,
#             value=self.h_bar_width)

#     def inc_dec_v_bar_width(self, btn_id, inc_dec):
#         self.v_bar_width += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.VBarWidth,
#             value=self.v_bar_width)

#     def inc_dec_h_bar_margin(self, btn_id, inc_dec):
#         self.h_bar_margin += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.HBarMargin,
#             value=self.h_bar_margin)

#     def inc_dec_v_bar_margin(self, btn_id, inc_dec):
#         self.v_bar_margin += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.VBarMargin,
#             value=self.v_bar_margin)

#     def inc_dec_h_scroller_width(self, btn_id: int, inc_dec: float):
#         self.h_scroller_width += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.HScrollerWidth,
#             value=self.h_scroller_width
#         )

#     def inc_dec_v_scroller_width(self, btn_id: int, inc_dec: float):
#         self.v_scroller_width += inc_dec
#         self.update_widget(
#             wid=self.scroll_id_3,
#             param=ScrollableParam.VScrollerWidth,
#             value=self.v_scroller_width
#         )


# instantiate the class
ds = DemoScrollable()

ds.create_gui()
