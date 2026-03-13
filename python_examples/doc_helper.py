import icedpygui as ipg

widgets = [
    "add_window", "add_container", "add_column", "add_row",
    "add_button", "add_checkbox",
]

def show_help(widget_name):
    fn = getattr(ipg, widget_name)
    print(fn.__doc__ or f"No docs for {widget_name}")

with ipg.Window(title="Widget Help") as win:
    with ipg.Column(spacing=10.0):
        for name in widgets:
            ipg.add_button(
                label=name,
                on_press=lambda _btn_id, name=name: show_help(name),
            )

ipg.start_session()
