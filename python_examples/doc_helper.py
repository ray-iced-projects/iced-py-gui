#!/usr/bin/env python3
"""
Help to display the Doc string for a widget
"""

import icedpygui as ipg

widgets = [
    # containers
    "add_window", "add_container", "add_column", "add_float",
    "add_grid", "add_mouse_area","add_row",
    "add_opaque", "add_stack",
    # widgets
    "add_button", "add_button_style", "add_checkbox", "add_checkbox_style", "add_color_picker",
    "add_date_picker", "add_divider", "add_image", 
    "add_picklist", "add_progress_bar", "add_radio", 
    "add_rule", "add_scrollable", "add_selectable_text", 
    "add_separator", "add_slider", "add_space", "add_svg", 
    "add_table", "add_text_input", "add_text", "add_toggle",
    # utilities
    "update_widget", "add_session", 
]

def show_help(_btn_id, widget_name):
    """Shows the doc string if available"""
    fn = getattr(ipg, widget_name)
    ipg.update_widget(wid=txt_id,
                  param=ipg.TextParam.Content,
                  value=fn.__doc__ or f"No docs for {widget_name}")


btn_style = ipg.add_button_style(border_radius=[5.0])

with ipg.Window(title="Widget Help", center=True) as win:
    with ipg.Row():
        with ipg.Scrollable():
            with ipg.Column(spacing=10.0, width=250.0, padding=[20.0]):
                for name in widgets:
                    ipg.add_button(
                        label=name,
                        padding=[5.0],
                        style_id=btn_style,
                        # on_press=lambda _btn_id, name=name: show_help(name),
                        on_press=show_help,
                        user_data=name,
                    )

        with ipg.Scrollable():
            txt_id = ipg.add_text(content="Documentation will be displayed here")


ipg.start_session()
