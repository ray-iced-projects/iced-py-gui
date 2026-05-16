# from imports import *

# def set_cp_label(rd_id: int, selected: tuple[int, str]):
#     global radio_selected
#     global radio_ids
#     radio_selected = selected

#     if "Header" in selected[1]:
#         match selected[0]:
#             case 0:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Header Bkg")
#             case 1:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Header Border Color")
#         # unselect other radios if selected
#         update_widget(wid=radio_ids[1], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[2], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[3], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[4], param=IpgRadioParam.SelectedIndex, value=None)

#     if "Body" in selected[1]:
#         match selected[0]:
#             case 0:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Body Bkg")

#             case 1:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Body Border Color")
#         # unselect other radios if selected
#         update_widget(wid=radio_ids[0], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[2], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[3], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[4], param=IpgRadioParam.SelectedIndex, value=None)

#     if "Footer" in selected[1]:
#         match selected[0]:
#             case 0:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Footer Bkg")

#             case 1:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Footer Border Color")
#         # unselect other radios if selected
#         update_widget(wid=radio_ids[0], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[1], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[3], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[4], param=IpgRadioParam.SelectedIndex, value=None)

#     if "Divider" in selected[1]:
#         match selected[0]:
#             case 0:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Divider Bkg")

#             case 1:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Divider Hover Color")
#         # unselect other radios if selected
#         update_widget(wid=radio_ids[0], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[1], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[2], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[4], param=IpgRadioParam.SelectedIndex, value=None)

#     if "Scroller" in selected[1]:
#         match selected[0]:
#             case 0:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Scroller Bkg")

#             case 1:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Scroller Hover Color")
#             case 2:
#                 update_widget(
#                     wid=cp_id,
#                     param=IpgColorPickerParam.Label,
#                     value="Set Scroller Rail Color")
#         # unselect other radios if selected
#         update_widget(wid=radio_ids[0], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[1], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[2], param=IpgRadioParam.SelectedIndex, value=None)
#         update_widget(wid=radio_ids[3], param=IpgRadioParam.SelectedIndex, value=None)


# def set_color(cp_id: int, color: list):
#     global radio_selected

#     if "Header" in radio_selected[1]:
#         match radio_selected[0]:
#             case 0:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.HeaderBackgroundRgbaColor,
#                     value=color)

#             case 1:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.HeaderBorderRgbaColor,
#                     value=color)

#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.HeaderBorderWidth,
#                     value=2.0)

#     if "Body" in radio_selected[1]:
#         match radio_selected[0]:
#             case 0:
#                 # Change the alpha of the color which makes it more transparent.
#                 color[3] = 0.3
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.BodyBackgroundRgbaColor,
#                     value=color)

#                 # Since table highlight defults to True, we'll
#                 # make the highlight color too.  You could skip this to
#                 # have a transparent background on alternate rows.
#                 # NOTE: we copy the color value because if you change
#                 # the color value, then the color for both will be changed
#                 # since the data does not go back to rust untill the callback
#                 # is finished.
#                 c = color.copy()
#                 c[3] = 0.1
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.BodyRowHighlightRgba,
#                     value=c)

#             case 1:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.BodyBorderRgbaColor,
#                     value=color)

#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.BodyBorderWidth,
#                     value=2.0)

#     if "Footer" in radio_selected[1]:
#         match radio_selected[0]:
#             case 0:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.FooterBackgroundRgbaColor,
#                     value=color)

#             case 1:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.FooterBorderRgbaColor,
#                     value=color)

#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.FooterBorderWidth,
#                     value=2.0)

#     if "Divider" in radio_selected[1]:
#         match radio_selected[0]:
#             case 0:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.DividerBackgroundRgbaColor,
#                     value=color)

#             case 1:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.DividerHoverRgbaColor,
#                     value=color)

#     if "Scroller" in radio_selected[1]:
#         match radio_selected[0]:
#             case 0:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.ScrollerBackgroundRgbaColor,
#                     value=color)

#             case 1:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.ScrollerHoverRgbaColor,
#                     value=color)

#             case 2:
#                 update_widget(
#                     wid=tbl_style_id,
#                     param=IpgTableStyleParam.ScrollerRailRgbaColor,
#                     value=color)


# radio_selected = (0, "")
# column_widths = [100.0] * 4
# width = sum(column_widths)

# headers = ["str", "one", "two", "three"]
# body = [
#     [0.0, 1.0, 2.0, 3.0],
#     [0.0, 2.0, 4.0, 6.0],
#     [0.0, 3.0, 6.0, 9.0],
#     [0.0, 4.0, 8.0, 12.0],
#     [0.0, 5.0, 10.0, 15.0],
#     [0.0, 6.0, 12.0, 18.0],
#     [0.0, 7.0, 14.0, 21.0],
#     [0.0, 8.0, 16.0, 24.0],
#     [0.0, 9.0, 18.0, 27.0],
#     [0.0, 10.0, 20.0, 30.0],
#     [0.0, 11.0, 22.0, 33.0],
# ]
# footers = ["", "", "", ""]


# # adding the default style by suppling no arguments
# tbl_style_id = add_table_style()


# # Add the window
# add_window(
#         id="main",
#         title="Table Demo",
#     size=(700, 600),
#         pos_centered=True,
#         theme=IpgWindowTheme.TokyoNightStorm,
#         debug=False)

# # Add the container for centering the table
# add_container(
#         window_id="main",
#         id="cont",
#         width_fill=True,
#         height_fill=True,
#         centered=True,)

# add_column(
#     window_id="main",
#     parent_id="cont",
#     id="col",
# )

# # Needed in callback
# radio_ids = []

# radio_ids.append(add_radio(
#     parent_id="col",
#     labels=["Set Header Bkg", "Set Header Border Color"],
#     on_select=set_cp_label,
#     direction=IpgRadioDirection.Horizontal))

# radio_ids.append(add_radio(
#     parent_id="col",
#     labels=["Set Body Bkg", "Set Body Border Color"],
#     on_select=set_cp_label,
#     direction=IpgRadioDirection.Horizontal))

# radio_ids.append(add_radio(
#     parent_id="col",
#     labels=["Set Footer Bkg", "Set Footer Border Color"],
#     on_select=set_cp_label,
#     direction=IpgRadioDirection.Horizontal))

# radio_ids.append(add_radio(
#     parent_id="col",
#     labels=["Set Divider Bkg", "Set Divider Hover Color"],
#     on_select=set_cp_label,
#     direction=IpgRadioDirection.Horizontal))

# radio_ids.append(add_radio(
#     parent_id="col",
#     labels=["Set Scroller Bkg", "Set Scroller Hover Color", "Set Scroller Rail Color"],
#     on_select=set_cp_label,
#     direction=IpgRadioDirection.Horizontal))

# cp_id = add_color_picker(
#     parent_id="col",
#     label="No Selection Made",
#     on_submit=set_color)


# # The table is added.
# table_id = add_table(
#         window_id="main",
#         table_id="table",
#         headers=headers,
#         body=body,
#         footers=footers,
#         parent_id="col",
#         column_widths=column_widths,
#         height=150.0,
#         # width=200.0, # uncomment to show header and footer scrollers
#         custom_footer_rows=1,
#         style_id=tbl_style_id
#         )

# footer = ["This", "is", "a", "footer"]
# for i in range(len(footer)):
#     add_text(
#             parent_id="table",
#             content=footer[i],
#             size=14.0)

# # Required to be the last widget sent to Iced,  If you start the program
# # and nothing happens, it might mean you forgot to add this command.
# start_session()
