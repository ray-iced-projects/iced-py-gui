from imports import *

# NOTE: To reduce the number of items that need to be changed,
# make the changes to the row's height instead of  each container
# in the row by using a height_fill on the containers.  This only then
# leaves having to do each container in the rows when the column
# is changed where the indexing of the 2d list is easier.

def divider_row_change(div_id: int, index: int, value: float):
    # Get the difference to be added to the below row
    diff = rows[index] - value
    
    # Update the local data
    rows[index] = value
    if index < len(rows)-1:
        rows[index+1] += diff
    
    # Update the row above the divider
    update_widget(
        wid=row_ids[index], 
        param=IpgRowParam.Height, 
        value=value)

    # Update the row below the divider
    if index < len(rows)-1:
        update_widget(
            wid=row_ids[index+1],
            param=IpgRowParam.Height,
            value=rows[index+1])
            
    # Update the divider
    update_widget(
        wid=div_id,
        param=IpgDividerParam.Heights,
        value=rows)
    
    # Update the height of the column divider
    update_widget(
        wid=col_div,
        param=IpgDividerParam.HandleHeight,
        value=sum(rows)
    )
    
 
def divider_col_change(div_id: int, index: int, value: float):
    # get the difference to be added to the right side of the divider
    diff = columns[index] - value
    
    # update the local values for storing the data
    columns[index] = value
    if index < len(columns)-1:
        columns[index+1] += diff
    
    for i in range(0, len(rows)):
        # Update all the containers on the left of the divider
        update_widget(
            wid=container_ids[i][index], 
            param=IpgContainerParam.Width, 
            value=value)

        # Update all the containers on the right of the divider
        if index < len(columns)-1:
            update_widget(
                wid=container_ids[i][index+1],
                param=IpgContainerParam.Width,
                value=columns[index+1])

    # Update the column divider
    update_widget(
        wid=div_id,
        param=IpgDividerParam.Widths,
        value=columns)
    
    # Update the width of the row divider
    update_widget(
        wid=row_div,
        param=IpgDividerParam.HandleWidth,
        value=sum(columns)
    )
    
    # Update the stack width
    # The stack width had to be adjusted unlike 
    # the case above.  Looking at the iced code
    # doesn't reveal any issues with the stack
    # but when it comes to calculating widths and heights
    # sometimes different combinations of the 
    # containing widgets cause issues where a width
    # or height needs to be set.
    update_widget(
        wid=stack_id,
        param=IpgStackParam.Width,
        value=sum(columns))

# It can be easy visualize to use row/column vs widths/heights
rows = [100.0, 100.0, 100.0]
columns = [150.0, 148.0]
row_ids = []
container_ids = []
row_handle_width = sum(columns)  
row_handle_height = 4.0

col_handle_width = 4.0
col_handle_height = sum(rows)

     
cont_style_id = add_container_style(
                        border_color=IpgColor.WHITE,
                        border_width=1.0)

divider_style_id = add_divider_style(
                        background_transparent=True)


# Add a window first
add_window(
        id="main", 
        title="CheckBox Demo",
    size=(600, 600),  
        pos_centered=True,
        # debug=True
        )

# Add a container to center the widgets in the middle
add_container(
        window_id="main", 
        id="main_cont", 
        width_fill=True,
        height_fill=True,
        centered=False,
        padding=[100, 0, 0, 100])

# add a column to hold the text and the stack
add_column(
        window_id="main",
        id="main_col",
        parent_id="main_cont",
        spacing=30)

content = "Pace the cursor over the highlighted divider and drag"

add_text(
        parent_id="main_col",
        content=content)

# make the stack to lay the dividers over the containers
stack_id = add_stack(
        window_id="main",
        id="stack",
        parent_id="main_col")


# make a column to hold the two columns
# this is added to stack
add_column(
        window_id="main",
        parent_id="stack",
        id="col",
        spacing=0,
        padding=[0],
        width=row_handle_width)

for i, height in enumerate(rows):
    row_ids.append(add_row(
        window_id="main",
        id=f"row{i}",
        parent_id="col",
        height=height,
        spacing=0))

    cont_ids = []
    for j, width in enumerate(columns):
        cont_ids.append(add_container(
                window_id="main",
                id=f"cont{i} {j}",
                parent_id=f"row{i}",
                width=width,
                height_fill=True,
                style_id=cont_style_id))
        
        add_text(
            parent_id=f"cont{i} {j}",
            content=f"Some Text")
        
    container_ids.append(cont_ids)
    
 
# Make the vertical divider (rows)
row_div = add_divider_vertical(
            parent_id="stack",
            heights=rows,
            handle_width=row_handle_width,
            handle_height=row_handle_height,
            on_change=divider_row_change,
            # use the style to see just the outline and not the divider
            # style_id=divider_style_id
            )

#Make the horizontal divider (columns)
col_div = add_divider_horizontal(
            parent_id="stack",
            widths=columns,
            handle_width=col_handle_width,
            handle_height=col_handle_height,
            on_change=divider_col_change,
            # use the style to see just the outline and not the divider
            # style_id=divider_style_id
            )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
