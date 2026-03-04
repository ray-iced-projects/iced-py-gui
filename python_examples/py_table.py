import random

from imports import *


def table_column_resize(tbl_id: int, index: int, values: list[float]):
    print(index, values)


def sum_of_column() -> str:
    return str(sum(row[5] for row in body))    


def sort_list(pkl_id: int, selected: str):
    global headers, body, table_id
    # Sort by the "str" column (index 1)
    match selected:
        case "Sort(a-z)":
            body.sort(key=lambda r: r[1]) 
        case "Sort(z-a)":
            body.sort(key=lambda r: r[1], reverse=True)
            
    # Use update_widget with the new Body param
    update_widget(table_id, IpgTableParam.Body, body)


def math_op(pkl_id: int, selected: str, index: int):
    global body, table_id, footer_ids
    col_values = [row[5] for row in body]
    match selected:
        case "Add":
            value = f"Sum={sum(col_values)}"
        case "Count":
            value = f"Count={len(col_values)}"
        case "Mean":
            value = f"Mean={sum(col_values)/len(col_values):.2f}"
    update_widget(footer_ids[index], IpgTextParam.Content, value)


# Filtering with plain lists
def filtering(pkl_id: int, selected: str):
    global body, original_body, table_id
    match selected:
        case "True": 
            value = True
        case "False":
            value = False
        case "None":
            update_widget(table_id, IpgTableParam.Body, original_body)
            for wid in button_ids:
                update_widget(wid, IpgButtonParam.Show, True)
            for id_tf in checkbox_ids:
                update_widget(id_tf[0], IpgCheckboxParam.Show, True)
            return
    
    # Filter body rows where the checks column (index 3) matches value
    filtered_body = [row for row, chk in zip(original_body, checks) if chk == value]
    
    # Show/hide buttons and checkboxes
    for i, wid in enumerate(button_ids):
        if checks[i] == value:
            update_widget(wid, IpgButtonParam.Show, True)
        else:
            update_widget(wid, IpgButtonParam.Show, False)
    
    for id_tf in checkbox_ids:
        if id_tf[1] == value:
            update_widget(id_tf[0], IpgCheckboxParam.Show, True)
        else:
            update_widget(id_tf[0], IpgCheckboxParam.Show, False)
               
    update_widget(table_id, IpgTableParam.Body, filtered_body)
   


total_id = 0
selected = "None"
list_ids = []
row_ids = []
filered_ids = []
column_widths = [100.0] * 6
button_ids = [ generate_id() for _ in range(11) ]
checkbox_ids = [ [generate_id(), random.choice([True, False])] for _ in range(11) ]
checks = [ tup[1] for tup in checkbox_ids ]

# Headers for the table columns (including control columns)
headers = ["Edit", "str", "one", "Checks", "two", "three"]

# Body data: each row is a list of floats.
# Control columns (Edit, Checks) use 0.0 as placeholder since
# actual widgets are added separately.
body = [
    [0.0, 0.0, 1.0, 0.0, 2.0, 3.0],
    [0.0, 0.0, 2.0, 0.0, 4.0, 6.0],
    [0.0, 0.0, 3.0, 0.0, 6.0, 9.0],
    [0.0, 0.0, 4.0, 0.0, 8.0, 12.0],
    [0.0, 0.0, 5.0, 0.0, 10.0, 15.0],
    [0.0, 0.0, 6.0, 0.0, 12.0, 18.0],
    [0.0, 0.0, 7.0, 0.0, 14.0, 21.0],
    [0.0, 0.0, 8.0, 0.0, 16.0, 24.0],
    [0.0, 0.0, 9.0, 0.0, 18.0, 27.0],
    [0.0, 0.0, 10.0, 0.0, 20.0, 30.0],
    [0.0, 0.0, 11.0, 0.0, 22.0, 33.0],
]
original_body = [row[:] for row in body]  # keep a copy for filtering
footers = [""] * 6
df_width = len(headers)
df_length = len(body)

# Some styling for the widgets
btn_style = add_button_style(border_radius=[10.0])


# Add the window
add_window(
        id="main", 
        title="Table Demo",
        size=(1000, 400),
        center=True,
        theme=IpgWindowTheme.TokyoNightStorm,
        debug=False)

# Add the container for centering the table
add_container(
        window_id="main", 
        id="cont",
        width_fill=True, 
        height_fill=True,
        center=True,)

width = sum(column_widths)

# The table is added.
table_id = add_table(
        window_id="main",
        table_id="table",
        headers=headers,
        body=body,
        footers=footers,
        parent_id="cont",
        column_widths=column_widths,
        height=150.0,
        # above required
        # width=300.0, # see the scroller when the table is smaller than the column widths, it defaults to sum of columns
        on_column_resize=table_column_resize, # may need in some cases where resizing causes alignment issues or other cases.
        # min_column_width=50.0, # uncomment to see effect
        custom_header_rows=1, # the number of additional header rows, default=0
        custom_footer_rows=1, # the number of footer rows, default=0
        control_columns=[0, 3], # list for the indexes of the control columns
        )


# Once the table is added, you can add the other items
# IMPORTANT:
# You must add these in order, if you have indicated
# in them in the table parameters.
# 1. control_columns
# 2. custom_header_row
# 3. custom_footer_row


# Important:
# When you add the control columns, The you must keep the rows
# together.  Below, the button was added then the toggler.
# Therefore when the table code iterates through the rows,
# each element is pulled out as needed from one vector.
# If you add all the buttons then the togglers, you'll have buttons
# in your rows until they run out then togglers.
# If you want mixed columns then you could use the index to
# determine whether it's a button or something else,
# i.e. if index == 1 add_button else add checkbox  

# For the widgets in the control columns,
# the lengths must match the dataframe or you'll
# get an error.  
# 
# In the Table code, each widget is
# placed into a container and the width of the column
# is used as the container width along with centering the
# widget.  
# 
# It's best to use the default shrink to keep things centered.  
# If you set the width to fill, some widgets may not align there labels
# correctly because it doesn't know the size.  If you use a set width, 
# the alignment works but if you resize the column, you'll need to resize all of 
# the widths too, not a big effort in the callback.  If you want a wider
# widget with a smaller label, try using just the padding on each side.  The button
# in the this table uses the padding as an example of making the button wider
# but keeping the default shrink. 
# 
# Some cases may require resizing the widget.

# You can place any widget or a combination of widgets in the table,
# just as long as you put only one widget or one parent widget in the column cell.

# As an example of having the widget interact with the database, note the button
# uses the user_data for the index, so when pressed, the callback has the row index.
# In the case of the checkbox, we are going to keep track of the if checked values
# update the checked column of the df and use a picklist to filter the df.
for i in range(df_length):
    add_button(
        parent_id="table",
        label="Edit",
        padding=[0.0, 20.0, 0.0, 20.0],
        style_id=btn_style,
        user_data=i,
        gen_id=button_ids[i])
    
    add_checkbox(
        parent_id="table",
        label="Check Me",
        is_checked=checks[i],
        user_data=i,
        gen_id=checkbox_ids[i][0])


# add the custom header row.  If you have another row,
# just repeat the process below making sure that the row
# count matches the header count.
header = [""] * df_width
for i in range(df_width):
    if i == 1:
        add_pick_list(
            parent_id="table",
            options=["Sort(a-z)", "Sort(z-a)"],
            placeholder="Sort",
            on_select=sort_list)
    elif i == 3:
        add_pick_list(
            parent_id="table",
            options=["True", "False", "None"],
            placeholder="Filter",
            on_select=filtering) 
    elif i == 5:
        add_pick_list(
            parent_id="table",
            options=["Add", "Count", "Mean"],
            placeholder="Math",
            on_select=math_op,
            user_data=5) # the footer index
    else:    
        add_text(
            parent_id="table",
            content=header[i],
            size=16.0)


# The custom footer is basically the same as the header.
# The footer ids are needed for the table footer update
# which is just a text update.
footer_ids = []
column_three_sum = sum(row[5] for row in body)
footer = [""] * 6
footer[5] = f"Sum={column_three_sum}"
for i in range(df_width):
    footer_ids.append(add_text(
                    parent_id="table",
                    content=footer[i],
                    size=14.0))




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
