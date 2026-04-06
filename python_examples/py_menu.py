from icedpygui import Window, Container, start_session, \
    Menu, MenuBarItem, add_menu_style, \
    IpgMenuParam, IpgMenuStyleParam, Color, \
    add_button, add_text, IpgButtonStyleStd, \
    add_separator, IpgSeparatorType


def on_press(id, name):
    print(f"selected: {name}")


# Add a window
with Window(title="Menu", center=True):
    with Container(padding=[20.0]):
        with Menu(bar_spacing=10.0, item_spacings=[5.0], item_widths=[75.0], item_paddings=[5.0]):
            # Each MenuBarItem groups a bar widget (first child) with its dropdown items
            with MenuBarItem():
                add_button(label="File", menu=True)          # No callback for the bar widget
                add_button(label="New", menu=True, on_press=on_press, user_data="New")          # dropdown item
                add_button(label="Open", menu=True, on_press=on_press, user_data="Open")        # dropdown item
                add_button(label="Save", menu=True, on_press=on_press, user_data="Save")        # dropdown item

            with MenuBarItem():
                add_button(label="Edit", menu=True)          # No callback for the bar widget
                add_button(label="Cut", menu=True, on_press=on_press, user_data="Cut")           # dropdown item
                add_button(label="Copy", menu=True, on_press=on_press, user_data="Copy")         # dropdown item
                add_separator(separator_type=IpgSeparatorType.Dot, dot_radius=3.0, dot_count=10, spacing=3.0,)
                add_button(label="Paste", menu=True, on_press=on_press, user_data="Paste")       # dropdown item

            with MenuBarItem():
                add_button(label="Help", menu=True)          # No callback for the bar widget
                add_button(label="About", menu=True, on_press=on_press, user_data="About")        # dropdown item



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
